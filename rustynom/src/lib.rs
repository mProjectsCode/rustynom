pub mod atomic_parsers;
pub mod combinator_parsers;
pub mod parser;
pub mod transformation_parsers;
pub mod utility_parsers;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsingPosition(usize);

impl ParsingPosition {
    pub fn new(index: usize) -> ParsingPosition {
        ParsingPosition(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn advance_by(&mut self, offset: usize) {
        self.0 += offset;
    }
}

impl Default for ParsingPosition {
    fn default() -> Self {
        ParsingPosition::new(0)
    }
}

impl From<usize> for ParsingPosition {
    fn from(index: usize) -> Self {
        ParsingPosition::new(index)
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
pub enum GenericParseResult<T, F> {
    Success(T),
    Failure(F),
}

impl<T, F> GenericParseResult<T, F> {
    pub fn is_success(&self) -> bool {
        match self {
            GenericParseResult::Success(_) => true,
            GenericParseResult::Failure(_) => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    pub fn unwrap_success(self) -> T {
        match self {
            GenericParseResult::Success(t) => t,
            GenericParseResult::Failure(_) => {
                panic!("Called unwrap_success on a GenericParseResult::Failure")
            }
        }
    }

    pub fn unwrap_failure(self) -> F {
        match self {
            GenericParseResult::Failure(f) => f,
            GenericParseResult::Success(_) => {
                panic!("Called unwrap_failure on a GenericParseResult::Success")
            }
        }
    }
}

pub type ParseResult<T> = GenericParseResult<T, ParseFailure>;

#[derive(Debug, PartialEq)]
pub struct ParsingContext<'a> {
    input: &'a [char],
    pub position: ParsingPosition,
}

impl<'a> Clone for ParsingContext<'a> {
    fn clone(&self) -> Self {
        Self {
            input: self.input,
            position: self.position.clone(),
        }
    }
}

impl<'a> ParsingContext<'a> {
    pub fn new(input: &'a [char], position: ParsingPosition) -> ParsingContext<'a> {
        ParsingContext { input, position }
    }

    pub fn slice_from_current(&self) -> &[char] {
        &self.input[self.position.index()..]
    }

    pub fn slice_from_current_to(&self, index: usize) -> &[char] {
        &self.input[self.position.index()..index]
    }

    pub fn current(&self) -> &char {
        &self.input[self.position.index()]
    }

    pub fn current_eq(&self, c: &char) -> bool {
        if self.at_eof() {
            false
        } else {
            &self.input[self.position.index()] == c
        }
    }

    pub fn test_current(&self, test_fn: impl Fn(char) -> bool) -> bool {
        if self.at_eof() {
            false
        } else {
            test_fn(self.input[self.position.index()])
        }
    }

    pub fn current_eq_slice(&self, slice: &[char]) -> bool {
        if self.at_eof() {
            false
        } else {
            let current = &self.input[self.position.index()..];
            current.len() >= slice.len() && current.starts_with(slice)
        }
    }

    pub fn move_to_position(&mut self, position: ParsingPosition) {
        self.position = position;
    }

    pub fn at_eof(&self) -> bool {
        self.position.index() >= self.input.len()
    }

    fn advance_to(&mut self, position: ParsingPosition) {
        self.position = position;
    }

    fn advance_by(&mut self, offset: usize) {
        self.position.advance_by(offset);
    }

    pub fn succeed_offset<T>(&mut self, offset: usize, result: T) -> ParseResult<T> {
        self.advance_by(offset);
        ParseResult::Success(result)
    }

    pub fn fail_offset<T>(&mut self, offset: usize, expected: Vec<String>) -> ParseResult<T> {
        self.advance_by(offset);
        ParseResult::Failure(ParseFailure::new(self.position.clone(), expected))
    }

    pub fn succeed_at<T>(&mut self, position: ParsingPosition, result: T) -> ParseResult<T> {
        self.advance_to(position);
        ParseResult::Success(result)
    }

    pub fn fail_at<T>(
        &mut self,
        position: ParsingPosition,
        expected: Vec<String>,
    ) -> ParseResult<T> {
        self.advance_to(position);
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
