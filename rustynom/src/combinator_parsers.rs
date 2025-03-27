use crate::{
    GenericParseResult, Parsable, ParseResult, ParsingPosition,
    parser::{ParserOut, RawTestParser},
};
use rustynom_macros::{and_parser, or_parser};

// ---------------
// And parser
// ---------------

and_parser!(2);
and_parser!(3);
and_parser!(4);
and_parser!(5);
and_parser!(6);
and_parser!(7);
and_parser!(8);

// ---------------
// Or parser
// ---------------

or_parser!(2);
or_parser!(3);
or_parser!(4);
or_parser!(5);
or_parser!(6);
or_parser!(7);
or_parser!(8);

// ---------------
// Skip and Then
// ---------------

#[derive(Clone)]
pub struct SkipParser<
    TIn: Parsable,
    P1: RawTestParser<TIn, ERROR>,
    P2: RawTestParser<TIn, ERROR>,
    const ERROR: bool,
> {
    parser1: P1,
    parser2: P2,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, P1: RawTestParser<TIn, ERROR>, P2: RawTestParser<TIn, ERROR>, const ERROR: bool>
    SkipParser<TIn, P1, P2, ERROR>
{
    pub fn new(parser1: P1, parser2: P2) -> Self {
        SkipParser {
            parser1,
            parser2,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<TIn: Parsable, P1: RawTestParser<TIn, ERROR>, P2: RawTestParser<TIn, ERROR>, const ERROR: bool>
    RawTestParser<TIn, ERROR> for SkipParser<TIn, P1, P2, ERROR>
{
    type TOut = ParserOut<P1, TIn, ERROR>;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        let result1 = self.parser1.parse(input, position);
        if let GenericParseResult::Failure(f) = result1 {
            return GenericParseResult::Failure(f);
        }

        let result2 = self.parser2.parse(input, position);
        if let GenericParseResult::Failure(f) = result2 {
            return GenericParseResult::Failure(f);
        }

        result1
    }
}

#[derive(Clone)]
pub struct ThenParser<
    TIn: Parsable,
    P1: RawTestParser<TIn, ERROR>,
    P2: RawTestParser<TIn, ERROR>,
    const ERROR: bool,
> {
    parser1: P1,
    parser2: P2,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, P1: RawTestParser<TIn, ERROR>, P2: RawTestParser<TIn, ERROR>, const ERROR: bool>
    ThenParser<TIn, P1, P2, ERROR>
{
    pub fn new(parser1: P1, parser2: P2) -> Self {
        ThenParser {
            parser1,
            parser2,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<TIn: Parsable, P1: RawTestParser<TIn, ERROR>, P2: RawTestParser<TIn, ERROR>, const ERROR: bool>
    RawTestParser<TIn, ERROR> for ThenParser<TIn, P1, P2, ERROR>
{
    type TOut = ParserOut<P2, TIn, ERROR>;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        let result1 = self.parser1.parse(input, position);
        if let GenericParseResult::Failure(f) = result1 {
            return GenericParseResult::Failure(f);
        }

        let result2 = self.parser2.parse(input, position);
        if let GenericParseResult::Failure(f) = result2 {
            return GenericParseResult::Failure(f);
        }

        result2
    }
}

// ---------------
// Separated by Parser
// ---------------

#[derive(Clone)]
pub struct SeparatedByParser<
    TIn: Parsable,
    P1: RawTestParser<TIn, ERROR>,
    P2: RawTestParser<TIn, ERROR>,
    const ERROR: bool,
> {
    parser: P1,
    separator: P2,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, P1: RawTestParser<TIn, ERROR>, P2: RawTestParser<TIn, ERROR>, const ERROR: bool>
    SeparatedByParser<TIn, P1, P2, ERROR>
{
    pub fn new(parser: P1, separator: P2) -> Self {
        SeparatedByParser {
            parser,
            separator,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<TIn: Parsable, P1: RawTestParser<TIn, ERROR>, P2: RawTestParser<TIn, ERROR>, const ERROR: bool>
    RawTestParser<TIn, ERROR> for SeparatedByParser<TIn, P1, P2, ERROR>
{
    type TOut = Vec<ParserOut<P1, TIn, ERROR>>;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        let mut result = Vec::new();

        let first = self.parser.parse(input, position);

        if first.is_failure() {
            return ParseResult::Failure(first.unwrap_failure());
        }

        result.push(first.unwrap_success());

        let mut cloned_position = position.clone();
        loop {
            let separator_result = self.separator.parse(input, position);
            if separator_result.is_failure() {
                // if the separator fails, the sequence is over and we return the result
                // we don't need to move the position back because the separator failed
                return ParseResult::Success(result);
            }

            let self_result = self.parser.parse(input, position);
            if self_result.is_failure() {
                // if the self parser fails, we return the result
                // we need to move the position back to the last successful position
                position.advance_to(cloned_position);
                return ParseResult::Success(result);
            }

            result.push(self_result.unwrap_success());
            cloned_position = position.clone();
        }
    }
}

// ---------------
// Surround Parser
// ---------------

#[derive(Clone)]
pub struct SurroundParser<
    TIn: Parsable,
    P: RawTestParser<TIn, ERROR>,
    PL: RawTestParser<TIn, ERROR>,
    PR: RawTestParser<TIn, ERROR>,
    const ERROR: bool,
> {
    parser: P,
    left: PL,
    right: PR,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<
    TIn: Parsable,
    P: RawTestParser<TIn, ERROR>,
    PL: RawTestParser<TIn, ERROR>,
    PR: RawTestParser<TIn, ERROR>,
    const ERROR: bool,
> SurroundParser<TIn, P, PL, PR, ERROR>
{
    pub fn new(parser: P, left: PL, right: PR) -> Self {
        SurroundParser {
            parser,
            left,
            right,
            __phantom1: std::marker::PhantomData,
        }
    }
}

impl<
    TIn: Parsable,
    P: RawTestParser<TIn, ERROR>,
    PL: RawTestParser<TIn, ERROR>,
    PR: RawTestParser<TIn, ERROR>,
    const ERROR: bool,
> RawTestParser<TIn, ERROR> for SurroundParser<TIn, P, PL, PR, ERROR>
{
    type TOut = ParserOut<P, TIn, ERROR>;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut> {
        let result1 = self.left.parse(input, position);
        if let GenericParseResult::Failure(f) = result1 {
            return GenericParseResult::Failure(f);
        }

        let result2 = self.parser.parse(input, position);
        if let GenericParseResult::Failure(f) = result2 {
            return GenericParseResult::Failure(f);
        }

        let result3 = self.right.parse(input, position);
        if let GenericParseResult::Failure(f) = result3 {
            return GenericParseResult::Failure(f);
        }

        result2
    }
}
