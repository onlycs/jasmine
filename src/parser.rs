use std::{fs::File, io::Read, path::PathBuf};

use crate::prelude::*;
use crate::types::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "jasmine.pest"]
struct JasmineParser;

pub fn parse(file: PathBuf) -> Result<Vec<JasmineProgramComponent>> {
    let mut file_str = String::new();

    File::open(file)?.read_to_string(&mut file_str)?;

    let pest_parsed = JasmineParser::parse(Rule::program, &file_str)
        .map(|mut n| n.next().context("Failed to parse"));

    match pest_parsed {
        Result::Ok(Result::Ok(pest_parsed)) => {
            let components =
                JasmineProgramComponent::parse_many(pest_parsed).context("Could not parse")?;

            Ok(components)
        }
        Err(e) => {
            println!("{:?}", e);
            Err(e).context("Failed to parse")
        }
        _ => unreachable!(),
    }
}
