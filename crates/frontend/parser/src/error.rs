#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::{
    positional::LabelSpan,
    report::{LabelBuilder, Report, ReportBuilder, Reportable, Serverity},
};

pub(crate) type Result<T> = std::result::Result<T, ParserError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct DidntExpect {
    got: String,
    span: LabelSpan,
    expected: String,
}

impl DidntExpect {
    pub fn error(got: String, span: LabelSpan, expected: impl Into<String>) -> ParserError {
        ParserError::new(ErrorKind::DidntExpect(DidntExpect {
            got,
            span,
            expected: expected.into(),
        }))
    }
}

impl Reportable for DidntExpect {
    fn into_report(self) -> Report {
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
                    .span(self.span.span)
                    .message(label_message)
                    .file(self.span.file_id)
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
    fn into_report(self) -> Report {
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
    DidntExpect(DidntExpect),
    UnexpectedEOF(UnexpectedEOF),
    InternalError(InternalError),
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum InternalError {
    EndOfFile(EndOfFile),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct ParserError {
    pub(crate) kind: ErrorKind,
    pub(crate) wrong_start: bool,
}

impl Reportable for ParserError {
    fn into_report(self) -> Report {
        match self.kind {
            ErrorKind::UnexpectedEOF(error) => error.into_report(),
            ErrorKind::DidntExpect(error) => error.into_report(),
            ErrorKind::InternalError(error) => panic!("Error: {:?}", error),
        }
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
