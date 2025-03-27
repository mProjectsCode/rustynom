use crate::{
    ParseResult, ParsingPosition,
    atomic_parsers::EofParser,
    combinator_parsers::{
        AndParser2, OrParser2, SameOrParser2, SeparatedByParser, SkipParser, SurroundParser,
    },
    parsable::Parsable,
    transformation_parsers::{ManyNonEmptyParser, ManyParser, MapParser, OptionalParser},
};

pub type ParserOut<P, TIn, const ERROR: bool> = <P as RawTestParser<TIn, ERROR>>::TOut;

pub trait ParserCombinator<TIn: Parsable, const ERROR: bool>: RawTestParser<TIn, ERROR>
where
    Self: Sized + Clone,
    <Self as RawTestParser<TIn, ERROR>>::TOut: Clone,
{
    fn and<P2: RawTestParser<TIn, ERROR> + Clone>(
        self,
        other: P2,
    ) -> AndParser2<ERROR, TIn, Self, P2> {
        AndParser2::new(self, other)
    }

    fn or<P2: RawTestParser<TIn, ERROR> + Clone>(
        self,
        other: P2,
    ) -> OrParser2<ERROR, TIn, Self, P2> {
        OrParser2::new(self, other)
    }

    fn or_same<P2: RawTestParser<TIn, ERROR, TOut = ParserOut<Self, TIn, ERROR>> + Clone>(
        self,
        other: P2,
    ) -> SameOrParser2<ERROR, TIn, Self, P2> {
        SameOrParser2::new(self, other)
    }

    fn map<TFn: (Fn(ParserOut<Self, TIn, ERROR>) -> TOut) + Clone, TOut: Clone>(
        self,
        f: TFn,
    ) -> MapParser<TIn, TOut, Self, TFn, ERROR> {
        MapParser::new(self, f)
    }

    fn skip<P2: RawTestParser<TIn, ERROR, TOut = ()> + Clone>(
        self,
        other: P2,
    ) -> SkipParser<TIn, Self, P2, ERROR> {
        SkipParser::new(self, other)
    }

    fn then_eof(self) -> SkipParser<TIn, Self, EofParser, ERROR> {
        self.skip(EofParser::new())
    }

    fn many(self) -> ManyParser<TIn, Self>
    where
        Self: RawTestParser<TIn, false>,
    {
        ManyParser::new(self)
    }

    fn many_non_empty(self) -> ManyNonEmptyParser<TIn, Self, ERROR> {
        ManyNonEmptyParser::new(self)
    }

    fn separated_by<P2: RawTestParser<TIn, ERROR, TOut = ()> + Clone>(
        self,
        separator: P2,
    ) -> SeparatedByParser<TIn, Self, P2, ERROR> {
        SeparatedByParser::new(self, separator)
    }

    fn optional(self) -> OptionalParser<TIn, Self>
    where
        Self: RawTestParser<TIn, false>,
    {
        OptionalParser::new(self)
    }

    fn trim<P2: RawTestParser<TIn, ERROR, TOut = ()> + Clone>(
        self,
        other: P2,
    ) -> SurroundParser<TIn, Self, P2, P2, ERROR> {
        SurroundParser::new(self, other.clone(), other)
    }

    fn surround<
        LParser: RawTestParser<TIn, ERROR, TOut = ()>,
        RParser: RawTestParser<TIn, ERROR, TOut = ()>,
    >(
        self,
        l: LParser,
        r: RParser,
    ) -> SurroundParser<TIn, Self, LParser, RParser, ERROR> {
        SurroundParser::new(self, l, r)
    }
}

impl<TIn: Parsable, TP: RawTestParser<TIn, ERROR> + Clone, const ERROR: bool>
    ParserCombinator<TIn, ERROR> for TP
where
    <Self as RawTestParser<TIn, ERROR>>::TOut: Clone,
{
}

pub trait RawTestParser<TIn: Parsable, const ERROR: bool> {
    type TOut;
    fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<Self::TOut>;
}

pub struct ParserWrapper<TIn: Parsable, TOut: Clone, const ERROR: bool = false> {
    parser: Box<dyn RawTestParser<TIn, ERROR, TOut = TOut>>,
    __phantom1: std::marker::PhantomData<TIn>,
}

impl<TIn: Parsable, TOut: Clone, const ERROR: bool> ParserWrapper<TIn, TOut, ERROR> {
    pub fn new(parser: Box<dyn RawTestParser<TIn, ERROR, TOut = TOut>>) -> Self {
        Self {
            parser,
            __phantom1: std::marker::PhantomData,
        }
    }

    pub fn from_parser<P: RawTestParser<TIn, ERROR, TOut = TOut> + 'static>(parser: P) -> Self {
        Self {
            parser: Box::new(parser),
            __phantom1: std::marker::PhantomData,
        }
    }

    // pub fn parse(&self, data: TIn::List) -> ParseResult<TOut> {
    //     let list = TIn::list_to_owned_slice(data);
    //     self.parser.parse(&list, &mut ParsingPosition::default())
    // }

    pub fn parse_slice<'s>(&self, slice: &'s [TIn::T]) -> ParseResult<TOut> {
        self.parser.parse(&slice, &mut ParsingPosition::default())
    }
}

// impl<TOut: Clone, P: Parser<'static, char, TOut>> ParserWrapper<'static, char, TOut, P> {
//     pub fn parse_str(&self, s: &str) -> ParseResult<TOut> {
//         let chars = s.chars().collect::<Vec<_>>();
//         self.parse_slice(&chars)
//     }
// }
