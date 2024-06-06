use std::{cell::Cell, fmt::Debug};

use rustynom_macros::{and_parser, or_parser};

and_parser!(2);
and_parser!(3);
and_parser!(4);
and_parser!(5);

or_parser!(2);
or_parser!(3);
or_parser!(4);
or_parser!(5);

// fn main() {
//     let a = StringParser::new("hello".to_string());
//     let b = StringParser::new("hey".to_string());
//     let c = StringParser::new("world".to_string());
//     let d = SameOrParser2::new(&a, &b);

//     let parser = AndParser2::new(&d, &c);

//     println!("{:?}", parser.parse_str("heyworld"));
//     println!("{:?}", parser.parse_str("helloworld"));
//     println!("{:?}", parser.parse_str("hiworld"));
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsingPosition {
    pub index: usize,
    pub line: u32,
    pub column: u32,
}

impl ParsingPosition {
    pub fn new(index: usize, line: u32, column: u32) -> ParsingPosition {
        ParsingPosition {
            index,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingRange {
    pub from: ParsingPosition,
    pub to: ParsingPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseFailure {
    pub furthest: ParsingPosition,
    pub expected: Vec<String>,
}

impl ParseFailure {
    pub fn new(furthest: ParsingPosition, expected: Vec<String>) -> ParseFailure {
        ParseFailure { furthest, expected }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseResult<T> {
    Success(T),
    Failure(ParseFailure),
}

impl<T> ParseResult<T> {
    pub fn is_success(&self) -> bool {
        match self {
            ParseResult::Success(_) => true,
            ParseResult::Failure(_) => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    pub fn unwrap_success(self) -> T {
        match self {
            ParseResult::Success(t) => t,
            ParseResult::Failure(_) => panic!("Called unwrap_success on a ParseResult::Failure"),
        }
    }

    pub fn unwrap_failure(self) -> ParseFailure {
        match self {
            ParseResult::Failure(f) => f,
            ParseResult::Success(_) => panic!("Called unwrap_failure on a ParseResult::Success"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingContext {
    input: Vec<char>,
    position: ParsingPosition,
}

impl ParsingContext {
    pub fn new(input: String, position: ParsingPosition) -> ParsingContext {
        ParsingContext {
            input: input.chars().collect(),
            position,
        }
    }

    pub fn move_to_position(&mut self, position: ParsingPosition) {
        self.position = position;
    }

    pub fn at_eof(&self) -> bool {
        self.position.index >= self.input.len()
    }

    fn advance_to(&mut self, index: usize) {
        if index < self.position.index {
            panic!("Cannot advance to a position before the current position");
        }
        if index == self.position.index {
            return;
        }

        for i in self.position.index..index {
            if self.input[i] == '\n' {
                self.position.line += 1;
                self.position.column = 0;
            } else {
                self.position.column += 1;
            }
        }

        self.position.index = index;
    }

    pub fn succeed_offset<T>(&mut self, offset: usize, result: T) -> ParseResult<T> {
        self.succeed_at(self.position.index + offset, result)
    }

    pub fn fail_offset<T>(&mut self, offset: usize, expected: Vec<String>) -> ParseResult<T> {
        self.fail_at(self.position.index + offset, expected)
    }

    pub fn succeed_at<T>(&mut self, index: usize, result: T) -> ParseResult<T> {
        self.advance_to(index);
        ParseResult::Success(result)
    }

    pub fn fail_at<T>(&mut self, index: usize, expected: Vec<String>) -> ParseResult<T> {
        self.advance_to(index);
        ParseResult::Failure(ParseFailure::new(self.position.clone(), expected))
    }

    // pub fn merge<T>(&self, a: ParseResult<T>, b: Option<ParseResult<T>>) -> ParseResult<T> {
    //     if b.is_none() {
    //         return a;
    //     }
    //     let b = b.unwrap();

    //     if b.is_success() {
    //         return b;
    //     }

    //     if a.is_success() {
    //         return a;
    //     }

    //     ParseResult::Failure(self.merge_failures(a.unwrap_failure(), b.unwrap_failure()))
    // }

    pub fn merge_failures(&self, mut a: ParseFailure, b: ParseFailure) -> ParseFailure {
        match a.furthest.cmp(&b.furthest) {
            std::cmp::Ordering::Less => b,
            std::cmp::Ordering::Greater => a,
            std::cmp::Ordering::Equal => {
                a.expected.extend(b.expected);
                a
            }
        }
    }
}

// maybe use cow here so that we can also pass owned stuff.
pub trait ParserCombinator<TParser, T>
where
    TParser: Parser<T>,
{
    fn and<'a, U>(&'a self, other: &'a dyn Parser<U>) -> AndParser2<'a, T, U>;
    fn or<'a, U>(&'a self, other: &'a dyn Parser<U>) -> OrParser2<'a, T, U>;
    fn map<U>(&self, f: impl Fn(T) -> U + 'static) -> MapParser<'_, TParser, T, U>;
    fn owned_map<U>(self, f: impl Fn(T) -> U + 'static) -> OwnedMapParser<TParser, T, U>;

    fn parse_string(&self, string: String) -> ParseResult<T>;
    fn parse_str(&self, string: &str) -> ParseResult<T>;
}

impl<TParser, T> ParserCombinator<TParser, T> for TParser
where
    TParser: Parser<T>,
{
    fn and<'a, U>(&'a self, other: &'a dyn Parser<U>) -> AndParser2<'a, T, U> {
        AndParser2::new(self, other)
    }

    fn or<'a, U>(&'a self, other: &'a dyn Parser<U>) -> OrParser2<'a, T, U> {
        OrParser2::new(self, other)
    }

    fn map<U>(&self, f: impl Fn(T) -> U + 'static) -> MapParser<'_, TParser, T, U> {
        MapParser::new(self, f)
    }

    fn owned_map<U>(self, f: impl Fn(T) -> U + 'static) -> OwnedMapParser<TParser, T, U> {
        OwnedMapParser::new(self, f)
    }

    fn parse_string(&self, string: String) -> ParseResult<T> {
        let mut context = ParsingContext::new(string, ParsingPosition::new(0, 1, 1));
        self.parse(&mut context)
    }

    fn parse_str(&self, string: &str) -> ParseResult<T> {
        self.parse_string(string.to_owned())
    }
}

pub trait Parser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T>;
}

pub struct StringParser {
    string: String,
}

impl StringParser {
    pub fn new(string: String) -> StringParser {
        StringParser { string }
    }
}

impl Parser<String> for StringParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<String> {
        for (i, c) in self.string.chars().enumerate() {
            if context.position.index + i >= context.input.len() {
                return context.fail_at(context.position.index, vec![self.string.clone()]);
            }

            if c != context.input[context.position.index + i] {
                return context.fail_at(context.position.index, vec![self.string.clone()]);
            }
        }

        context.succeed_offset(self.string.len(), self.string.clone())
    }
}

pub struct CharParser {
    c: char,
}

impl CharParser {
    pub fn new(c: char) -> CharParser {
        CharParser { c }
    }
}

impl Parser<char> for CharParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<char> {
        if context.at_eof() {
            return context.fail_offset(0, vec![self.c.to_string()]);
        }

        if self.c == context.input[context.position.index] {
            context.succeed_offset(1, self.c)
        } else {
            context.fail_offset(0, vec![self.c.to_string()])
        }
    }
}

pub struct EofParser;

impl EofParser {
    pub fn new() -> EofParser {
        EofParser
    }
}

impl Parser<()> for EofParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<()> {
        if context.at_eof() {
            context.succeed_offset(0, ())
        } else {
            context.fail_offset(0, vec!["EOF".to_string()])
        }
    }
}

pub struct RecParser<'a, T> {
    parser: Cell<Option<&'a dyn Parser<T>>>,
}

impl<'a, T> RecParser<'a, T> {
    pub fn new() -> RecParser<'a, T> {
        RecParser {
            parser: Cell::new(None),
        }
    }

    pub fn set(&self, parser: &'a dyn Parser<T>) {
        self.parser.set(Some(parser));
    }
}

impl<'a, T> Parser<T> for RecParser<'a, T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        if let Some(parser) = self.parser.get() {
            parser.parse(context)
        } else {
            panic!("RecParser has no parser set");
        }
    }
}

// pub struct MapParser<'a, TParser, T, U>
// where
//     TParser: Parser<T>,
// {
//     parser: MaybeOwned<'a, TParser>,
//     f: Box<dyn Fn(T) -> U>,
// }

// impl<'a, TParser, T, U> MapParser<'a, TParser, T, U>
// where
//     TParser: Parser<T>,
// {
//     pub fn new(
//         parser: MaybeOwned<'a, TParser>,
//         f: impl Fn(T) -> U + 'static,
//     ) -> MapParser<'a, TParser, T, U> {
//         MapParser {
//             parser,
//             f: Box::new(f),
//         }
//     }
// }

// impl<'a, TParser, T, U> Parser<U> for MapParser<'a, TParser, T, U>
// where
//     TParser: Parser<T>,
// {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<U> {
//         match self.parser.as_ref().parse(context) {
//             ParseResult::Success(a) => ParseResult::Success((self.f)(a)),
//             ParseResult::Failure(f) => ParseResult::Failure(f),
//         }
//     }
// }

// pub enum MaybeOwned<'a, T: 'a> {
//     Owned(T),
//     Borrowed(&'a T),
// }

// impl<'a, T> MaybeOwned<'a, T> {
//     pub fn as_ref(&self) -> &T {
//         match self {
//             MaybeOwned::Borrowed(t) => t,
//             MaybeOwned::Owned(t) => &t,
//         }
//     }
// }

pub struct MapParser<'a, TParser, T, U>
where
    TParser: Parser<T>,
{
    parser: &'a TParser,
    f: Box<dyn Fn(T) -> U>,
}

impl<'a, TParser, T, U> MapParser<'a, TParser, T, U>
where
    TParser: Parser<T>,
{
    pub fn new(parser: &'a TParser, f: impl Fn(T) -> U + 'static) -> MapParser<'a, TParser, T, U> {
        MapParser {
            parser,
            f: Box::new(f),
        }
    }
}

impl<'a, TParser, T, U> Parser<U> for MapParser<'a, TParser, T, U>
where
    TParser: Parser<T>,
{
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<U> {
        match self.parser.parse(context) {
            ParseResult::Success(a) => ParseResult::Success((self.f)(a)),
            ParseResult::Failure(f) => ParseResult::Failure(f),
        }
    }
}

pub struct OwnedMapParser<TParser, T, U>
where
    TParser: Parser<T>,
{
    parser: TParser,
    f: Box<dyn Fn(T) -> U>,
}

impl<TParser, T, U> OwnedMapParser<TParser, T, U>
where
    TParser: Parser<T>,
{
    pub fn new(parser: TParser, f: impl Fn(T) -> U + 'static) -> OwnedMapParser<TParser, T, U> {
        OwnedMapParser {
            parser,
            f: Box::new(f),
        }
    }
}

impl<TParser, T, U> Parser<U> for OwnedMapParser<TParser, T, U>
where
    TParser: Parser<T>,
{
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<U> {
        match self.parser.parse(context) {
            ParseResult::Success(a) => ParseResult::Success((self.f)(a)),
            ParseResult::Failure(f) => ParseResult::Failure(f),
        }
    }
}

pub struct ManyParser<'a, TParser, T> where TParser: Parser<T> {
    parser: &'a TParser,
    __phantom: std::marker::PhantomData<T>,
}

impl<'a, TParser, T> ManyParser<'a, TParser, T> where TParser: Parser<T> {
    pub fn new(parser: &'a TParser) -> ManyParser<'a, TParser, T> {
        ManyParser {
            parser,
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, TParser, T> Parser<Vec<T>> for ManyParser<'a, TParser, T> where TParser: Parser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<Vec<T>> {
        let mut result = Vec::new();
        loop {
            match self.parser.parse(context) {
                ParseResult::Success(t) => result.push(t),
                ParseResult::Failure(_) => break,
            }
        }

        ParseResult::Success(result)
    }
}

// pub struct AndParser<'a, A, B> {
//     a: Box<&'a dyn Parser<A>>,
//     b: Box<&'a dyn Parser<B>>,
// }

// impl<'a, A, B> AndParser<'a, A, B> {
//     pub fn new(a: &'a dyn Parser<A>, b: &'a dyn Parser<B>) -> AndParser<'a, A, B> {
//         AndParser {
//             a: Box::from(a),
//             b: Box::from(b),
//         }
//     }
// }

// impl<'a, A, B> Parser<(A, B)> for AndParser<'a, A, B> {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<(A, B)> {
//         match (self.a.parse(context), self.b.parse(context)) {
//             (ParseResult::Success(a), ParseResult::Success(b)) => ParseResult::Success((a, b)),
//             (ParseResult::Failure(f), _) => ParseResult::Failure(f),
//             (_, ParseResult::Failure(f)) => ParseResult::Failure(f),
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Variant<T1, T2> {
//     V1(T1),
//     V2(T2),
// }

// pub struct OrParser<'a, A, B> {
//     a: Box<&'a dyn Parser<A>>,
//     b: Box<&'a dyn Parser<B>>,
// }

// impl<'a, A, B> OrParser<'a, A, B> {
//     pub fn new(a: &'a dyn Parser<A>, b: &'a dyn Parser<B>) -> OrParser<'a, A, B> {
//         OrParser {
//             a: Box::from(a),
//             b: Box::from(b),
//         }
//     }
// }

// impl<'a, A, B> Parser<Variant<A, B>> for OrParser<'a, A, B> {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<Variant<A, B>> {
//         let mut cloned_context = context.clone();
//         let a = self.a.parse(&mut cloned_context);

//         if let ParseResult::Success(a) = a {
//             return context.succeed_at(cloned_context.position.index, Variant::V1(a));
//         }

//         match self.b.parse(context) {
//             ParseResult::Success(b) => ParseResult::Success(Variant::V2(b)),
//             ParseResult::Failure(f) => ParseResult::Failure(f),
//         }
//     }
// }
