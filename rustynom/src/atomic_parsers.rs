use crate::{ParseResult, ParsingPosition, parsable::Parsable, parser::RawTestParser};

// ---------------
// String parser
// ---------------

#[derive(Clone)]
pub struct LiteralListParser<TIn: Parsable> {
    list: TIn::List,
    slice: Box<[TIn::T]>,
}

impl<TIn: Parsable> LiteralListParser<TIn> {
    pub fn new(list: TIn::List) -> Self {
        LiteralListParser {
            list: list.clone(),
            slice: TIn::list_to_owned_slice(list),
        }
    }
}

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for LiteralListParser<TIn> {
    type TOut = TIn::List;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TIn::List> {
        let error = if ERROR {
            Some(vec![TIn::list_to_string(&self.list)])
        } else {
            None
        };

        match position.current_eq_slice(input, &self.slice) {
            true => position.succeed_offset(self.slice.len(), self.list.clone()),
            false => position.fail_offset(0, error),
        }
    }
}

#[derive(Clone)]
pub struct LiteralListParserNoOutput<TIn: Parsable> {
    list: TIn::List,
    slice: Box<[TIn::T]>,
}

impl<TIn: Parsable> LiteralListParserNoOutput<TIn> {
    pub fn new(list: TIn::List) -> Self {
        LiteralListParserNoOutput {
            list: list.clone(),
            slice: TIn::list_to_owned_slice(list),
        }
    }
}

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR>
    for LiteralListParserNoOutput<TIn>
{
    type TOut = ();

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<()> {
        let error = if ERROR {
            Some(vec![TIn::list_to_string(&self.list)])
        } else {
            None
        };

        match position.current_eq_slice(input, &self.slice) {
            true => position.succeed_offset(self.slice.len(), ()),
            false => position.fail_offset(0, error),
        }
    }
}

// ---------------
// String parser with custom result
// ---------------

#[derive(Clone)]
pub struct LiteralListMapParser<TIn: Parsable, TOut: Clone> {
    out: TOut,
    list: TIn::List,
    slice: Box<[TIn::T]>,
}

impl<TIn: Parsable, TOut: Clone> LiteralListMapParser<TIn, TOut> {
    pub fn new(list: TIn::List, out: TOut) -> Self {
        LiteralListMapParser {
            out,
            list: list.clone(),
            slice: TIn::list_to_owned_slice(list),
        }
    }
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> RawTestParser<TIn, ERROR>
    for LiteralListMapParser<TIn, TOut>
{
    type TOut = TOut;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TOut> {
        let error = if ERROR {
            Some(vec![TIn::list_to_string(&self.list)])
        } else {
            None
        };

        match position.current_eq_slice(input, &self.slice) {
            true => position.succeed_offset(self.slice.len(), self.out.clone()),
            false => position.fail_offset(0, error),
        }
    }
}

// ---------------
// Literal parser
// ---------------

#[derive(Clone)]
pub struct LiteralParser<TIn: Parsable> {
    literal: TIn::T,
}

impl<TIn: Parsable> LiteralParser<TIn> {
    pub fn new(t: TIn::T) -> Self {
        LiteralParser { literal: t }
    }
}

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for LiteralParser<TIn> {
    type TOut = TIn::T;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TIn::T> {
        let error = if ERROR {
            Some(vec![TIn::t_to_string(&self.literal)])
        } else {
            None
        };

        if position.at_eof(input) {
            return position.fail_offset(0, error);
        }

        if position.current_eq(input, &self.literal) {
            position.succeed_offset(1, position.current(input).clone())
        } else {
            position.fail_offset(0, error)
        }
    }
}

#[derive(Clone)]
pub struct LiteralParserNoOutput<TIn: Parsable> {
    literal: TIn::T,
}

impl<TIn: Parsable> LiteralParserNoOutput<TIn> {
    pub fn new(t: TIn::T) -> Self {
        LiteralParserNoOutput { literal: t }
    }
}

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for LiteralParserNoOutput<TIn> {
    type TOut = ();

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<()> {
        let error = if ERROR {
            Some(vec![TIn::t_to_string(&self.literal)])
        } else {
            None
        };

        if position.at_eof(input) {
            return position.fail_offset(0, error);
        }

        if position.current_eq(input, &self.literal) {
            position.succeed_offset(1, ())
        } else {
            position.fail_offset(0, error)
        }
    }
}

// ---------------
// EOF parser
// ---------------

#[derive(Clone)]
pub struct EofParser;

impl EofParser {
    pub fn new() -> EofParser {
        EofParser
    }
}

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for EofParser {
    type TOut = ();

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<()> {
        if position.at_eof(input) {
            position.succeed_offset(0, ())
        } else {
            let error = if ERROR {
                Some(vec!["EOF".to_string()])
            } else {
                None
            };
            position.fail_offset(0, error)
        }
    }
}

// ---------------
// Success parser
// ---------------

#[derive(Clone)]
pub struct SuccessParser<TOut: Clone> {
    result: TOut,
}

impl<TOut: Clone> SuccessParser<TOut> {
    pub fn new(result: TOut) -> SuccessParser<TOut> {
        SuccessParser { result }
    }
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> RawTestParser<TIn, ERROR>
    for SuccessParser<TOut>
{
    type TOut = TOut;

    fn parse(&self, _input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TOut> {
        position.succeed_offset(0, self.result.clone())
    }
}

// ---------------
// Custom parser
// ---------------

#[derive(Clone)]
pub struct CustomParser<
    TIn: Parsable,
    TOut: Clone,
    TFn: Fn(&[TIn::T], &mut ParsingPosition) -> ParseResult<TOut>,
> {
    f: TFn,
    __phantom1: std::marker::PhantomData<TIn>,
    __phantom2: std::marker::PhantomData<TOut>,
}

impl<'b, TIn: Parsable, TOut: Clone, TFn: Fn(&[TIn::T], &mut ParsingPosition) -> ParseResult<TOut>>
    CustomParser<TIn, TOut, TFn>
{
    pub fn new(f: TFn) -> Self {
        CustomParser {
            f,
            __phantom1: std::marker::PhantomData,
            __phantom2: std::marker::PhantomData,
        }
    }
}

impl<
    'a,
    TIn: Parsable,
    TOut: Clone,
    TFn: Fn(&[TIn::T], &mut ParsingPosition) -> ParseResult<TOut>,
    const ERROR: bool,
> RawTestParser<TIn, ERROR> for CustomParser<TIn, TOut, TFn>
{
    type TOut = TOut;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TOut> {
        (self.f)(input, position)
    }
}
