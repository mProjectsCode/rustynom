// ---------------
// Recursive parser
// ---------------

use std::{cell::RefCell, rc::Rc};

use crate::{
    ParseResult, ParsingPosition,
    parsable::Parsable,
    parser::{ParserOut, RawTestParser},
};

pub struct RecRefParser<TIn: Parsable, TOut: Clone, const ERROR: bool> {
    parser_ref: Rc<RefCell<Option<Box<dyn RawTestParser<TIn, ERROR, TOut = TOut>>>>>,
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> Clone for RecRefParser<TIn, TOut, ERROR> {
    fn clone(&self) -> Self {
        RecRefParser {
            parser_ref: self.parser_ref.clone(),
        }
    }
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> RecRefParser<TIn, TOut, ERROR> {
    pub fn new() -> Self {
        RecRefParser {
            parser_ref: Rc::new(RefCell::new(None)),
        }
    }

    pub fn set(&self, parser: Box<dyn RawTestParser<TIn, ERROR, TOut = TOut>>) {
        self.parser_ref.borrow_mut().replace(parser);
    }
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> RawTestParser<TIn, ERROR>
    for RecRefParser<TIn, TOut, ERROR>
{
    type TOut
        = TOut
    where
        TIn::T:;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TOut> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("RecRefParser has no parser set")
            .parse(input, position)
    }
}

#[derive(Clone)]
pub struct RecParser<TIn: Parsable, TOut: Clone, const ERROR: bool> {
    parser: RecRefParser<TIn, TOut, ERROR>,
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> RecParser<TIn, TOut, ERROR> {
    pub fn new<TP>(decl: impl FnOnce(RecRefParser<TIn, TOut, ERROR>) -> TP) -> Self
    where
        TP: RawTestParser<TIn, ERROR, TOut = TOut> + 'static,
    {
        let rec_ref = RecRefParser::new();

        let parser = decl(rec_ref.clone());

        rec_ref.set(Box::from(parser));

        RecParser { parser: rec_ref }
    }
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> RawTestParser<TIn, ERROR>
    for RecParser<TIn, TOut, ERROR>
{
    type TOut
        = TOut
    where
        TIn::T:;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TOut> {
        self.parser.parse(input, position)
    }
}

#[derive(Clone)]
pub struct MapParser<
    TIn: Parsable,
    TOut: Clone,
    TP: RawTestParser<TIn, ERROR>,
    TFn: (Fn(<TP as RawTestParser<TIn, ERROR>>::TOut) -> TOut),
    const ERROR: bool,
> where
    TIn::T:,
{
    parser: TP,
    f: TFn,
    __phantom1: std::marker::PhantomData<TIn>,
    __phantom2: std::marker::PhantomData<TOut>,
}

impl<
    TIn: Parsable,
    TOut: Clone,
    TP: RawTestParser<TIn, ERROR>,
    TFn: (Fn(<TP as RawTestParser<TIn, ERROR>>::TOut) -> TOut),
    const ERROR: bool,
> MapParser<TIn, TOut, TP, TFn, ERROR>
{
    pub fn new(parser: TP, f: TFn) -> Self {
        MapParser {
            parser,
            f,
            __phantom1: std::marker::PhantomData,
            __phantom2: std::marker::PhantomData,
        }
    }
}

impl<
    TIn: Parsable,
    TOut: Clone,
    TP: RawTestParser<TIn, ERROR>,
    TFn: (Fn(<TP as RawTestParser<TIn, ERROR>>::TOut) -> TOut),
    const ERROR: bool,
> RawTestParser<TIn, ERROR> for MapParser<TIn, TOut, TP, TFn, ERROR>
{
    type TOut
        = TOut
    where
        TIn::T:;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TOut> {
        match self.parser.parse(input, position) {
            ParseResult::Success(x) => ParseResult::Success((self.f)(x)),
            ParseResult::Failure(f) => ParseResult::Failure(f),
        }
    }
}

// ---------------
// Many parser
// ---------------

#[derive(Clone)]
pub struct ManyParser<TIn: Parsable, TP: RawTestParser<TIn, false>> {
    parser: TP,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, TP: RawTestParser<TIn, false>> ManyParser<TIn, TP> {
    pub fn new(parser: TP) -> Self {
        ManyParser {
            parser,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<TIn: Parsable, TP: RawTestParser<TIn, false>, const ERROR: bool> RawTestParser<TIn, ERROR>
    for ManyParser<TIn, TP>
{
    type TOut
        = Vec<ParserOut<TP, TIn, false>>
    where
        TIn::T:;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        let mut result = Vec::new();
        while let ParseResult::Success(t) = self.parser.parse(input, position) {
            result.push(t);
        }
        position.succeed_offset(0, result)
    }
}

// ---------------
// Many-non-empty parser
// ---------------

#[derive(Clone)]
pub struct ManyNonEmptyParser<TIn: Parsable, TP: RawTestParser<TIn, ERROR>, const ERROR: bool> {
    parser: TP,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, TP: RawTestParser<TIn, ERROR>, const ERROR: bool>
    ManyNonEmptyParser<TIn, TP, ERROR>
{
    pub fn new(parser: TP) -> Self {
        ManyNonEmptyParser {
            parser,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<TIn: Parsable, TP: RawTestParser<TIn, ERROR>, const ERROR: bool> RawTestParser<TIn, ERROR>
    for ManyNonEmptyParser<TIn, TP, ERROR>
{
    type TOut
        = Vec<ParserOut<TP, TIn, ERROR>>
    where
        TIn::T:;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        let mut result = Vec::new();

        match self.parser.parse(input, position) {
            ParseResult::Success(t) => result.push(t),
            ParseResult::Failure(f) => return ParseResult::Failure(f),
        }

        while let ParseResult::Success(t) = self.parser.parse(input, position) {
            result.push(t);
        }
        position.succeed_offset(0, result)
    }
}

// ---------------
// Optional parser
// ---------------

#[derive(Clone)]
pub struct OptionalParser<TIn: Parsable, TP: RawTestParser<TIn, false>> {
    parser: TP,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, TP: RawTestParser<TIn, false>> OptionalParser<TIn, TP> {
    pub fn new(parser: TP) -> Self {
        OptionalParser {
            parser,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<TIn: Parsable, TP: RawTestParser<TIn, false>, const ERROR: bool> RawTestParser<TIn, ERROR>
    for OptionalParser<TIn, TP>
{
    type TOut
        = Option<ParserOut<TP, TIn, false>>
    where
        TIn::T:;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        match self.parser.parse(input, position) {
            ParseResult::Success(t) => position.succeed_offset(0, Some(t)),
            ParseResult::Failure(_) => position.succeed_offset(0, None),
        }
    }
}
