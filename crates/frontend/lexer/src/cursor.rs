#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{iter::Peekable, str::CharIndices};

use crate::error::{DidntExpect, EndOfFile, Result};
use diagnostics::{
    file::{FileID, Files},
    positional::{LabelSpan, Span},
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Cursor<'a> {
    file_id: FileID,
    files: &'a Files,
    #[serde(skip)]
    chars: Peekable<CharIndices<'a>>,
    length: usize,
    start: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(file_id: FileID, files: &'a Files) -> Cursor<'a> {
        let source = files
            .source(file_id)
            .expect("Couldn't get the source of this file.");

        Self {
            file_id,
            files,
            chars: source.char_indices().peekable(),
            length: source.len(),
            start: 0,
        }
    }

    pub fn current_index(&mut self) -> usize {
        self.peek_indexed()
            .map(|(index, _)| index)
            .unwrap_or(self.length)
    }

    pub fn mark_start(&mut self) {
        self.start = self.current_index()
    }

    pub fn as_span(&mut self) -> LabelSpan {
        LabelSpan::new(Span::new(self.start, self.current_index()), self.file_id)
    }

    // TODO: Remove the expect
    pub fn as_str(&mut self) -> &'a str {
        let span = self.as_span();
        self.files
            .slice(self.file_id, &span.span)
            .expect("Couldn't slice the source")
    }

    pub fn peek_indexed(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.peek_indexed().map(|(_, char)| char)
    }

    pub fn try_consume(&mut self) -> Option<char> {
        let char = self.chars.next().map(|(_, char)| char)?;
        Some(char)
    }

    pub fn try_eat(&mut self, expected: char) -> Result<char> {
        match self.peek_indexed() {
            Some((_, char)) if char == expected => Ok(self.try_consume().unwrap()),
            Some((index, char)) => Err(DidntExpect::new(
                char,
                LabelSpan::new(Span::single(index), self.file_id),
                expected,
            )
            .into()),
            None => Err(EndOfFile.into()),
        }
    }

    pub fn eat_if<F>(&mut self, predicate: F, message: &'static str) -> Result<char>
    where
        F: FnOnce(char) -> bool,
    {
        match self.peek_indexed() {
            Some((_, char)) if predicate(char) => Ok(self.try_consume().unwrap()),
            Some((index, char)) => Err(DidntExpect::new(
                char,
                LabelSpan::new(Span::single(index), self.file_id),
                message,
            )
            .into()),
            None => Err(EndOfFile.into()),
        }
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
            self.try_consume();
        }
    }
}
