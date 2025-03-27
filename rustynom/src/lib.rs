use parsable::Parsable;

pub mod atomic_parsers;
pub mod combinator_parsers;
pub mod parsable;
pub mod parser;
pub mod transformation_parsers;
pub mod utility_parsers;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParsingPosition(usize);

impl ParsingPosition {
    pub fn new(index: usize) -> Self {
        ParsingPosition(index)
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }

    pub fn advance_by(&mut self, offset: usize) {
        self.0 += offset;
    }

    pub fn advance_to(&mut self, position: ParsingPosition) {
        self.0 = position.0;
    }

    pub fn advance_to_index(&mut self, index: usize) {
        self.0 = index;
    }

    pub fn slice<'a, T>(&self, slice: &'a [T]) -> &'a [T] {
        &slice[self.index()..]
    }

    pub fn slice_to<'a, T>(&self, slice: &'a [T], index: usize) -> &'a [T] {
        &slice[self.index()..index]
    }

    pub fn slice_with_length<'a, T>(&self, slice: &'a [T], length: usize) -> &'a [T] {
        &slice[self.index()..self.index() + length]
    }

    pub fn current<'a, T>(&self, slice: &'a [T]) -> &'a T {
        &slice[self.index()]
    }

    pub fn current_eq<T: Eq>(&self, slice: &[T], c: &T) -> bool {
        if self.at_eof(slice) {
            false
        } else {
            &slice[self.index()] == c
        }
    }

    pub fn test_current<'a, T>(&self, slice: &[T], test_fn: impl Fn(&T) -> bool) -> bool {
        if self.at_eof(slice) {
            false
        } else {
            test_fn(self.current(slice))
        }
    }

    pub fn current_eq_slice<T: Eq>(&self, slice: &[T], other: &[T]) -> bool {
        if self.at_eof(slice) {
            false
        } else {
            let current = self.slice(slice);
            current.len() >= other.len() && current.starts_with(other)
        }
    }

    pub fn at_eof<T>(&self, slice: &[T]) -> bool {
        self.index() >= slice.len()
    }

    pub fn succeed_offset<T>(&mut self, offset: usize, result: T) -> ParseResult<T> {
        self.advance_by(offset);
        ParseResult::Success(result)
    }

    pub fn fail_offset<T>(
        &mut self,
        offset: usize,
        expected: Option<Vec<String>>,
    ) -> ParseResult<T> {
        self.advance_by(offset);
        ParseResult::Failure(ParseFailure::new(self.clone(), expected))
    }

    pub fn succeed_at<T>(&mut self, position: ParsingPosition, result: T) -> ParseResult<T> {
        self.advance_to(position);
        ParseResult::Success(result)
    }

    pub fn fail_at<T>(
        &mut self,
        position: ParsingPosition,
        expected: Option<Vec<String>>,
    ) -> ParseResult<T> {
        self.advance_to(position);
        ParseResult::Failure(ParseFailure::new(self.clone(), expected))
    }

    pub fn merge_failures(&self, mut a: ParseFailure, b: ParseFailure) -> ParseFailure {
        match a.furthest.cmp(&b.furthest) {
            std::cmp::Ordering::Less => b,
            std::cmp::Ordering::Greater => a,
            std::cmp::Ordering::Equal => {
                match (&mut a.expected, b.expected) {
                    (Some(a), Some(b)) => a.extend(b),
                    (None, Some(b)) => a.expected = Some(b),
                    _ => {}
                }

                // a.expected.extend(b.expected);
                a
            }
        }
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
    pub expected: Option<Vec<String>>,
}

impl ParseFailure {
    pub fn new(furthest: ParsingPosition, expected: Option<Vec<String>>) -> ParseFailure {
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

#[macro_export]
macro_rules! parse_str {
    ($p:expr, $str:literal) => {{
        let chars = $str.chars().collect::<Vec<_>>();
        $p.parse_slice(&chars)
    }};
}
