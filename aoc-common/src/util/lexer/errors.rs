use std::num::{IntErrorKind, ParseIntError};

use super::Lexer;

#[derive(Debug, PartialEq, Eq)]
pub enum Lexeme {
    EOL,
    Literal(String),
    Number,
    Symbol,
    Symbols(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedEOL { expected: Lexeme },
    UnexpectedSymbol { expected: Lexeme, at: usize },
    ParseIntError { at: usize, kind: IntErrorKind },
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    str: String,
    kind: ErrorKind,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    #[must_use]
    pub fn eol(lexer: &Lexer<'_>, expected: Lexeme) -> Self {
        Self {
            str: lexer.s.to_owned(),
            kind: ErrorKind::UnexpectedEOL { expected },
        }
    }

    #[must_use]
    pub fn symbol(lexer: &Lexer<'_>, expected: Lexeme) -> Self {
        Self {
            str: lexer.s.to_owned(),
            kind: ErrorKind::UnexpectedSymbol { expected, at: lexer.pos },
        }
    }

    #[must_use]
    pub fn parse_error(lexer: &Lexer<'_>, error: &ParseIntError) -> Self {
        Self {
            str: lexer.s.to_owned(),
            kind: ErrorKind::ParseIntError {
                at: lexer.pos,
                kind: error.kind().clone(),
            },
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
