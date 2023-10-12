#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Rodeo;

use diagnostics::{
    positional::LabelSpan,
    report::{LabelBuilder, Report, ReportBuilder, Reportable, Serverity},
};

pub(crate) type Result<T> = std::result::Result<T, ParserError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Unexpected {
    got: String,
    span: LabelSpan,
    expected: String,
}

impl Unexpected {
    pub fn new(got: String, span: LabelSpan, expected: impl Into<String>) -> Self {
        Self {
            got,
            span,
            expected: expected.into(),
        }
    }
}

impl From<Unexpected> for ParserError {
    fn from(value: Unexpected) -> Self {
        Self::Unexpected(value)
    }
}

impl Reportable for Unexpected {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message = format!(
            "Expected to find '[{}]' but instead got '[{}]'.",
            self.expected, self.got,
        );
        let label_message = format!("Expected '[{}]' instead of this token.", self.expected);

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
pub struct UnexpectedEOF {
    expected: String,
}

impl UnexpectedEOF {
    pub fn new(expected: impl Into<String>) -> Self {
        Self {
            expected: expected.into(),
        }
    }
}

impl From<UnexpectedEOF> for ParserError {
    fn from(value: UnexpectedEOF) -> Self {
        Self::UnexpectedEOF(value)
    }
}

impl Reportable for UnexpectedEOF {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message = format!(
            "Expected to find '[{}]' but came to the end of the file.",
            self.expected
        );

        ReportBuilder::default()
            .message(report_message)
            .code(2)
            .serverity(Serverity::Error)
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ParserError {
    Unexpected(Unexpected),
    UnexpectedEOF(UnexpectedEOF),
    InternalError(InternalError),
}

impl Reportable for ParserError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::UnexpectedEOF(error) => error.into_report(interner),
            Self::Unexpected(error) => error.into_report(interner),
            Self::InternalError(error) => error.into_report(interner),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct EndOfFile;

impl From<EndOfFile> for ParserError {
    fn from(value: EndOfFile) -> Self {
        Self::InternalError(InternalError::EndOfFile(value))
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
