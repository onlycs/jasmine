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
    let ast = parser::parse(PathBuf::new().join(args.input))?;

    if args.skip_rewrite {
        let str_fmt = format!("{:?}", ast);

        if args.save {
            let mut file =
                File::create(PathBuf::new().join(format!("{}.jasmine_ast", args.program_name)))?;
            writeln!(&mut file, "{str_fmt}")?;
        } else {
            println!("{str_fmt}");
        }
    } else {
        let file_str = rewrite::rewrite(ast, &args.program_name);

        if args.save {
            let mut file =
                File::create(PathBuf::new().join(format!("{}.java", args.program_name)))?;
            writeln!(&mut file, "{file_str}")?;
        } else {
            println!("{file_str}");
        }
    }

    Ok(())
}
