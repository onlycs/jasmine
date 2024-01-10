mod parser;
mod prelude;

use crate::prelude::*;

pub fn parser(input: &str) -> Program {
    let input = input.parse().unwrap();

    parser::parser(input).into()
}
