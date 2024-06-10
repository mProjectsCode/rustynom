use crate::{
    atomic_parsers::{CharParser, CustomParser, SuccessParser},
    combinator_parsers::AndParser2,
    parser::{Parser, ParserCombinator},
    ParseResult, ParsingPosition,
};

pub fn position() -> impl Parser<ParsingPosition> {
    CustomParser::new(|context| ParseResult::Success(context.position.clone()))
}

pub fn any() -> impl Parser<char> {
    CustomParser::new(|context| {
        if context.at_eof() {
            context.fail_offset(0, vec!["any character".to_string()])
        } else {
            context.succeed_offset(1, context.input[context.position.index])
        }
    })
}

pub fn remaining() -> impl Parser<String> {
    CustomParser::new(|context| {
        let remaining = context.input[context.position.index..]
            .iter()
            .collect::<String>();
        context.succeed_offset(remaining.len(), remaining)
    })
}

pub fn char_test<TFn: Fn(char) -> bool + Clone>(
    test_fn: TFn,
    error_str: String,
) -> impl Parser<char> {
    CustomParser::new(move |context| {
        if context.at_eof() {
            context.fail_offset(0, vec![error_str.to_string()])
        } else {
            let c = context.input[context.position.index];
            if test_fn(c) {
                context.succeed_offset(1, c)
            } else {
                context.fail_offset(0, vec![error_str.to_string()])
            }
        }
    })
}

pub fn multi_char_test<TFn: Fn(char) -> bool + Clone>(
    test_fn: TFn,
    error_str: String,
) -> impl Parser<String> {
    CustomParser::new(move |context| {
        let mut index = context.position.index;
        while index < context.input.len() && test_fn(context.input[index]) {
            index += 1;
        }

        let chars = context.input[context.position.index..index]
            .iter()
            .collect::<String>();

        if chars.is_empty() {
            context.fail_offset(0, vec![error_str.to_string()])
        } else {
            context.succeed_at(index, chars)
        }
    })
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
    CustomParser::new(move |context| {
        let mut index = context.position.index;
        let mut acc = initial.clone();

        while index < context.input.len() {
            if let Some(x) = test_fn(context.input[index]) {
                reduce_fn(&mut acc, x, index - context.position.index);

                index += 1;
            } else {
                break;
            }
        }

        if index == context.position.index {
            context.fail_offset(0, vec![error_str.to_string()])
        } else {
            context.succeed_at(index, (acc, index - context.position.index))
        }
    })
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
    AndParser2::new(uint(), CharParser::new('.').then(digits()).optional()).map(
        |(before_decimal, after_decimal)| {
            let mut result = before_decimal as f32;
            if let Some(x) = after_decimal {
                x.chars().enumerate().for_each(|(i, c)| {
                    let digit = c.to_digit(10).unwrap();
                    let index = i as i32;

                    result = result + (digit as f32) * 10_f32.powi(-(index + 1));
                });
            }

            result
        },
    )
}

pub fn letter() -> impl Parser<char> {
    char_test(|c| c.is_ascii_alphabetic(), "a digit".to_string())
}

pub fn letters() -> impl Parser<String> {
    multi_char_test(|c| c.is_ascii_alphabetic(), "multiple digits".to_string())
}

pub fn whitespace() -> impl Parser<String> {
    multi_char_test(|c| c.is_whitespace(), "whitespace".to_string())
}

pub fn optional_whitespace() -> impl Parser<String> {
    whitespace().or_same(SuccessParser::new("".to_string()))
}
