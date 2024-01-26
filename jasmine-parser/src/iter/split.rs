use crate::prelude::*;

pub struct Split<'a> {
    iter: &'a mut TokenIterator,
    pat: String,
}

pub trait IntoSplit {
    fn split<'a>(&'a mut self, split: impl Into<String>) -> Split<'a>;
}

impl<'a> Iterator for Split<'a> {
    type Item = Vec<TokenTree>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.collect_while(|n| n.to_string() == self.pat);

        match self.iter.collect_while(|n| n.to_string() != self.pat) {
            v if v.len() == 0 => None,
            v => Some(v),
        }
    }
}

impl IntoSplit for TokenIterator {
    fn split<'a>(&'a mut self, split: impl Into<String>) -> Split<'a> {
        Split {
            iter: self,
            pat: split.into(),
        }
    }
}
