use crate::prelude::*;

pub struct Split<'a, T, I, P>
where
    I: Iterator<Item = T>,
    P: Fn(&T) -> bool,
{
    iter: &'a mut Peekable<I>,
    predicate: P,
}

pub trait IntoSplit<It>
where
    It: Iterator,
{
    fn split<'a, P>(&'a mut self, predicate: P) -> Split<'a, It::Item, It, P>
    where
        P: Fn(&It::Item) -> bool;
}

impl<'a, T, I, P> Iterator for Split<'a, T, I, P>
where
    I: Iterator<Item = T>,
    P: Fn(&T) -> bool,
{
    type Item = Peekable<<Vec<T> as IntoIterator>::IntoIter>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_if(|a| (self.predicate)(a));

        if self.iter.peek().is_none() {
            return None;
        }

        // if the self.iter is not empty, then an empty collect_while indicates two consecutive
        // elements that satisfy the predicate
        Some(self.iter.collect_while(|a| !(self.predicate)(a)))
    }
}

impl<It> IntoSplit<It> for Peekable<It>
where
    It: Iterator,
{
    fn split<'a, P>(&'a mut self, predicate: P) -> Split<'a, It::Item, It, P>
    where
        P: Fn(&It::Item) -> bool,
    {
        Split {
            iter: self,
            predicate,
        }
    }
}
