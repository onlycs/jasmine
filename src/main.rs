use std::path::PathBuf;

mod jasmine;
mod parser;
mod prelude;

fn main() {
    println!(
        "{:#?}",
        parser::parse(PathBuf::new().join("example.jasmine")).unwrap()
    );
}
