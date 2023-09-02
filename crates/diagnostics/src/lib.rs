pub mod utils;

use std::{fmt::Display, fs, io};

use serde::Serialize;
use strum::EnumProperty;
use strum_macros::AsRefStr;

use utils::{color_fmt, Color};

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
    pub fn new<S>(source: S, path: S) -> SourceDetails
    where
        S: Into<String>,
    {
        SourceDetails {
            source: source.into(),
            path: path.into(),
        }
    }

    pub fn read<S>(file_path: S) -> Result<SourceDetails, io::Error>
    where
        S: Into<String>,
    {
        let file_path = file_path.into();
        let file_source = fs::read_to_string(&file_path)?;

        Ok(SourceDetails {
            source: file_source,
            path: file_path,
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
            message_colors: None,
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

impl<'a> Label<'a> {
    pub fn new(span: Span<'a>) -> LabelBuilder<'a> {
        LabelBuilder {
            span,
            message: None,
            message_colors: None,
            color: None,
        }
    }
}

#[derive(Debug)]
pub struct LabelBuilder<'a> {
    span: Span<'a>,
    message: Option<String>,
    message_colors: Option<&'a [Color]>,
    color: Option<Color>,
}

impl<'a> LabelBuilder<'a> {
    pub fn message<S>(&mut self, message: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.message = Some(message.into());
        self
    }

    pub fn message_colors(&mut self, colors: &'a [Color]) -> &mut Self {
        self.message_colors = Some(colors);
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
    message_colors: Option<&'a [Color]>,
    kind: ReportKind,
    note: Option<String>,
    note_colors: Option<&'a [Color]>,
}

impl<'a> Display for Report<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = self.kind.get_str("prefix").unwrap();
        let kind = self.kind.as_ref();

        let mut header = format!("[[[{}{:04}]]] [{}]: {}", prefix, self.code, kind, self.message);
        let colors = [&[Color::Red, Color::Red], self.message_colors.unwrap_or(&[])].concat();
        header = color_fmt(&header, &colors);

        write!(f, "{}", header)
    }
}

impl<'a> Report<'a> {
    pub fn new<S>(message: S, code: usize, kind: ReportKind) -> ReportBuilder<'a>
    where
        S: Into<String>,
    {
        ReportBuilder {
            labels: Vec::new(),
            code,
            message: message.into(),
            message_colors: None,
            kind,
            note: None,
            note_colors: None,
        }
    }
}

#[derive(Debug)]
pub struct ReportBuilder<'a> {
    labels: Vec<Label<'a>>,
    code: usize,
    message: String,
    message_colors: Option<&'a [Color]>,
    kind: ReportKind,
    note: Option<String>,
    note_colors: Option<&'a [Color]>,
}

impl<'a> ReportBuilder<'a> {
    pub fn add_label(&mut self, label: Label<'a>) -> &mut Self {
        self.labels.push(label);
        self
    }

    pub fn note<S>(&mut self, note: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.note = Some(note.into());
        self
    }

    pub fn note_colors(&mut self, colors: &'a [Color]) -> &mut Self {
        self.note_colors = Some(colors);
        self
    }

    pub fn message_colors(&mut self, colors: &'a [Color]) -> &mut Self {
        self.message_colors = Some(colors);
        self
    }

    pub fn build(&self) -> Report<'a> {
        Report {
            labels: self.labels.clone(),
            code: self.code,
            message: self.message.clone(),
            message_colors: self.message_colors,
            kind: self.kind.clone(),
            note: self.note.clone(),
            note_colors: self.note_colors,
        }
    }
}
