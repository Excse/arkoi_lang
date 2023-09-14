use diagnostics::{
    file::{Files, FileID},
    positional::Spannable,
    report::{LabelBuilder, Report, ReportBuilder, Serverity},
};

pub fn didnt_expect(
    files: &Files,
    file_id: FileID,
    got: Spannable<impl Into<String>>,
    expected: impl Into<String>,
) -> Report {
    let expected = expected.into();

    let Spannable { content, span } = got;
    let content = content.into();

    let report_message = format!(
        "Expected to find '[{}]' but instead got '[{}]'.",
        expected, content,
    );
    let label_message = format!("Expected '[{}]' instead of this token.", expected);

    ReportBuilder::default()
        .message(report_message)
        .code(1)
        .serverity(Serverity::Error)
        // .message_colors(&[Color::Red, Color::Blue])
        .label(
            LabelBuilder::default()
                .span(span)
                .message(label_message)
                .file(file_id)
                // .message_colors(&[Color::Red])
                // .color(Color::Blue)
                .build(files)
                .unwrap(),
        )
        .build()
        .unwrap()
}

pub fn unexpected_eof(expected: impl Into<String>) -> Report {
    let expected = expected.into();

    let report_message = format!(
        "Expected to find '[{}]' but came to the end of the file.",
        expected
    );

    ReportBuilder::default()
        .message(report_message)
        .code(2)
        .serverity(Serverity::Error)
        // .message_colors(&[Color::Red])
        .build()
        .unwrap()
}
