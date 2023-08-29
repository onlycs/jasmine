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
        short = 'n',
        long,
        value_name = "name",
        default_value = "JasmineProgram",
        help = "Java class name",
        long_help = "The name of the program. Will also set filename if save is enabled"
    )]
    pub program_name: String,

    #[arg(
        short = 'r',
        long,
        help = "Just print (or save) the AST",
        long_help = "Print or save the Abstract Syntax Tree. Use -f in addition to pretty-print"
    )]
    pub skip_rewrite: bool,

    #[arg(
        short = 'i',
        long,
        help = "Input file",
        long_help = "The input file to compile.",
        default_value = "program.jasmine"
    )]
    pub input: String,

    #[arg(short, long, help = "Save the file")]
    pub save: bool,
}
