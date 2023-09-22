extern crate anyhow;
extern crate clap;
extern crate itertools;
extern crate pest;
extern crate pest_derive;

mod args;
mod jasmine;
mod parser;
mod prelude;
mod rewrite;

use crate::prelude::*;
use clap::Parser;
use std::{fs::File, io::Write, path::PathBuf, process::Command};

fn main() -> Result<()> {
    let args = args::JasmineCli::parse();
    let ast = parser::parse(PathBuf::new().join(&args.input.first().unwrap()))?;

    let input = &args
        .input
        .first()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string();

    let input_first_upper = input.chars().next().unwrap().to_uppercase().to_string() + &input[1..];

    let mut f = File::create(PathBuf::new().join(format!("{}.java", &input_first_upper)))?;
    let r = rewrite::rewrite(ast, &input_first_upper);

    writeln!(f, "{}", r)?;

    Ok(())
}
