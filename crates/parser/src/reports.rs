use diagnostics::utils::Color;
use diagnostics::{Label, Report, ReportKind};
use lexer::token::{Token, TokenKind};

use crate::ParserError;

pub(crate) fn didnt_expect<'a>(
    got: &Token<'a>,
    expected: &'static [TokenKind<'static>],
) -> ParserError<'a> {
    let expected = expected
        .iter()
        .map(|kind| format!("{}", kind.as_ref()))
        .collect::<Vec<String>>()
        .join(", ");

    let report_message = format!(
        "Expected to find '[{}]' but instead got '[{}]'.",
        expected,
        got.kind.as_ref()
    );
    let label_message = format!("Expected '[{}]' instead of this character.", expected);

    let report = Report::new(report_message, 0001, ReportKind::Error)
        .message_colors(&[Color::Red, Color::Blue])
        .add_label(
            Label::new(got.span.clone())
                .message(label_message)
                .message_colors(&[Color::Red])
                .color(Color::Blue)
                .build(),
        )
        .build();

    ParserError::Diagnostic(report)
}

pub(crate) fn unexpected_eof<'a>(expected: &'static [TokenKind<'static>]) -> ParserError<'a> {
    let expected = expected
        .iter()
        .map(|kind| format!("{}", kind.as_ref()))
        .collect::<Vec<String>>()
        .join(", ");

    let report_message = format!(
        "Expected to find '[{}]' but came to the end of the file.",
        expected
    );

    let report = Report::new(report_message, 0002, ReportKind::Error)
        .message_colors(&[Color::Red])
        .build();

    ParserError::Diagnostic(report)
}
