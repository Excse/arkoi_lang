use diagnostics::{SourceDetails, Span};
use std::iter::Peekable;
use std::str::Chars;

pub struct Cursor<'a> {
    source_details: &'a SourceDetails,
    chars: Peekable<Chars<'a>>,
    position: usize,
    start: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source_details: &'a SourceDetails) -> Cursor<'a> {
        Cursor {
            source_details,
            chars: source_details.source.chars().peekable(),
            position: 0,
            start: 0,
            line: 0,
        }
    }

    pub fn mark_start(&mut self) {
        self.start = self.position;
    }

    pub fn as_span(&mut self) -> Span<'a> {
        Span::new(self.source_details, self.line, self.start, self.position)
    }

    pub fn as_str(&self) -> &'a str {
        &self.source_details.source[self.start..self.position]
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|char| *char)
    }

    pub fn consume(&mut self) -> Option<char> {
        let char = self.chars.next()?;
        if char == '\n' {
            self.line += 1;
        }

        self.position += 1;

        Some(char)
    }

    pub fn eat_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        self.eat_windowed_while(|_, current| predicate(current))
    }

    pub fn eat_windowed_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char, char) -> bool,
    {
        let mut last: char = '\0';
        while let Some(char) = self.peek() {
            if !predicate(last, char) {
                break;
            }

            last = char;
            self.consume();
        }
    }
}
