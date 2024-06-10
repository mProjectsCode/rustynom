// ---------------
// String parser
// ---------------

use crate::{parser::RawParser, ParseResult, ParsingContext};

#[derive(Clone)]
pub struct StringParser {
    string: String,
    chars: Vec<char>
}

impl StringParser {
    pub fn new(string: String) -> StringParser {
        StringParser { string: string.clone(), chars: string.chars().collect() }
    }
}

impl RawParser<String> for StringParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<String> {
        for (i, c) in self.chars.iter().enumerate() {
            if context.position.index + i >= context.input.len() {
                return context.fail_at(context.position.index, vec![self.string.clone()]);
            }

            if *c != context.input[context.position.index + i] {
                return context.fail_at(context.position.index, vec![self.string.clone()]);
            }
        }

        context.succeed_offset(self.chars.len(), self.string.clone())
    }
}

// ---------------
// Char parser
// ---------------

#[derive(Clone)]
pub struct CharParser {
    c: char,
}

impl CharParser {
    pub fn new(c: char) -> CharParser {
        CharParser { c }
    }
}

impl RawParser<char> for CharParser {
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

impl RawParser<()> for EofParser {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<()> {
        if context.at_eof() {
            context.succeed_offset(0, ())
        } else {
            context.fail_offset(0, vec!["EOF".to_string()])
        }
    }
}

// ---------------
// Success parser
// ---------------

#[derive(Clone)]
pub struct SuccessParser<T: Clone> {
    result: T,
}

impl<T: Clone> SuccessParser<T> {
    pub fn new(result: T) -> SuccessParser<T> {
        SuccessParser { result }
    }
}

impl<T: Clone> RawParser<T> for SuccessParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        context.succeed_offset(0, self.result.clone())
    }
}

// ---------------
// Custom parser
// ---------------

#[derive(Clone)]
pub struct CustomParser<T: Clone, TFn: Fn(&mut ParsingContext) -> ParseResult<T>> {
    f: TFn,
    __phantom: std::marker::PhantomData<T>,
}

impl<T: Clone, TFn: Fn(&mut ParsingContext) -> ParseResult<T>> CustomParser<T, TFn> {
    pub fn new(f: TFn) -> CustomParser<T, TFn> {
        CustomParser {
            f,
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Clone, TFn: Fn(&mut ParsingContext) -> ParseResult<T>> RawParser<T>
    for CustomParser<T, TFn>
{
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        (self.f)(context)
    }
}
