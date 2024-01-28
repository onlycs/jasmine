use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Syn: {error}")]
    Syn {
        #[from]
        error: syn::Error,
        backtrace: Backtrace,
    },

    #[error("IO: {error}")]
    IO {
        #[from]
        error: std::io::Error,
        backtrace: Backtrace,
    },

    #[error("Unresolved module: could not find file `{0}.jasmine` or `{0}/mod.jasmine`")]
    UnresolvedModule(String),

    #[error("Can only use shebang in the crate-level module")]
    UnexpectedShebang,
}
