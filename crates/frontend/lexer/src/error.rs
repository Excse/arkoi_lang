#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::{
    positional::LabelSpan,
    report::{LabelBuilder, Report, ReportBuilder, Reportable, Serverity},
};
use lasso::Rodeo;

pub type Result<T> = std::result::Result<T, LexerError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct DidntExpect {
    got: char,
    span: LabelSpan,
    expected: String,
}

impl DidntExpect {
    pub fn error(got: char, span: LabelSpan, expected: impl Into<String>) -> LexerError {
        LexerError::DidntExpect(DidntExpect {
            got,
            span,
            expected: expected.into(),
        })
    }
}

impl Reportable for DidntExpect {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message = format!(
            "Expected to find '[{}]' but instead got '[{}]'.",
            self.expected, self.got,
        );
        let label_message = format!("Expected '[{}]' instead of this character.", self.expected);

        ReportBuilder::default()
            .message(report_message)
            .code(1)
            .serverity(Serverity::Error)
            .label(
                LabelBuilder::default()
                    .message(label_message)
                    .span(self.span)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum LexerError {
    DidntExpect(DidntExpect),
    InternalError(InternalError),
}

impl Reportable for LexerError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::DidntExpect(error) => error.into_report(interner),
            Self::InternalError(error) => error.into_report(interner),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct EndOfFile;

impl EndOfFile {
    pub fn error() -> LexerError {
        LexerError::InternalError(InternalError::EndOfFile(EndOfFile))
    }
}

impl Reportable for EndOfFile {
    fn into_report(self, _interner: &Rodeo) -> Report {
        ReportBuilder::default()
            .message("Unexpectedly reached the end of the file.")
            .code(2)
            .serverity(Serverity::Bug)
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum InternalError {
    EndOfFile(EndOfFile),
}

impl Reportable for InternalError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::EndOfFile(error) => error.into_report(interner),
        }
    }
}
