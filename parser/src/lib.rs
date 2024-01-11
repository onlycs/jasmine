#![feature(error_generic_member_access)]

use crate::prelude::*;

mod parsers;
mod prelude;

pub mod errors;

pub fn parse(input: &str) -> Result<Program, JasmineParserError> {
    Ok(parsers::parse(input.parse()?)?)
}
