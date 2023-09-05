use crate::LexerError;
use diagnostics::utils::Color;
use diagnostics::{Label, Report, ReportKind, Span};

pub(crate) fn didnt_expect<'a, S>(
    got: char,
    span: Span<'a>,
    expected: String,
) -> Result<S, LexerError<'a>> {
    let report_message = format!(
        "Expected to find '[{}]' but instead got '[{}]'.",
        expected, got,
    );
    let label_message = format!("Expected '[{}]' instead of this character.", expected);

    let report = Report::new(report_message, 0001, ReportKind::Error)
        .message_colors(&[Color::Red, Color::Blue])
        .add_label(
            Label::new(span)
                .message(label_message)
                .message_colors(&[Color::Red])
                .color(Color::Blue)
                .build(),
        )
        .build();

    Err(LexerError::Diagnostic(report))
}
