use crate::{ParseResult, ParsingContext, parser::RawParser};

// ---------------
// String parser
// ---------------

#[derive(Clone)]
pub struct StringParser<const OUTPUT: bool = true> {
    string: String,
    chars: Vec<char>,
}

impl<const OUTPUT: bool> StringParser<OUTPUT> {
    pub fn new(string: String) -> StringParser<OUTPUT> {
        StringParser {
            string: string.clone(),
            chars: string.chars().collect(),
        }
    }
}

impl RawParser<String> for StringParser<true> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<String> {
        match context.slice_from_current().starts_with(&self.chars) {
            true => context.succeed_offset(self.chars.len(), self.string.clone()),
            false => context.fail_offset(0, vec![self.string.clone()]),
        }
    }
}

impl RawParser<()> for StringParser<false> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<()> {
        match context.slice_from_current().starts_with(&self.chars) {
            true => context.succeed_offset(self.chars.len(), ()),
            false => context.fail_offset(0, vec![self.string.clone()]),
        }
    }
}

// ---------------
// String parser with custom result
// ---------------

#[derive(Clone)]
pub struct StringMapParser<T: Clone> {
    result: T,
    string: String,
    chars: Vec<char>,
}

impl<T: Clone> StringMapParser<T> {
    pub fn new(string: String, result: T) -> StringMapParser<T> {
        StringMapParser {
            result,
            string: string.clone(),
            chars: string.chars().collect(),
        }
    }
}

impl<T: Clone> RawParser<T> for StringMapParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        match context.slice_from_current().starts_with(&self.chars) {
            true => context.succeed_offset(self.chars.len(), self.result.clone()),
            false => context.fail_offset(0, vec![self.string.clone()]),
        }
    }
}

// ---------------
// Char parser
// ---------------

#[derive(Clone)]
pub struct CharParser<const OUTPUT: bool = true> {
    c: char,
}

impl<const OUTPUT: bool> CharParser<OUTPUT> {
    pub fn new(c: char) -> CharParser<OUTPUT> {
        CharParser { c }
    }
}

impl RawParser<char> for CharParser<true> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<char> {
        if context.at_eof() {
            return context.fail_offset(0, vec![self.c.to_string()]);
        }

        if &self.c == context.current() {
            context.succeed_offset(1, self.c)
        } else {
            context.fail_offset(0, vec![self.c.to_string()])
        }
    }
}

impl RawParser<()> for CharParser<false> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<()> {
        if context.at_eof() {
            return context.fail_offset(0, vec![self.c.to_string()]);
        }

        if &self.c == context.current() {
            context.succeed_offset(1, ())
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
