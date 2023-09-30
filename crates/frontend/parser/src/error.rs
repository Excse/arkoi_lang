#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::report::{LabelBuilder, Labelable, Report, ReportBuilder, Reportable, Serverity};

pub(crate) type Result<T> = std::result::Result<T, ParserError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct DidntExpect {
    got: Labelable<String>,
    expected: String,
}

impl DidntExpect {
    pub fn error(got: impl Into<Labelable<String>>, expected: impl Into<String>) -> ParserError {
        ParserError::new(ErrorKind::DidntExpect(DidntExpect {
            got: got.into(),
            expected: expected.into(),
        }))
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
            ErrorKind::UnexpectedEOF(error) => unexpected_eof(error),
            ErrorKind::DidntExpect(error) => didnt_expect(error),
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

fn didnt_expect(args: DidntExpect) -> Report {
    let report_message = format!(
        "Expected to find '[{}]' but instead got '[{}]'.",
        args.expected, args.got.content,
    );
    let label_message = format!("Expected '[{}]' instead of this token.", args.expected);

    ReportBuilder::default()
        .message(report_message)
        .code(1)
        .serverity(Serverity::Error)
        // .message_colors(&[Color::Red, Color::Blue])
        .label(
            LabelBuilder::default()
                .span(args.got.span)
                .message(label_message)
                .file(args.got.file_id)
                // .message_colors(&[Color::Red])
                // .color(Color::Blue)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

fn unexpected_eof(args: UnexpectedEOF) -> Report {
    let report_message = format!(
        "Expected to find '[{}]' but came to the end of the file.",
        args.expected
    );

    ReportBuilder::default()
        .message(report_message)
        .code(2)
        .serverity(Serverity::Error)
        // .message_colors(&[Color::Red])
        .build()
        .unwrap()
}
