// ---------------
// String parser
// ---------------

use crate::{parser::RawParser, ParseResult, ParsingContext};

#[derive(Clone)]
pub struct StringParser {
    string: String,
}

impl StringParser {
    pub fn new(string: String) -> StringParser {
        StringParser { string }
    }
}

impl RawParser<String> for StringParser {
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
