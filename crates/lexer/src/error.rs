#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::report::{LabelBuilder, Labelable, Report, ReportBuilder, Reportable, Serverity};

pub type Result<T> = std::result::Result<T, LexerError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct DidntExpect {
    got: Labelable<char>,
    expected: String,
}

impl DidntExpect {
    pub fn error(got: Labelable<char>, expected: impl Into<String>) -> LexerError {
        LexerError::DidntExpect(DidntExpect {
            got,
            expected: expected.into(),
        })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum InternalError {
    UnexpectedEOF,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum LexerError {
    DidntExpect(DidntExpect),
    Internal(InternalError),
}

impl Reportable for LexerError {
    fn into_report(self) -> Report {
        match self {
            Self::DidntExpect(error) => didnt_expect(error),
            Self::Internal(error) => panic!("Error: {:?}", error),
        }
    }
}

fn didnt_expect(args: DidntExpect) -> Report {
    let report_message = format!(
        "Expected to find '[{}]' but instead got '[{}]'.",
        args.expected, args.got.content,
    );
    let label_message = format!("Expected '[{}]' instead of this character.", args.expected);

    ReportBuilder::default()
        .message(report_message)
        .code(1)
        .serverity(Serverity::Error)
        .label(
            LabelBuilder::default()
                .message(label_message)
                .file(args.got.file_id)
                .span(args.got.span)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}
