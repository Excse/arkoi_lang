#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::report::Report;

pub type Result<T> = std::result::Result<T, LexerError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum LexerError {
    Diagnostic(Report),
    EndOfFile,
}
