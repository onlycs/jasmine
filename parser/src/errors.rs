use std::backtrace::Backtrace;

use proc_macro2::LexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JasmineParserError {
    #[error("Failed to parse program to tokenstream: {error}")]
    ParseTokenStream {
        #[from]
        error: LexError,
        backtrace: Backtrace,
    },

    #[error("Type error: {error}")]
    Type {
        #[from]
        error: TypeError,
        backtrace: Backtrace,
    },

    #[error("Incorrect next item: {error}")]
    Syntax {
        #[from]
        error: SyntaxError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("cannot find type `{0}` in this scope")]
    UnresolvedType(String),

    #[error("the name `{0}` is used multiple times. `{0}` must be defined only once")]
    DuplicateType(String),
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Expected {0}")]
    ExpectWithoutCheck(&'static str),

    #[error("Expected {0} such that {1}")]
    ExpectWithCheck(&'static str, &'static str),

    #[error("Invalid identifier: {0}")]
    InvalidIdent(String),

    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[error("Unexpected EOF")]
    UnexpectedEOF,
}
