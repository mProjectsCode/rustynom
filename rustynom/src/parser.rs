use crate::{
    atomic_parsers::EofParser,
    combinator_parsers::{AndParser2, OrParser2, Variant2},
    transformation_parsers::{ManyParser, MapParser},
    ParseResult, ParsingContext, ParsingPosition,
};

// TODO: This can probably done without the dynamic dispatch, but it's going to be a lot of explicit types
pub trait ParserCombinator<TParser: Parser<T>, T: Clone> {
    fn and<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<(T, U)>;
    fn or<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<Variant2<T, U>>;
    fn map<TFn: (Fn(T) -> U) + Clone, U: Clone>(self, f: TFn) -> impl Parser<U>;
    fn skip<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        other: UParser,
    ) -> impl Parser<T>;
    fn then<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        other: UParser,
    ) -> impl Parser<U>;
    fn then_eof(self) -> impl Parser<T>;
    fn many(self) -> impl Parser<Vec<T>>;
    fn separated_by_non_empty<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        separator: UParser,
    ) -> impl Parser<Vec<T>>;

    fn parse_string(&self, string: String) -> ParseResult<T>;
    fn parse_str(&self, string: &str) -> ParseResult<T>;
}

impl<TParser: Parser<T> + 'static, T: Clone + 'static> ParserCombinator<TParser, T> for TParser {
    fn and<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<(T, U)> {
        AndParser2::new(self, other)
    }

    fn or<UParser: Parser<U>, U: Clone>(self, other: UParser) -> impl Parser<Variant2<T, U>> {
        OrParser2::new(self, other)
    }

    fn map<TFn: (Fn(T) -> U) + Clone, U: Clone>(self, f: TFn) -> impl Parser<U> {
        MapParser::new(self, f)
    }

    fn skip<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        other: UParser,
    ) -> impl Parser<T> {
        AndParser2::new(self, other).map(|(a, _)| a)
    }

    fn then<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        other: UParser,
    ) -> impl Parser<U> {
        AndParser2::new(self, other).map(|(_, b)| b)
    }

    fn then_eof(self) -> impl Parser<T> {
        AndParser2::new(self, EofParser::new()).map(|(a, _)| a)
    }

    fn many(self) -> impl Parser<Vec<T>> {
        ManyParser::new(self)
    }

    fn separated_by_non_empty<UParser: Parser<U> + 'static, U: Clone + 'static>(
        self,
        separator: UParser,
    ) -> impl Parser<Vec<T>> {
        self.clone().skip(separator).many().and(self).map(|(a, b)| {
            let mut result = a.clone();
            result.push(b);
            result
        })
    }

    fn parse_string(&self, string: String) -> ParseResult<T> {
        let mut context = ParsingContext::new(string, ParsingPosition::new(0, 1, 1));
        self.parse(&mut context)
    }

    fn parse_str(&self, string: &str) -> ParseResult<T> {
        self.parse_string(string.to_owned())
    }
}

pub trait RawParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T>;
}

pub trait Parser<T>: RawParser<T> + Clone {}

impl<T, U: RawParser<T> + Clone> Parser<T> for U {}
