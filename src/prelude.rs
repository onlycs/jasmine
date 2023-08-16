pub use crate::parser::Rule;
pub use anyhow::*;
pub use pest::iterators::Pair;

pub trait Parse: Sized {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self>;
}

pub trait ParseMany: Sized {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>>;
}
