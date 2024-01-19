use std::iter::{self, FromFn};

use crate::prelude::*;

/// LAZY: does not advance the iterator on it's own
pub trait CopyWhile: Iterator {
    fn copy_while<'a>(
        &'a mut self,
        predicate: impl Fn(&Self::Item) -> bool + Copy + 'a,
    ) -> Peekable<FromFn<impl FnMut() -> Option<Self::Item> + 'a>>;
}

pub trait CollectWhile: Iterator {
    fn collect_while(
        &mut self,
        predicate: impl Fn(&Self::Item) -> bool + Copy,
    ) -> Peekable<<Vec<Self::Item> as IntoIterator>::IntoIter>;
}

impl<I> CopyWhile for Peekable<I>
where
    I: Iterator,
{
    fn copy_while<'a>(
        &'a mut self,
        predicate: impl Fn(&Self::Item) -> bool + Copy + 'a,
    ) -> Peekable<FromFn<impl FnMut() -> Option<Self::Item>>> {
        iter::from_fn(move || self.next_if(predicate)).peekable()
    }
}

impl<I> CollectWhile for Peekable<I>
where
    I: Iterator,
{
    fn collect_while(
        &mut self,
        predicate: impl Fn(&Self::Item) -> bool + Copy,
    ) -> Peekable<<Vec<Self::Item> as IntoIterator>::IntoIter> {
        let mut collect = vec![];

        while let Some(next) = self.next_if(predicate) {
            collect.push(next);
        }

        collect.into_iter().peekable()
    }
}
