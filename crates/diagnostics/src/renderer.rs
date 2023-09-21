#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serdebug")]
use serdebug::SerDebug;

use std::collections::HashMap;

use termcolor::WriteColor;

use crate::{file::Files, report::Reportable};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Renderer<'a, Writer: WriteColor> {
    writer: Writer,
    files: &'a Files,
}

impl<'a, Writer: WriteColor> Renderer<'a, Writer> {
    pub fn new(files: &'a Files, writer: Writer) -> Self {
        Renderer { files, writer }
    }

    pub fn render<R: Reportable>(&mut self, report: R) {
        let mut report = report.into_report();
        for label in report.labels.iter_mut() {
            label.gather_data(self.files);
        }

        write!(
            self.writer,
            "{}[{}{:03}]",
            report.serverity.as_str(),
            report.serverity.prefix(),
            report.code,
        )
        .unwrap();

        writeln!(self.writer, ": {}", report.message).unwrap();

        let biggest_number = report
            .labels
            .iter()
            .map(|label| label.line_span.unwrap())
            .max_by(|first, second| first.end.cmp(&second.end))
            .map(|span| span.end.to_string().len())
            .unwrap();

        let mut files = HashMap::new();

        for label in report.labels.iter() {
            if label.multiline.unwrap() {
                panic!("Multiline not supported yet.");
            }

            files.entry(label.file).or_insert(vec![]).push(label);
        }

        for (file_id, labels) in files.iter() {
            let file = self.files.get(*file_id).unwrap();

            writeln!(
                self.writer,
                " {:width$} | {}",
                " ",
                file.path,
                width = biggest_number
            )
            .unwrap();
            writeln!(self.writer, " {:width$} |", " ", width = biggest_number).unwrap();

            for label in labels.iter() {
                let label = *label;

                let source_span = file.lines.get(label.line_span.unwrap().start).unwrap();
                let source = file.slice(source_span).unwrap();

                write!(
                    self.writer,
                    " {:width$} | ",
                    label.line_span.unwrap().start,
                    width = biggest_number
                )
                .unwrap();
                writeln!(self.writer, "{}", source).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use termcolor::{ColorChoice, StandardStream};

    use crate::{
        file::Files,
        renderer::Renderer,
        report::{LabelBuilder, ReportBuilder, Serverity},
    };

    #[test]
    fn render() {
        let stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut files = Files::new();

        let test_file = files.add("test.ark", "Hello World!\nWhat is\nup?\nGreeting!");

        let report = ReportBuilder::default()
            .message("This is just a note on how awesome you are")
            .code(0)
            .serverity(Serverity::Note)
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(0..4)
                    .message("This is a greeting.")
                    .build()
                    .unwrap(),
            )
            .note("Just wanted to say hi!")
            .note("Also good luck!")
            .build()
            .unwrap();

        let mut renderer = Renderer::new(&files, stdout);
        renderer.render(report);
    }
}
