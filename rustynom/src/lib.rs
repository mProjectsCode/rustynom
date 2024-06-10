pub mod atomic_parsers;
pub mod combinator_parsers;
pub mod parser;
pub mod transformation_parsers;
pub mod utility_parsers;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsingPosition {
    pub index: usize,
    pub line: u32,
    pub column: u32,
}

impl ParsingPosition {
    pub fn new(index: usize, line: u32, column: u32) -> ParsingPosition {
        ParsingPosition {
            index,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingRange {
    pub from: ParsingPosition,
    pub to: ParsingPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseFailure {
    pub furthest: ParsingPosition,
    pub expected: Vec<String>,
}

impl ParseFailure {
    pub fn new(furthest: ParsingPosition, expected: Vec<String>) -> ParseFailure {
        ParseFailure { furthest, expected }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseResult<T> {
    Success(T),
    Failure(ParseFailure),
}

impl<T> ParseResult<T> {
    pub fn is_success(&self) -> bool {
        match self {
            ParseResult::Success(_) => true,
            ParseResult::Failure(_) => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    pub fn unwrap_success(self) -> T {
        match self {
            ParseResult::Success(t) => t,
            ParseResult::Failure(_) => panic!("Called unwrap_success on a ParseResult::Failure"),
        }
    }

    pub fn unwrap_failure(self) -> ParseFailure {
        match self {
            ParseResult::Failure(f) => f,
            ParseResult::Success(_) => panic!("Called unwrap_failure on a ParseResult::Success"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingContext<'a> {
    input: &'a Vec<char>,
    pub position: ParsingPosition,
}

impl<'a> ParsingContext<'a> {
    pub fn new(input: &'a Vec<char>, position: ParsingPosition) -> ParsingContext<'a> {
        ParsingContext {
            input,
            position,
        }
    }

    pub fn move_to_position(&mut self, position: ParsingPosition) {
        self.position = position;
    }

    pub fn at_eof(&self) -> bool {
        self.position.index >= self.input.len()
    }

    fn advance_to(&mut self, index: usize) {
        if index < self.position.index {
            panic!("Cannot advance to a position before the current position");
        }
        if index == self.position.index {
            return;
        }

        for i in self.position.index..index {
            if self.input[i] == '\n' {
                self.position.line += 1;
                self.position.column = 0;
            } else {
                self.position.column += 1;
            }
        }

        self.position.index = index;
    }

    pub fn succeed_offset<T>(&mut self, offset: usize, result: T) -> ParseResult<T> {
        self.succeed_at(self.position.index + offset, result)
    }

    pub fn fail_offset<T>(&mut self, offset: usize, expected: Vec<String>) -> ParseResult<T> {
        self.fail_at(self.position.index + offset, expected)
    }

    pub fn succeed_at<T>(&mut self, index: usize, result: T) -> ParseResult<T> {
        self.advance_to(index);
        ParseResult::Success(result)
    }

    pub fn fail_at<T>(&mut self, index: usize, expected: Vec<String>) -> ParseResult<T> {
        self.advance_to(index);
        ParseResult::Failure(ParseFailure::new(self.position.clone(), expected))
    }

    pub fn merge_failures(&self, mut a: ParseFailure, b: ParseFailure) -> ParseFailure {
        match a.furthest.cmp(&b.furthest) {
            std::cmp::Ordering::Less => b,
            std::cmp::Ordering::Greater => a,
            std::cmp::Ordering::Equal => {
                a.expected.extend(b.expected);
                a
            }
        }
    }
}
