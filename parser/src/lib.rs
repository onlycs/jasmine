#![feature(
    error_generic_member_access,
    let_chains,
    type_alias_impl_trait,
    trait_alias,
    stmt_expr_attributes
)]

use crate::prelude::*;

mod parsers;
mod prelude;

pub mod errors;

pub fn parse(input: &str) -> Result<UncheckedProgram, FullParserError> {
    Ok(parsers::parse(input.parse()?)?)
}
