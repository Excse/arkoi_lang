pub mod utils;

use crate::utils::color_fmt;
use crate::utils::Color::Red;
use std::fmt::Formatter;
use std::{fmt, fs, io};
use strum_macros::AsRefStr;
use serde::Serialize;
use utils::Color;

static mut CODE_COUNT: usize = 0;

#[derive(Debug, Clone, AsRefStr, strum_macros::EnumProperty)]
pub enum ReportKind {
    #[strum(props(prefix = "I"))]
    Info,
    #[strum(props(prefix = "W"))]
    Warning,
    #[strum(props(prefix = "E"))]
    Error,
}

#[derive(Debug)]
pub struct SourceDetails {
    pub source: String,
    path: String,
}

impl SourceDetails {
    pub fn new(source: &str, path: &str) -> SourceDetails {
        SourceDetails {
            source: source.to_string(),
            path: path.to_string(),
        }
    }

    pub fn read(file_path: &str) -> Result<SourceDetails, io::Error> {
        let file_source = fs::read_to_string(file_path)?;

        Ok(SourceDetails {
            source: file_source,
            path: file_path.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Span<'a> {
    #[serde(skip)]
    source_details: &'a SourceDetails,
    line: usize,
    start: usize,
    end: usize,
}

impl<'a> Span<'a> {
    pub fn new(source_details: &'a SourceDetails, line: usize, start: usize, end: usize) -> Self {
        Span {
            source_details,
            line,
            start,
            end,
        }
    }

    pub fn label(
        source_details: &'a SourceDetails,
        line: usize,
        start: usize,
        end: usize,
    ) -> LabelBuilder {
        LabelBuilder {
            span: Span::new(source_details, line, start, end),
            message: None,
            color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label<'a> {
    span: Span<'a>,
    message: Option<String>,
    color: Option<Color>,
}

#[derive(Debug)]
pub struct LabelBuilder<'a> {
    span: Span<'a>,
    message: Option<String>,
    color: Option<Color>,
}

impl<'a> LabelBuilder<'a> {
    pub fn message(&mut self, message: &str) -> &mut Self {
        self.message = Some(message.to_owned());
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn build(&self) -> Label<'a> {
        Label {
            span: self.span.clone(),
            message: self.message.clone(),
            color: self.color.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Report<'a> {
    labels: Vec<Label<'a>>,
    code: usize,
    message: String,
    kind: ReportKind,
    note: Option<String>,
}

impl<'a> Report<'a> {
    pub fn new(message: &'a str, kind: ReportKind) -> ReportBuilder {
        ReportBuilder {
            labels: Vec::new(),
            code: None,
            message: message.to_string(),
            kind,
            note: None,
        }
    }
}

impl fmt::Display for Report<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", color_format!("{[{{:04}}] {{}}:}", Red))
        // write!(f, "{}", color_format!(&format!("{{[{:04}] {}:}}", self.code, self.kind.as_ref()), Red))
    }
}

#[derive(Debug)]
pub struct ReportBuilder<'a> {
    labels: Vec<Label<'a>>,
    code: Option<usize>,
    message: String,
    kind: ReportKind,
    note: Option<String>,
}

impl<'a> ReportBuilder<'a> {
    pub fn add_label(&mut self, label: Label<'a>) -> &mut Self {
        self.labels.push(label);
        self
    }

    pub fn code(&mut self, code: usize) -> &mut Self {
        self.code = Some(code);
        self
    }

    pub fn note(&mut self, note: &str) -> &mut Self {
        self.note = Some(note.to_owned());
        self
    }

    pub fn build(&self) -> Report<'a> {
        Report {
            labels: self.labels.clone(),
            code: self.code.unwrap_or(unsafe {
                CODE_COUNT += 1;
                CODE_COUNT
            }),
            message: self.message.to_owned(),
            kind: self.kind.clone(),
            note: self.note.clone(),
        }
    }
}
