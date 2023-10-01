#[cfg(feature = "serialize")]
use serde::Serialize;

use std::fmt::Display;
use std::ops::Deref;

use derive_builder::UninitializedFieldError;

use crate::{
    file::{FileID, Files},
    positional::Span,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum Serverity {
    Help,
    Note,
    Warning,
    Error,
    Bug,
}

impl Serverity {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Help => "help",
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Bug => "bug",
        }
    }

    pub fn prefix(&self) -> &'static str {
        match *self {
            Self::Help => "H",
            Self::Note => "N",
            Self::Warning => "W",
            Self::Error => "E",
            Self::Bug => "B",
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ReportBuilderError {
    UninitializedField(&'static str),
    OverlappingLabels(Vec<(Span, Span)>),
}

impl From<UninitializedFieldError> for ReportBuilderError {
    fn from(value: UninitializedFieldError) -> Self {
        Self::UninitializedField(value.field_name())
    }
}

impl Display for ReportBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OverlappingLabels(labels) => {
                write!(f, "There are overlapping labels '{:?}'", labels)
            }
            Self::UninitializedField(field) => write!(f, "Required field '{}' not set", field),
        }
    }
}

pub trait Reportable {
    fn into_report(self) -> Report;
}

impl Reportable for Report {
    fn into_report(self) -> Report {
        self
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Builder)]
#[builder(build_fn(private, name = "build_report", error = "ReportBuilderError"))]
pub struct Report {
    #[builder(setter(into))]
    pub(crate) message: String,
    pub(crate) code: usize,
    pub(crate) serverity: Serverity,
    #[builder(default, setter(each(name = "label")))]
    pub(crate) labels: Vec<Label>,
    #[builder(default, setter(each(name = "note", into)))]
    pub(crate) notes: Vec<String>,
}

impl ReportBuilder {
    pub fn build(&self) -> Result<Report, ReportBuilderError> {
        self.check_overlap()?;
        self.build_report()
    }

    fn check_overlap(&self) -> Result<(), ReportBuilderError> {
        let labels = match self.labels {
            Some(ref labels) => labels,
            None => return Ok(()),
        };

        let mut overlapping = Vec::new();

        for (index, label) in labels.iter().enumerate() {
            for other in labels.iter().skip(index + 1) {
                if label.span.intersect(&other.span) {
                    overlapping.push((label.span, other.span));
                }
            }
        }

        if !overlapping.is_empty() {
            return Err(ReportBuilderError::OverlappingLabels(overlapping));
        }

        Ok(())
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Labelable<C> {
    content: C,
    pub span: Span,
    pub file_id: FileID,
}

impl<C> Deref for Labelable<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<C> Labelable<C> {
    pub fn new(content: C, span: Span, file_id: FileID) -> Self {
        Labelable {
            content,
            span,
            file_id,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Label {
    pub(crate) file: FileID,
    #[builder(setter(into))]
    pub(crate) span: Span,
    #[builder(setter(into, strip_option))]
    pub(crate) message: Option<String>,

    #[builder(setter(skip))]
    pub(crate) line_span: Option<Span>,
    #[builder(setter(skip))]
    pub(crate) multiline: Option<bool>,
}

impl Label {
    pub fn gather_data(&mut self, files: &Files) {
        let file = files.get(self.file).expect("Couldn't find the file.");

        let line_span = file.find_line_span(&self.span).expect("Invalid line span.");
        let multiline = line_span.start != line_span.end;

        self.line_span = Some(line_span);
        self.multiline = Some(multiline);
    }
}

#[cfg(test)]
mod test {
    use crate::file::Files;

    use super::*;

    #[test]
    #[should_panic]
    fn overlapping() {
        let mut files = Files::new();

        let test_file = files.add("test.ark", "This is a test.");

        let report = ReportBuilder::default()
            .message("")
            .code(0)
            .serverity(Serverity::Note)
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(0..4)
                    .message("")
                    .build()
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(3..6)
                    .message("")
                    .build()
                    .unwrap(),
            )
            .note("Just wanted to say hi!")
            .note("Also good luck!")
            .build()
            .unwrap();

        println!("{:#?}", report);
    }

    #[test]
    fn report() {
        let mut files = Files::new();

        let test_file = files.add("test.ark", "Hello World!");

        ReportBuilder::default()
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
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(5..8)
                    .message("This is another greeting.")
                    .build()
                    .unwrap(),
            )
            .note("Just wanted to say hi!")
            .note("Also good luck!")
            .build()
            .unwrap();
    }
}
