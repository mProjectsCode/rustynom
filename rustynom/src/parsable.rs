use std::{fmt::Debug, marker::PhantomData};

use itertools::Itertools;

pub trait Parsable: Clone {
    type T: Eq + Clone;
    type List: Eq + Clone;

    fn t_to_string(t: &Self::T) -> String;

    fn list_to_owned_slice(list: Self::List) -> Box<[Self::T]>;
    fn list_ref_to_owned_slice(list: &Self::List) -> Box<[Self::T]>;
    fn list_to_string(list: &Self::List) -> String;

    fn slice_to_list(slice: &[Self::T]) -> Self::List;
}

impl Parsable for char {
    type T = char;
    type List = String;

    fn t_to_string(t: &Self::T) -> String {
        t.to_string()
    }

    fn list_to_owned_slice(list: Self::List) -> Box<[Self::T]> {
        list.chars().collect::<Vec<_>>().into_boxed_slice()
    }

    fn list_ref_to_owned_slice(list: &Self::List) -> Box<[Self::T]> {
        list.chars().collect::<Vec<_>>().into_boxed_slice()
    }

    fn list_to_string(list: &Self::List) -> String {
        list.clone()
    }

    fn slice_to_list(slice: &[Self::T]) -> Self::List {
        slice.iter().collect()
    }
}

pub struct ParsableSlice<T> {
    __phantom: PhantomData<T>,
}

impl<T> Clone for ParsableSlice<T> {
    fn clone(&self) -> Self {
        ParsableSlice {
            __phantom: PhantomData,
        }
    }
}

impl<T: Eq + Clone + Debug> Parsable for ParsableSlice<T> {
    type T = T;
    type List = Box<[T]>;

    fn t_to_string(t: &Self::T) -> String {
        format!("{:?}", t)
    }

    fn list_to_owned_slice(list: Self::List) -> Box<[Self::T]> {
        list
    }

    fn list_ref_to_owned_slice(list: &Self::List) -> Box<[Self::T]> {
        list.clone()
    }

    fn list_to_string(list: &Self::List) -> String {
        format!("{:?}", list.iter().map(|t| Self::t_to_string(t)).join(", "))
    }

    fn slice_to_list(slice: &[Self::T]) -> Self::List {
        Box::from(slice)
    }
}
