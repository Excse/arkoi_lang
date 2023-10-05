use lasso::Rodeo;
#[cfg(feature = "serialize")]
use serde::Serialize;

use std::fmt::Display;

use derive_builder::UninitializedFieldError;

use crate::{
    file::Files,
    positional::{LabelSpan, Span},
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
    fn into_report(self, interner: &Rodeo) -> Report;
}

impl Reportable for Report {
    fn into_report(self, _interner: &Rodeo) -> Report {
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
                if label.span.span.intersect(&other.span.span) {
                    overlapping.push((label.span.span, other.span.span));
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
#[derive(Debug, Builder, Clone, PartialEq)]
pub struct Label {
    pub(crate) span: LabelSpan,
    #[builder(setter(into, strip_option))]
    pub(crate) message: Option<String>,

    #[builder(setter(skip))]
    pub(crate) line_span: Option<Span>,
    #[builder(setter(skip))]
    pub(crate) multiline: Option<bool>,
}

impl Label {
    pub fn gather_data(&mut self, files: &Files) {
        let file = files
            .get(self.span.file_id)
            .expect("Couldn't find the file.");

        let line_span = file
            .find_line_span(&self.span.span)
            .expect("Invalid line span.");
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
                    .span(LabelSpan::new(0..4, test_file))
                    .message("")
                    .build()
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .span(LabelSpan::new(3..6, test_file))
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
                    .span(LabelSpan::new(0..4, test_file))
                    .message("This is a greeting.")
                    .build()
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .span(LabelSpan::new(5..8, test_file))
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
