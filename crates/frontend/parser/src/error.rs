use lasso::Rodeo;
#[cfg(feature = "serialize")]
use serde::Serialize;

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
    pub fn error(got: String, span: LabelSpan, expected: impl Into<String>) -> ParserError {
        ParserError::new(ErrorKind::Unexpected(Unexpected {
            got,
            span,
            expected: expected.into(),
        }))
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
    pub fn error(expected: impl Into<String>) -> ParserError {
        ParserError::new(ErrorKind::UnexpectedEOF(UnexpectedEOF {
            expected: expected.into(),
        }))
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
pub enum ErrorKind {
    Unexpected(Unexpected),
    UnexpectedEOF(UnexpectedEOF),
    InternalError(InternalError),
}

impl Reportable for ErrorKind {
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

impl EndOfFile {
    pub fn error() -> ParserError {
        ParserError::new(ErrorKind::InternalError(InternalError::EndOfFile(
            EndOfFile,
        )))
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct ParserError {
    pub(crate) kind: ErrorKind,
    pub(crate) wrong_start: bool,
}

impl Reportable for ParserError {
    fn into_report(self, interner: &Rodeo) -> Report {
        self.kind.into_report(interner)
    }
}

impl ParserError {
    fn new(kind: ErrorKind) -> Self {
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
