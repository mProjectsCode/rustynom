// ---------------
// Recursive parser
// ---------------

use std::{cell::RefCell, rc::Rc};

use crate::{parser::RawParser, ParseResult, ParsingContext};

pub struct RecRefParser<T> {
    parser_ref: Rc<RefCell<Option<Box<dyn RawParser<T>>>>>,
    __phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for RecRefParser<T> {
    fn clone(&self) -> Self {
        RecRefParser {
            parser_ref: self.parser_ref.clone(),
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<T> RecRefParser<T> {
    pub fn new() -> RecRefParser<T> {
        RecRefParser {
            parser_ref: Rc::new(RefCell::new(None)),
            __phantom: std::marker::PhantomData,
        }
    }

    pub fn set(&self, parser: Box<dyn RawParser<T>>) {
        self.parser_ref.borrow_mut().replace(parser);
    }
}

impl<T> RawParser<T> for RecRefParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("RecRefParser has no parser set")
            .parse(context)
    }
}

#[derive(Clone)]
pub struct RecParser<T> {
    parser: RecRefParser<T>,
}

impl<T> RecParser<T> {
    pub fn new<TParser: RawParser<T> + 'static>(
        decl: impl FnOnce(RecRefParser<T>) -> TParser,
    ) -> RecParser<T> {
        let rec_ref = RecRefParser::new();

        let parser = decl(rec_ref.clone());

        rec_ref.set(Box::from(parser));

        RecParser { parser: rec_ref }
    }
}

impl<T: Clone> RawParser<T> for RecParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        self.parser.parse(context)
    }
}

#[derive(Clone)]
pub struct MapParser<TParser: RawParser<T>, T: Clone, TFn: (Fn(T) -> U) + Clone, U: Clone> {
    parser: TParser,
    f: TFn,
    __phantom1: std::marker::PhantomData<T>,
    __phantom2: std::marker::PhantomData<U>,
}

impl<TParser: RawParser<T>, T: Clone, TFn: (Fn(T) -> U) + Clone, U: Clone>
    MapParser<TParser, T, TFn, U>
{
    pub fn new(parser: TParser, f: TFn) -> MapParser<TParser, T, TFn, U> {
        MapParser {
            parser,
            f,
            __phantom1: std::marker::PhantomData,
            __phantom2: std::marker::PhantomData,
        }
    }
}

impl<TParser: RawParser<T>, T: Clone, TFn: (Fn(T) -> U) + Clone, U: Clone> RawParser<U>
    for MapParser<TParser, T, TFn, U>
{
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<U> {
        match self.parser.parse(context) {
            ParseResult::Success(a) => ParseResult::Success((self.f)(a)),
            ParseResult::Failure(f) => ParseResult::Failure(f),
        }
    }
}

// ---------------
// Many parser
// ---------------

#[derive(Clone)]
pub struct ManyParser<TParser, T>
where
    TParser: RawParser<T>,
{
    parser: TParser,
    __phantom: std::marker::PhantomData<T>,
}

impl<TParser, T> ManyParser<TParser, T>
where
    TParser: RawParser<T>,
{
    pub fn new(parser: TParser) -> ManyParser<TParser, T> {
        ManyParser {
            parser,
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<TParser: RawParser<T>, T: Clone> RawParser<Vec<T>> for ManyParser<TParser, T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<Vec<T>> {
        let mut result = Vec::new();
        while let ParseResult::Success(t) = self.parser.parse(context) {
            result.push(t);
        }

        ParseResult::Success(result)
    }
}
