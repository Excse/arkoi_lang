#[cfg(feature = "serialize")]
use serde::Serialize;

use std::fmt::Display;

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
#[derive(Debug)]
pub enum LabelBuilderError {
    UninitializedField(&'static str),
    FileNotFound(FileID),
    InvalidLineSpan(Span),
}

impl From<UninitializedFieldError> for LabelBuilderError {
    fn from(value: UninitializedFieldError) -> Self {
        Self::UninitializedField(value.field_name())
    }
}

impl Display for LabelBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLineSpan(span) => {
                write!(f, "Couldn't find a valid line span for this '{:?}'", span)
            }
            Self::FileNotFound(file_id) => {
                write!(f, "Couldn't find a file with this id '{}'", file_id)
            }
            Self::UninitializedField(field) => write!(f, "Required field '{}' not set", field),
        }
    }
}
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Builder, Clone, PartialEq)]
#[builder(build_fn(private, name = "build_label", error = "LabelBuilderError"))]
pub struct Label {
    pub(crate) file: FileID,
    #[builder(setter(into))]
    pub(crate) span: Span,
    #[builder(setter(into, strip_option))]
    pub(crate) message: Option<String>,

    #[builder(setter(skip))]
    pub(crate) line_span: Span,
    #[builder(setter(skip))]
    pub(crate) multiline: bool,
}

impl LabelBuilder {
    pub fn build(&self, files: &Files) -> Result<Label, LabelBuilderError> {
        let mut label = self.build_label()?;

        let span = self.span.as_ref().unwrap();
        let file_id = self.file.unwrap();

        let file = files
            .get(file_id)
            .ok_or(LabelBuilderError::FileNotFound(file_id))?;
        label.line_span = file
            .find_line_span(span)
            .ok_or(LabelBuilderError::InvalidLineSpan(*span))?;
        label.multiline = label.line_span.start != label.line_span.end;

        Ok(label)
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
                    .build(&files)
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(3..6)
                    .message("")
                    .build(&files)
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

        let report = ReportBuilder::default()
            .message("This is just a note on how awesome you are")
            .code(0)
            .serverity(Serverity::Note)
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(0..4)
                    .message("This is a greeting.")
                    .build(&files)
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .file(test_file)
                    .span(5..8)
                    .message("This is another greeting.")
                    .build(&files)
                    .unwrap(),
            )
            .note("Just wanted to say hi!")
            .note("Also good luck!")
            .build()
            .unwrap();
    }
}
