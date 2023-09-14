#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::positional::Span;

pub type FileID = u32;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct File {
    pub(crate) path: String,
    pub(crate) source: String,
    pub(crate) lines: Vec<Span>,
}

impl File {
    fn new<S>(path: S, source: S) -> Self
    where
        S: Into<String>,
    {
        let source = source.into();
        let path = path.into();

        let lines = File::line_ranges(&source);

        File {
            path,
            source,
            lines,
        }
    }

    fn line_ranges(source: &String) -> Vec<Span> {
        let mut lines = Vec::new();

        let mut start = 0;
        for (index, test) in source.match_indices('\n') {
            let span = Span::new(start, index);
            lines.push(span);
            start = index + 1;
        }

        if start != source.len() {
            let span = Span::new(start, source.len());
            lines.push(span)
        }

        lines
    }

    pub fn find_line_span(&self, char_span: &Span) -> Option<Span> {
        let start = self
            .lines
            .iter()
            .position(|span| span.is_inside(char_span.start))?;
        let end = self
            .lines
            .iter()
            .position(|span| span.is_inside(char_span.end))?;

        Some(Span::new(start, end))
    }

    pub fn slice(&self, span: &Span) -> Option<&str> {
        if span.end > self.source.len() {
            return None;
        }

        Some(&self.source[span.start..span.end])
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Files {
    files: Vec<File>,
}

impl Default for Files {
    fn default() -> Self {
        Files::new()
    }
}

impl Files {
    pub fn new() -> Self {
        Files { files: Vec::new() }
    }

    pub fn add<S>(&mut self, path: S, source: S) -> FileID
    where
        S: Into<String>,
    {
        self.files.push(File::new(path, source));

        u32::try_from(self.files.len()).expect("Shouldn't exceed the u32 bounds.")
    }

    pub fn get(&self, file_id: FileID) -> Option<&File> {
        self.files.get((file_id - 1) as usize)
    }

    pub fn path(&self, file_id: FileID) -> Option<&str> {
        let file = self.get(file_id)?;
        Some(&file.path)
    }

    pub fn source(&self, file_id: FileID) -> Option<&str> {
        let file = self.get(file_id)?;
        Some(&file.source)
    }

    pub fn slice(&self, file_id: FileID, span: &Span) -> Option<&str> {
        let file = self.get(file_id)?;
        file.slice(span)
    }

    pub fn find_line_span(&self, file_id: FileID, char_span: &Span) -> Option<Span> {
        let file = self.get(file_id)?;
        file.find_line_span(char_span)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn files() {
        let mut files = Files::default();

        let file_id = files.add("test_1.ark", "Hello World from 1!");
        assert_eq!(file_id, 1);
        assert_eq!(files.path(file_id), Some("test_1.ark"));
        assert_eq!(files.source(file_id), Some("Hello World from 1!"));
        let span = Span::new(17, 18);
        assert_eq!(files.slice(file_id, &span), Some("1"));

        let file_id = files.add("test_2.ark", "Hello World from 2!");
        assert_eq!(file_id, 2);
        assert_eq!(files.path(file_id), Some("test_2.ark"));
        assert_eq!(files.source(file_id), Some("Hello World from 2!"));
        let span = Span::new(17, 18);
        assert_eq!(files.slice(file_id, &span), Some("2"));
    }

    #[test]
    fn lines() {
        let mut files = Files::new();

        let file_id = files.add("test.ark", "Hello\nWorld\n!");
        let file = files.get(file_id).unwrap();

        assert_eq!(
            file.lines,
            vec![Span::new(0, 5), Span::new(6, 11), Span::new(12, 13)]
        );

        let lines = file
            .lines
            .iter()
            .map(|span| file.slice(span).unwrap())
            .collect::<Vec<&str>>();
        assert_eq!(lines, vec!["Hello", "World", "!"]);
    }

    #[test]
    fn line_spans() {
        let mut files = Files::new();

        let file_id = files.add(
            "test.ark",
            "Hello World!\nWhat is\nup?\nHave a great day :)",
        );
        let file = files.get(file_id).unwrap();

        let span = Span::new(0, 4);
        assert_eq!(file.find_line_span(&span), Some(Span::new(0, 0)));
        assert_eq!(file.slice(&span), Some("Hell"));

        let span = Span::new(0, 13);
        assert_eq!(file.find_line_span(&span), Some(Span::new(0, 1)));
        assert_eq!(file.slice(&span), Some("Hello World!\n"));

        let span = Span::new(13, 20);
        assert_eq!(file.find_line_span(&span), Some(Span::new(1, 1)));
        assert_eq!(file.slice(&span), Some("What is"));

        let span = Span::new(19, 23);
        assert_eq!(file.find_line_span(&span), Some(Span::new(1, 2)));
        assert_eq!(file.slice(&span), Some("s\nup"));
    }
}
