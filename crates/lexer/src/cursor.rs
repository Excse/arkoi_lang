#[cfg(feature = "serialize")]
use serde::Serialize;

use std::iter::Peekable;
use std::str::Chars;

use crate::lexer::LexerError;
use diagnostics::{
    file::{FileID, Files},
    positional::{Span, Spannable},
};
use errors::lexer::*;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Cursor<'a> {
    file_id: FileID,
    files: &'a Files,
    #[serde(skip)]
    chars: Peekable<Chars<'a>>,
    position: usize,
    start: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(file_id: FileID, files: &'a Files) -> Cursor<'a> {
        let source = files
            .source(file_id)
            .expect("Couldn't get the source of this file.");

        Cursor {
            file_id,
            files,
            chars: source.chars().peekable(),
            position: 0,
            start: 0,
            line: 0,
        }
    }

    pub fn mark_start(&mut self) {
        self.start = self.position;
    }

    pub fn as_span(&self) -> Span {
        Span::new(self.start, self.position)
    }

    // TODO: Remove the expect
    pub fn as_str(&self) -> &'a str {
        let span = self.as_span();
        self.files
            .slice(self.file_id, &span)
            .expect("Couldn't slice the source")
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn try_consume(&mut self) -> Option<char> {
        let char = self.chars.next()?;
        if char == '\n' {
            self.line += 1;
        }

        self.position += 1;

        Some(char)
    }

    pub fn try_eat(&mut self, expected: char) -> Result<char, LexerError> {
        match self.peek() {
            Some(char) if char == expected => Ok(self.try_consume().unwrap()),
            Some(char) => Err(LexerError::Diagnostic(didnt_expect(
                self.files,
                self.file_id,
                Spannable::new(char, Span::new(self.position, self.position)),
                expected.to_string(),
            ))),
            None => Err(LexerError::EndOfFile),
        }
    }

    pub fn eat_if<F>(&mut self, predicate: F, message: &'static str) -> Result<char, LexerError>
    where
        F: FnOnce(char) -> bool,
    {
        match self.peek() {
            Some(char) if predicate(char) => Ok(self.try_consume().unwrap()),
            Some(char) => Err(LexerError::Diagnostic(didnt_expect(
                self.files,
                self.file_id,
                Spannable::new(char, Span::new(self.position, self.position)),
                message.to_string(),
            ))),
            None => Err(LexerError::EndOfFile),
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
