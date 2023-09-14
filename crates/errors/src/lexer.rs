use diagnostics::{
    file::{FileID, Files},
    positional::Spannable,
    report::{LabelBuilder, Report, ReportBuilder, Serverity},
};

pub fn didnt_expect(
    files: &Files,
    file_id: FileID,
    got: Spannable<char>,
    expected: String,
) -> Report {
    let report_message = format!(
        "Expected to find '[{}]' but instead got '[{}]'.",
        expected, got.content,
    );
    let label_message = format!("Expected '[{}]' instead of this character.", expected);

    ReportBuilder::default()
        .message(report_message)
        .code(1)
        .serverity(Serverity::Error)
        .label(
            LabelBuilder::default()
                .message(label_message)
                .file(file_id)
                .span(got.span)
                .build(files)
                .unwrap(),
        )
        .build()
        .unwrap()
}
