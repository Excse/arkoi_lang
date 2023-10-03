#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::{
    positional::LabelSpan,
    report::{LabelBuilder, Report, ReportBuilder, Reportable, Serverity},
};

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
    fn into_report(self) -> Report {
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
                    .file(self.span.file_id)
                    .span(self.span.span)
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
    fn into_report(self) -> Report {
        match self {
            Self::DidntExpect(error) => error.into_report(),
            Self::InternalError(error) => panic!("Internal error: {:?}", error),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct UnexpectedEOF;

impl UnexpectedEOF {
    pub fn error() -> LexerError {
        LexerError::InternalError(InternalError::UnexpectedEOF(UnexpectedEOF))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum InternalError {
    UnexpectedEOF(UnexpectedEOF),
}
