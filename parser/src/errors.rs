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
    #[error("cannot find type  in this scope. Types must be fully defined before use")]
    UnresolvedType,

    #[error("the name `{0}` is used multiple times. `{0}` must be defined only once")]
    DuplicateType(String),

    #[error("the number of generic types provided do not match")]
    NonMatchingGenerics,

    #[error("the generic type provided does not satisfy the trait bounds")]
    TraitBoundsNotMet,

    #[error("tried to constrain using a type that is not a trait")]
    NonTraitConstraint,
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
