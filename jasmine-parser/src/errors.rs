use std::{backtrace::Backtrace, fmt};

use proc_macro2::{LexError, TokenTree};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct FullParserError {
    pub error: ParserError,
    pub next_item: Option<TokenTree>,
}

impl fmt::Display for FullParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "error: {}", self.error)?;
        writeln!(f, "next item: {:?}", self.next_item)?;

        Ok(())
    }
}

impl From<LexError> for FullParserError {
    fn from(value: LexError) -> Self {
        Self {
            error: ParserError::from(value),
            next_item: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to parse program to tokenstream: {error}")]
    ParseTokenStream {
        #[from]
        error: LexError,
        backtrace: Backtrace,
    },

    #[error("Syntax error: {error}")]
    Syntax {
        #[from]
        error: SyntaxError,
        backtrace: Backtrace,
    },

    #[error("Type error: {error}")]
    Type {
        #[from]
        error: TypeError,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Expected {item}. Parsing on {next:?}")]
    ExpectWithoutCheck { item: &'static str, next: TokenTree },

    #[error("Expected {item} such that {check}. Parsing on {next:?}")]
    ExpectWithCheck {
        item: &'static str,
        check: &'static str,
        next: TokenTree,
    },

    #[error("Invalid identifier: {ident}. Parsing on {next:?}")]
    InvalidIdent { ident: String, next: TokenTree },

    #[error("Unexpected token: {0}")]
    UnexpectedToken(TokenTree),

    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Duplicate type: {0}")]
    DuplicateType(String),

    #[error("Duplicate function: {0}")]
    DuplicateFunction(String),

    #[error("Duplicate enum variant: {0}")]
    DuplicateVariant(String),

    #[error("Duplicate struct field or argument: {0}")]
    DupilicateKV(String),
}
