use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Jasmine is a Rust-like programming language that compiles to Java. It is intended to be a \"Javan't\" to my Computer Science class"
)]
pub struct JasmineCli {
    #[arg(
        short = 'r',
        long,
        help = "Just print (or save) the AST",
        long_help = "Print or save the Abstract Syntax Tree. Use -f in addition to pretty-print"
    )]
    pub skip_rewrite: bool,

    #[arg(
        trailing_var_arg = true,
        help = "Input file",
        long_help = "The input file to compile.",
        required = true
    )]
    pub input: Vec<String>,
}
