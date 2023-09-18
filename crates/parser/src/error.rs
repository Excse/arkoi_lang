#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::positional::Spannable;

pub(crate) type Result<T> = std::result::Result<T, ParserError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ErrorKind {
    DidntExpect(Spannable<String>, String),
    UnexpectedEOF(String),
    EndOfFile,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct ParserError {
    pub(crate) kind: ErrorKind,
    pub(crate) wrong_start: bool,
}

impl ParserError {
    pub fn new(kind: ErrorKind) -> Self {
        ParserError {
            kind,
            wrong_start: false,
        }
    }

    pub fn wrong_start(mut self, wrong_start: bool) -> Self {
        self.wrong_start = wrong_start;
        self
    }
}

