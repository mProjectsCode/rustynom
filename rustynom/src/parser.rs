use crate::{
    ParseResult, ParsingContext, ParsingPosition,
    atomic_parsers::{CustomParser, EofParser},
    combinator_parsers::{AndParser2, AndParser3, OrParser2, SameOrParser2, Variant2},
    transformation_parsers::{ManyParser, MapParser},
};

pub trait ParserCombinator<TParser: Parser<T>, T: Clone> {
    fn and<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<(T, U)>;
    fn or<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<Variant2<T, U>>;
    fn or_same<UParser: Parser<T>>(self, other: UParser) -> impl Parser<T>;
    fn map<TFn: (Fn(T) -> U) + Clone, U: Clone>(self, f: TFn) -> impl Parser<U>;
    fn skip<UParser: Parser<()> + 'static>(self, other: UParser) -> impl Parser<T>;
    fn then<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        other: UParser,
    ) -> impl Parser<U>;
    fn then_eof(self) -> impl Parser<T>;
    fn many(self) -> impl Parser<Vec<T>>;
    fn many_non_empty(self) -> impl Parser<Vec<T>>;
    fn separated_by_non_empty<UParser: Parser<()> + 'static>(
        self,
        separator: UParser,
    ) -> impl Parser<Vec<T>>;
    fn optional(self) -> impl Parser<Option<T>>;
    fn trim<UParser: Parser<()> + 'static>(self, other: UParser) -> impl Parser<T>;
    fn surround<LParser: Parser<()> + 'static, RParser: Parser<()> + 'static>(
        self,
        l: LParser,
        r: RParser,
    ) -> impl Parser<T>;
    fn describe(self, description: &str) -> impl Parser<T>;
    fn box_describe(self, description: &str) -> impl Parser<T>;

    fn parse_string(&self, string: String) -> ParseResult<T>;
    fn parse_str(&self, string: &str) -> ParseResult<T>;
    fn parse_chars(&self, chars: &[char]) -> ParseResult<T>;
}

impl<TParser: Parser<T> + 'static, T: Clone + 'static> ParserCombinator<TParser, T> for TParser {
    fn and<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<(T, U)> {
        AndParser2::new(self, other)
    }

    fn or<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<Variant2<T, U>> {
        OrParser2::new(self, other)
    }

    fn or_same<UParser: Parser<T>>(self, other: UParser) -> impl Parser<T> {
        SameOrParser2::new(self, other)
    }

    fn map<TFn: (Fn(T) -> U) + Clone, U: Clone>(self, f: TFn) -> impl Parser<U> {
        MapParser::new(self, f)
    }

    fn skip<UParser: Parser<()> + 'static>(self, other: UParser) -> impl Parser<T> {
        AndParser2::new(self, other).map(|(a, _)| a)
    }

    fn then<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        other: UParser,
    ) -> impl Parser<U> {
        AndParser2::new(self, other).map(|(_, b)| b)
    }

    fn then_eof(self) -> impl Parser<T> {
        self.skip(EofParser::new())
    }

    fn many(self) -> impl Parser<Vec<T>> {
        ManyParser::new(self)
    }

    fn many_non_empty(self) -> impl Parser<Vec<T>> {
        self.clone().and(self.many()).map(|(a, mut b)| {
            b.insert(0, a);
            b
        })
    }

    fn separated_by_non_empty<UParser: Parser<()> + 'static>(
        self,
        separator: UParser,
    ) -> impl Parser<Vec<T>> {
        // self.clone()
        //     .skip(separator)
        //     .many()
        //     .and(self)
        //     .map(|(mut a, b)| {
        //         a.push(b);
        //         a
        //     })

        CustomParser::new(move |context| {
            let mut result = Vec::new();

            let first = self.parse(context);

            if first.is_failure() {
                return ParseResult::Failure(first.unwrap_failure());
            }

            result.push(first.unwrap_success());

            let mut cloned_position = context.position.clone();
            loop {
                let separator_result = separator.parse(context);
                if separator_result.is_failure() {
                    // if the separator fails, the sequence is over and we return the result
                    // we don't need to move the position back because the separator failed
                    return ParseResult::Success(result);
                }

                let self_result = self.parse(context);
                if self_result.is_failure() {
                    // if the self parser fails, we return the result
                    // we need to move the position back to the last successful position
                    context.advance_to(cloned_position);
                    return ParseResult::Success(result);
                }

                result.push(self_result.unwrap_success());
                cloned_position = context.position.clone();
            }
        })
    }

    fn optional(self) -> impl Parser<Option<T>> {
        CustomParser::new(move |context| match self.parse(context) {
            ParseResult::Success(a) => ParseResult::Success(Some(a)),
            ParseResult::Failure(_) => ParseResult::Success(None),
        })
    }

    fn trim<UParser: Parser<()> + 'static>(self, other: UParser) -> impl Parser<T> {
        self.surround(other.clone(), other)
    }

    fn surround<LParser: Parser<()> + 'static, RParser: Parser<()> + 'static>(
        self,
        l: LParser,
        r: RParser,
    ) -> impl Parser<T> {
        AndParser3::new(l, self, r).map(|(_, a, _)| a)
    }

    fn describe(self, description: &str) -> impl Parser<T> {
        let d = description.to_string();
        CustomParser::new(move |context| match self.parse(context) {
            ParseResult::Success(a) => ParseResult::Success(a),
            ParseResult::Failure(mut f) => {
                f.expected = vec![d.clone()];
                ParseResult::Failure(f)
            }
        })
    }

    fn box_describe(self, description: &str) -> impl Parser<T> {
        let d = description.to_string();
        CustomParser::new(move |context| match self.parse(context) {
            ParseResult::Success(a) => ParseResult::Success(a),
            ParseResult::Failure(mut f) => {
                f.expected = vec![format!("[{}] as part of {}", f.expected.join(", "), d)];
                ParseResult::Failure(f)
            }
        })
    }

    fn parse_string(&self, string: String) -> ParseResult<T> {
        let chars = string.chars().collect::<Vec<char>>();
        let mut context = ParsingContext::new(&chars, ParsingPosition::default());
        self.parse(&mut context)
    }

    fn parse_str(&self, string: &str) -> ParseResult<T> {
        let chars = string.chars().collect::<Vec<char>>();
        let mut context = ParsingContext::new(&chars, ParsingPosition::default());
        self.parse(&mut context)
    }

    fn parse_chars(&self, chars: &[char]) -> ParseResult<T> {
        let mut context = ParsingContext::new(chars, ParsingPosition::default());
        self.parse(&mut context)
    }
}

pub trait RawParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T>;
}

pub trait Parser<T>: RawParser<T> + Clone {}

impl<T, P: RawParser<T> + Clone> Parser<T> for P {}
