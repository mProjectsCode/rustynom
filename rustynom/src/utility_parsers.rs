use crate::{
    ParseResult, ParsingContext, ParsingPosition,
    atomic_parsers::CustomParser,
    combinator_parsers::AndParser2,
    parser::{Parser, ParserCombinator, RawParser},
};

pub fn position() -> impl Parser<ParsingPosition> {
    CustomParser::new(|context| ParseResult::Success(context.position.clone()))
}

#[derive(Clone)]
pub struct AnyParser;

impl RawParser<char> for AnyParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<char> {
        if context.at_eof() {
            context.fail_offset(0, vec!["any character".to_string()])
        } else {
            context.succeed_offset(1, *context.current())
        }
    }
}

pub fn any() -> impl Parser<char> {
    AnyParser
}

#[derive(Clone)]
pub struct RemainingParser;

impl RawParser<String> for RemainingParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<String> {
        let remaining = context.slice_from_current().iter().collect::<String>();
        context.succeed_offset(remaining.len(), remaining)
    }
}

pub fn remaining() -> impl Parser<String> {
    RemainingParser
}

#[derive(Clone)]
pub struct CharTestParser<TFn: Fn(char) -> bool> {
    test_fn: TFn,
    error_str: String,
}

impl<TFn: Fn(char) -> bool> RawParser<char> for CharTestParser<TFn> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<char> {
        if context.test_current(&self.test_fn) {
            context.succeed_offset(1, *context.current())
        } else {
            context.fail_offset(0, vec![self.error_str.clone()])
        }
    }
}

pub fn char_test<TFn: Fn(char) -> bool + Clone>(
    test_fn: TFn,
    error_str: String,
) -> impl Parser<char> {
    CharTestParser { test_fn, error_str }
}

#[derive(Clone)]
pub struct MultiCharTestParser<TFn: Fn(char) -> bool> {
    test_fn: TFn,
    error_str: String,
}

impl<TFn: Fn(char) -> bool> RawParser<String> for MultiCharTestParser<TFn> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<String> {
        let mut index = context.position.index();
        while index < context.input.len() && (self.test_fn)(context.input[index]) {
            index += 1;
        }

        let chars = context
            .slice_from_current_to(index)
            .iter()
            .collect::<String>();

        if chars.is_empty() {
            context.fail_offset(0, vec![self.error_str.clone()])
        } else {
            context.succeed_at(index.into(), chars)
        }
    }
}

pub fn multi_char_test<TFn: Fn(char) -> bool + Clone>(
    test_fn: TFn,
    error_str: String,
) -> impl Parser<String> {
    MultiCharTestParser { test_fn, error_str }
}

#[derive(Clone)]
pub struct MultiCharTestWithReduceParser<
    T: Clone,
    TAcc: Clone,
    TFn: Fn(char) -> Option<T>,
    TRed: Fn(&mut TAcc, T, usize),
> {
    test_fn: TFn,
    reduce_fn: TRed,
    initial: TAcc,
    error_str: String,
}

impl<T: Clone, TAcc: Clone, TFn: Fn(char) -> Option<T>, TRed: Fn(&mut TAcc, T, usize)>
    RawParser<(TAcc, usize)> for MultiCharTestWithReduceParser<T, TAcc, TFn, TRed>
{
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<(TAcc, usize)> {
        let mut index = context.position.index();
        let mut acc = self.initial.clone();

        while index < context.input.len() {
            if let Some(x) = (self.test_fn)(context.input[index]) {
                (self.reduce_fn)(&mut acc, x, index - context.position.index());

                index += 1;
            } else {
                break;
            }
        }

        if index == context.position.index() {
            context.fail_offset(0, vec![self.error_str.clone()])
        } else {
            context.succeed_at(index.into(), (acc, index - context.position.index()))
        }
    }
}

pub fn multi_char_test_with_reduce<
    T: Clone,
    TAcc: Clone,
    TFn: Fn(char) -> Option<T> + Clone,
    TRed: Fn(&mut TAcc, T, usize) + Clone,
>(
    test_fn: TFn,
    reduce_fn: TRed,
    initial: TAcc,
    error_str: String,
) -> impl Parser<(TAcc, usize)> {
    MultiCharTestWithReduceParser {
        test_fn,
        reduce_fn,
        initial,
        error_str,
    }
}

pub fn digit() -> impl Parser<char> {
    char_test(|c| c.is_digit(10), "a digit".to_string())
}

pub fn digits() -> impl Parser<String> {
    multi_char_test(|c| c.is_digit(10), "multiple digits".to_string())
}

pub fn uint() -> impl Parser<u32> {
    multi_char_test_with_reduce(
        |c| c.to_digit(10),
        |acc, d, _| *acc = *acc * 10 + d,
        0,
        "multiple digits".to_string(),
    )
    .map(|(acc, _)| acc)
}

pub fn float() -> impl Parser<f32> {
    let after_decimal = CustomParser::new(move |context| {
        if !context.current_eq(&'.') {
            return context.succeed_offset(0, 0_f32);
        }

        let mut index = 1;
        let mut num = 0_f32;

        while context.position.index() + index < context.input.len() {
            if let Some(x) = context.input[context.position.index() + index].to_digit(10) {
                num = num + (x as f32) * 10_f32.powi(-(index as i32));
                index += 1;
            } else {
                break;
            }
        }

        if index == 1 {
            context.succeed_offset(0, 0_f32)
        } else {
            context.succeed_offset(index, num)
        }
    });

    AndParser2::new(uint(), after_decimal)
        .map(|(before_decimal, after_decimal)| before_decimal as f32 + after_decimal)
}

pub fn letter() -> impl Parser<char> {
    char_test(|c| c.is_ascii_alphabetic(), "a digit".to_string())
}

pub fn letters() -> impl Parser<String> {
    multi_char_test(|c| c.is_ascii_alphabetic(), "multiple digits".to_string())
}

#[derive(Clone)]
pub struct WhiteSpaceParser<const OPTIONAL: bool>;

impl RawParser<()> for WhiteSpaceParser<false> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<()> {
        let mut index = context.position.index();
        while index < context.input.len() && context.input[index].is_whitespace() {
            index += 1;
        }

        if context.position.index() == index {
            context.fail_offset(0, vec!["whitespace".to_string()])
        } else {
            context.succeed_at(index.into(), ())
        }
    }
}

impl RawParser<()> for WhiteSpaceParser<true> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<()> {
        let mut index = context.position.index();
        while index < context.input.len() && context.input[index].is_whitespace() {
            index += 1;
        }

        context.succeed_at(index.into(), ())
    }
}

pub fn whitespace() -> impl Parser<()> {
    WhiteSpaceParser::<false>
}

pub fn optional_whitespace() -> impl Parser<()> {
    WhiteSpaceParser::<true>
}
