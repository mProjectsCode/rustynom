use crate::{ParseResult, ParsingPosition, parsable::Parsable, parser::RawTestParser};

#[derive(Clone)]
pub struct PositionParser;

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for PositionParser {
    type TOut = ParsingPosition;

    fn parse(
        &self,
        _input: &[TIn::T],
        position: &mut ParsingPosition,
    ) -> ParseResult<ParsingPosition> {
        ParseResult::Success(position.clone())
    }
}

pub fn position<TIn: Parsable>() -> PositionParser {
    PositionParser
}

#[derive(Clone)]
pub struct AnyParser;

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for AnyParser {
    type TOut = TIn::T;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TIn::T> {
        let error = if ERROR {
            Some(vec!["any character".to_string()])
        } else {
            None
        };

        if position.at_eof(input) {
            position.fail_offset(0, error)
        } else {
            position.succeed_offset(1, position.current(input).clone())
        }
    }
}

pub fn any<TIn: Parsable>() -> AnyParser {
    AnyParser
}

#[derive(Clone)]
pub struct RemainingParser;

impl<TIn: Parsable, const ERROR: bool> RawTestParser<TIn, ERROR> for RemainingParser {
    type TOut = TIn::List;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TIn::List> {
        let slice = position.slice(input);
        let len = slice.len();
        let remaining = TIn::slice_to_list(slice);
        position.succeed_offset(len, remaining)
    }
}

pub fn remaining<TIn: Parsable>() -> RemainingParser {
    RemainingParser
}

#[derive(Clone)]
pub struct TestParser<TIn: Parsable, TFn: Fn(&TIn::T) -> bool> {
    test_fn: TFn,
    error_str: String,
    __phantom: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, TFn: Fn(&TIn::T) -> bool, const ERROR: bool> RawTestParser<TIn, ERROR>
    for TestParser<TIn, TFn>
{
    type TOut = TIn::T;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TIn::T> {
        let error = if ERROR {
            Some(vec![self.error_str.clone()])
        } else {
            None
        };

        if position.test_current(input, &self.test_fn) {
            position.succeed_offset(1, position.current(input).clone())
        } else {
            position.fail_offset(0, error)
        }
    }
}

pub fn test<TIn: Parsable, TFn: Fn(&TIn::T) -> bool + Clone>(
    test_fn: TFn,
    error_str: String,
) -> TestParser<TIn, TFn> {
    TestParser {
        test_fn,
        error_str,
        __phantom: std::marker::PhantomData,
    }
}

#[derive(Clone)]
pub struct MultiTestParser<TIn: Parsable, TFn: Fn(&TIn::T) -> bool> {
    test_fn: TFn,
    error_str: String,
    __phantom: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, TFn: Fn(&TIn::T) -> bool, const ERROR: bool> RawTestParser<TIn, ERROR>
    for MultiTestParser<TIn, TFn>
{
    type TOut = TIn::List;

    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<TIn::List> {
        let error = if ERROR {
            Some(vec![self.error_str.clone()])
        } else {
            None
        };

        let mut index = position.index();
        while index < input.len() && (self.test_fn)(&input[index]) {
            index += 1;
        }

        let slice = position.slice_to(input, index);

        if slice.is_empty() {
            position.fail_offset(0, error)
        } else {
            position.succeed_at(index.into(), TIn::slice_to_list(slice))
        }
    }
}

pub fn multi_test<TIn: Parsable, TFn: Fn(&TIn::T) -> bool + Clone>(
    test_fn: TFn,
    error_str: String,
) -> MultiTestParser<TIn, TFn> {
    MultiTestParser {
        test_fn,
        error_str,
        __phantom: std::marker::PhantomData,
    }
}

#[derive(Clone)]
pub struct MultiTestWithReduceParser<
    TIn: Parsable,
    TOut: Clone,
    TAcc: Clone,
    TFn: Fn(&TIn::T) -> Option<TOut>,
    TRed: Fn(&mut TAcc, TOut, usize),
> {
    test_fn: TFn,
    reduce_fn: TRed,
    initial: TAcc,
    error_str: String,
    __phantom1: std::marker::PhantomData<TIn>,
    __phantom2: std::marker::PhantomData<TOut>,
}

impl<
    'a,
    TIn: Parsable,
    TOut: Clone,
    TAcc: Clone,
    TFn: Fn(&TIn::T) -> Option<TOut>,
    TRed: Fn(&mut TAcc, TOut, usize),
    const ERROR: bool,
> RawTestParser<TIn, ERROR> for MultiTestWithReduceParser<TIn, TOut, TAcc, TFn, TRed>
{
    type TOut = (TAcc, usize);

    fn parse(
        &self,
        input: &[TIn::T],
        position: &mut ParsingPosition,
    ) -> ParseResult<(TAcc, usize)> {
        let mut index = position.index();
        let mut acc = self.initial.clone();

        while index < input.len() {
            if let Some(x) = (self.test_fn)(&input[index]) {
                (self.reduce_fn)(&mut acc, x, index - position.index());

                index += 1;
            } else {
                break;
            }
        }

        if index == position.index() {
            let error = if ERROR {
                Some(vec![self.error_str.clone()])
            } else {
                None
            };
            position.fail_offset(0, error)
        } else {
            position.succeed_at(index.into(), (acc, index - position.index()))
        }
    }
}

pub fn multi_test_with_reduce<
    TIn: Parsable,
    TOut: Clone,
    TAcc: Clone,
    TFn: (Fn(&TIn::T) -> Option<TOut>) + Clone,
    TRed: (Fn(&mut TAcc, TOut, usize)) + Clone,
>(
    test_fn: TFn,
    reduce_fn: TRed,
    initial: TAcc,
    error_str: String,
) -> MultiTestWithReduceParser<TIn, TOut, TAcc, TFn, TRed> {
    MultiTestWithReduceParser {
        test_fn,
        reduce_fn,
        initial,
        error_str,
        __phantom1: std::marker::PhantomData,
        __phantom2: std::marker::PhantomData,
    }
}

pub fn digit<'a>() -> TestParser<char, impl Fn(&char) -> bool> {
    test::<char, _>(|c| c.is_digit(10), "a digit".to_string())
}

pub fn digits<'a>() -> MultiTestParser<char, impl Fn(&char) -> bool> {
    multi_test::<char, _>(|c| c.is_digit(10), "multiple digits".to_string())
}

#[derive(Clone)]
pub struct UIntParser;

impl<const ERROR: bool> RawTestParser<char, ERROR> for UIntParser {
    type TOut = u64;

    fn parse(&self, input: &[char], position: &mut ParsingPosition) -> ParseResult<u64> {
        let mut index = position.index();
        let mut num = 0_u64;

        while index < input.len() {
            if let Some(x) = input[index].to_digit(10) {
                num = 10_u64 * num + x as u64;

                index += 1;
            } else {
                break;
            }
        }

        if index == position.index() {
            let error = if ERROR {
                Some(vec!["a digit".to_string()])
            } else {
                None
            };
            return position.fail_offset(0, error);
        }
        position.succeed_at(index.into(), num)
    }
}

pub fn uint() -> UIntParser {
    UIntParser
}

#[derive(Clone)]
pub struct UFloatParser;

impl<const ERROR: bool> RawTestParser<char, ERROR> for UFloatParser {
    type TOut = f64;

    fn parse(&self, input: &[char], position: &mut ParsingPosition) -> ParseResult<f64> {
        let mut index = position.index();
        let mut num = 0_f64;

        while index < input.len() {
            if let Some(x) = input[index].to_digit(10) {
                num = 10_f64 * num + x as f64;

                index += 1;
            } else {
                break;
            }
        }

        if index == position.index() {
            let error = if ERROR {
                Some(vec!["a digit".to_string()])
            } else {
                None
            };
            return position.fail_offset(0, error);
        }

        let comma_index = index;

        if index >= input.len() || input[index] != '.' {
            return position.succeed_at(index.into(), num);
        }
        index += 1;

        while index < input.len() {
            if let Some(x) = input[index].to_digit(10) {
                let pos = index - comma_index;
                num = num + (x as f64) * 10_f64.powi(-(pos as i32));

                index += 1;
            } else {
                break;
            }
        }

        position.succeed_at(index.into(), num)
    }
}

pub fn float() -> UFloatParser {
    UFloatParser
}

pub fn letter() -> TestParser<char, impl Fn(&char) -> bool> {
    test::<char, _>(|c| c.is_ascii_alphabetic(), "a digit".to_string())
}

pub fn letters() -> MultiTestParser<char, impl Fn(&char) -> bool> {
    multi_test::<char, _>(|c| c.is_ascii_alphabetic(), "multiple digits".to_string())
}

#[derive(Clone)]
pub struct WhiteSpaceParser<const OPTIONAL: bool>;

impl<const ERROR: bool> RawTestParser<char, ERROR> for WhiteSpaceParser<false> {
    type TOut = ();

    fn parse(&self, input: &[char], position: &mut ParsingPosition) -> ParseResult<()> {
        let mut index = position.index();
        while index < input.len() && input[index].is_whitespace() {
            index += 1;
        }

        if position.index() == index {
            let error = if ERROR {
                Some(vec!["whitespace".to_string()])
            } else {
                None
            };
            position.fail_offset(0, error)
        } else {
            position.succeed_at(index.into(), ())
        }
    }
}

impl<const ERROR: bool> RawTestParser<char, ERROR> for WhiteSpaceParser<true> {
    type TOut = ();

    fn parse(&self, input: &[char], position: &mut ParsingPosition) -> ParseResult<()> {
        let mut index = position.index();
        while index < input.len() && input[index].is_whitespace() {
            index += 1;
        }

        position.succeed_at(index.into(), ())
    }
}

pub fn whitespace() -> WhiteSpaceParser<false> {
    WhiteSpaceParser::<false>
}

pub fn optional_whitespace() -> WhiteSpaceParser<true> {
    WhiteSpaceParser::<true>
}
