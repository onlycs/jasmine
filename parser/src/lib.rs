#![feature(
    error_generic_member_access,
    let_chains,
    type_alias_impl_trait,
    trait_alias,
    stmt_expr_attributes,
    if_let_guard,
    impl_trait_in_assoc_type,
    iter_collect_into
)]

use crate::prelude::*;

mod iter;
mod parsers;
mod prelude;

pub mod errors;

pub fn parse(input: &str) -> Result<UncheckedProgram, FullParserError> {
    Ok(parsers::parse(input.parse()?)?)
}
