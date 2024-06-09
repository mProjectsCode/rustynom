// ---------------
// Recursive parser
// ---------------

use std::{cell::RefCell, rc::Rc};

use crate::{parser::RawParser, ParseResult, ParsingContext};

pub struct RecRefParser<T> {
    parser_ref: Rc<RefCell<Option<Box<dyn RawParser<T>>>>>,
    __phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for RecRefParser<T> {
    fn clone(&self) -> Self {
        RecRefParser {
            parser_ref: self.parser_ref.clone(),
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<T> RecRefParser<T> {
    pub fn new() -> RecRefParser<T> {
        RecRefParser {
            parser_ref: Rc::new(RefCell::new(None)),
            __phantom: std::marker::PhantomData,
        }
    }

    pub fn set(&self, parser: Box<dyn RawParser<T>>) {
        self.parser_ref.borrow_mut().replace(parser);
    }
}

impl<T> RawParser<T> for RecRefParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        self.parser_ref
            .borrow()
            .as_ref()
            .expect("RecRefParser has no parser set")
            .parse(context)
    }
}

#[derive(Clone)]
pub struct RecParser<T> {
    parser: RecRefParser<T>,
}

impl<T> RecParser<T> {
    pub fn new<TParser: RawParser<T> + 'static>(
        decl: impl FnOnce(RecRefParser<T>) -> TParser,
    ) -> RecParser<T> {
        let rec_ref = RecRefParser::new();

        let parser = decl(rec_ref.clone());

        rec_ref.set(Box::from(parser));

        RecParser { parser: rec_ref }
    }
}

impl<T: Clone> RawParser<T> for RecParser<T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
        self.parser.parse(context)
    }
}

// infinite type size

// pub struct RecRefParser<TParser: Parser<T>, T> {
//     parser_ref: Rc<RefCell<Option<Box<TParser>>>>,
//     __phantom: std::marker::PhantomData<T>,
// }

// impl<TParser: Parser<T>, T> Clone for RecRefParser<TParser, T> {
//     fn clone(&self) -> Self {
//         RecRefParser {
//             parser_ref: self.parser_ref.clone(),
//             __phantom: std::marker::PhantomData,
//         }
//     }
// }

// impl<TParser: Parser<T>, T> RecRefParser<TParser, T> {
//     pub fn new() -> RecRefParser<TParser, T> {
//         RecRefParser {
//             parser_ref: Rc::new(RefCell::new(None)),
//             __phantom: std::marker::PhantomData,
//         }
//     }

//     pub fn set(&self, parser: TParser) {
//         self.parser_ref.borrow_mut().replace(Box::from(parser));
//     }
// }

// impl <TParser: Parser<T>, T> Parser<T> for RecRefParser<TParser, T> {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
//         self.parser_ref.borrow().as_ref().expect("RecRefParser has no parser set").parse(context)
//     }
// }

// #[derive(Clone)]
// pub struct RecParser<TParser: Parser<T>, T> {
//     parser: RecRefParser<TParser, T>,
//     __phantom: std::marker::PhantomData<T>,
// }

// impl<TParser: Parser<T>, T> RecParser<TParser, T> {
//     pub fn new(decl: impl FnOnce(RecRefParser<TParser, T>) -> TParser) -> RecParser<TParser, T> {
//         let rec_ref = RecRefParser::new();

//         let parser = decl(rec_ref.clone());

//         rec_ref.set(parser);

//         RecParser {
//             parser: rec_ref,
//             __phantom: std::marker::PhantomData,
//         }
//     }
// }

// impl<TParser: Parser<T>, T: Clone> Parser<T> for RecParser<TParser, T> {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
//         self.parser.parse(context)
//     }
// }

// ---------------
// Map parser
// ---------------

// pub struct MapParser<'a, TParser, T, U>
// where
//     TParser: Parser<T>,
// {
//     parser: MaybeOwned<'a, TParser>,
//     f: Box<dyn Fn(T) -> U>,
// }

// impl<'a, TParser, T, U> MapParser<'a, TParser, T, U>
// where
//     TParser: Parser<T>,
// {
//     pub fn new(
//         parser: MaybeOwned<'a, TParser>,
//         f: impl Fn(T) -> U + 'static,
//     ) -> MapParser<'a, TParser, T, U> {
//         MapParser {
//             parser,
//             f: Box::new(f),
//         }
//     }
// }

// impl<'a, TParser, T, U> Parser<U> for MapParser<'a, TParser, T, U>
// where
//     TParser: Parser<T>,
// {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<U> {
//         match self.parser.as_ref().parse(context) {
//             ParseResult::Success(a) => ParseResult::Success((self.f)(a)),
//             ParseResult::Failure(f) => ParseResult::Failure(f),
//         }
//     }
// }

// pub enum MaybeOwned<'a, T: 'a> {
//     Owned(T),
//     Borrowed(&'a T),
// }

// impl<'a, T> MaybeOwned<'a, T> {
//     pub fn as_ref(&self) -> &T {
//         match self {
//             MaybeOwned::Borrowed(t) => t,
//             MaybeOwned::Owned(t) => &t,
//         }
//     }
// }

#[derive(Clone)]
pub struct MapParser<TParser: RawParser<T>, T: Clone, TFn: (Fn(T) -> U) + Clone, U: Clone> {
    parser: TParser,
    f: TFn,
    __phantom1: std::marker::PhantomData<T>,
    __phantom2: std::marker::PhantomData<U>,
}

impl<TParser: RawParser<T>, T: Clone, TFn: (Fn(T) -> U) + Clone, U: Clone>
    MapParser<TParser, T, TFn, U>
{
    pub fn new(parser: TParser, f: TFn) -> MapParser<TParser, T, TFn, U> {
        MapParser {
            parser,
            f,
            __phantom1: std::marker::PhantomData,
            __phantom2: std::marker::PhantomData,
        }
    }
}

impl<TParser: RawParser<T>, T: Clone, TFn: (Fn(T) -> U) + Clone, U: Clone> RawParser<U>
    for MapParser<TParser, T, TFn, U>
{
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<U> {
        match self.parser.parse(context) {
            ParseResult::Success(a) => ParseResult::Success((self.f)(a)),
            ParseResult::Failure(f) => ParseResult::Failure(f),
        }
    }
}

// ---------------
// Many parser
// ---------------

#[derive(Clone)]
pub struct ManyParser<TParser, T>
where
    TParser: RawParser<T>,
{
    parser: TParser,
    __phantom: std::marker::PhantomData<T>,
}

impl<TParser, T> ManyParser<TParser, T>
where
    TParser: RawParser<T>,
{
    pub fn new(parser: TParser) -> ManyParser<TParser, T> {
        ManyParser {
            parser,
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<TParser: RawParser<T>, T: Clone> RawParser<Vec<T>> for ManyParser<TParser, T> {
    fn parse(&self, context: &mut ParsingContext) -> ParseResult<Vec<T>> {
        let mut result = Vec::new();
        while let ParseResult::Success(t) = self.parser.parse(context) {
            result.push(t);
        }

        ParseResult::Success(result)
    }
}

// ---------------
// Glued parser
// ---------------

// A parser that is created from a closure that returns a parser.
// pub struct DynamicDispatchParser<T>
// {
//     parser: Box<dyn Parser<T>>,
//     __phantom: std::marker::PhantomData<T>,
// }

// impl<T> DynamicDispatchParser<T>
// {
//     pub fn new<TParser: Parser<T> + 'static>(parser: TParser) -> DynamicDispatchParser<T>{
//         let parser = Box::new(parser);
//         DynamicDispatchParser {
//             parser,
//             __phantom: std::marker::PhantomData,
//         }
//     }
// }

// impl<T> Parser<T> for DynamicDispatchParser<T>
// {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {
//         self.parser.parse(context)
//     }
// }
