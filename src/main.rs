use std::path::PathBuf;

mod parser;
mod prelude;
mod types;

fn main() {
    println!(
        "{:#?}",
        parser::parse(PathBuf::new().join("example.jasmine")).unwrap()
    );

    println!("Hello, world!");
}
