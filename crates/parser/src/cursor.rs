#[cfg(feature = "serialize")]
use serde::Serialize;

use std::iter::Peekable;

use crate::{ErrorKind, ParserError};
use diagnostics::{
    file::{FileID, Files},
    positional::Spannable,
};
use errors::parser::*;
use lexer::{
    token::{Token, TokenKind},
    Lexer, TokenIter,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub(crate) struct Cursor<'a> {
    #[serde(skip)]
    iterator: Peekable<TokenIter<'a>>,
    files: &'a Files,
    file_id: FileID,
}

impl<'a> Cursor<'a> {
    pub fn new(files: &'a Files, file_id: FileID, lexer: &'a mut Lexer<'a>) -> Cursor<'a> {
        Cursor {
            iterator: lexer.iter().peekable(),
            files,
            file_id,
        }
    }

    pub fn synchronize(&mut self) {
        if let Some(token) = self.consume() {
            if token.kind == TokenKind::Semicolon {
                return;
            }
        }

        while let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::Fun | TokenKind::Struct | TokenKind::Let | TokenKind::Return => return,
                TokenKind::Semicolon => {
                    self.consume();
                    return;
                }
                _ => {}
            };

            self.consume();
        }
    }

    pub fn consume(&mut self) -> Option<Token> {
        self.iterator.next()
    }

    pub fn peek(&mut self) -> Result<&Token, ParserError> {
        self.iterator
            .peek()
            .ok_or(ParserError::new(ErrorKind::EndOfFile))
    }

    pub fn eat_any(&mut self, expected: &[TokenKind]) -> Result<Token, ParserError> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => {
                let expected = expected
                    .iter()
                    .map(|kind| kind.as_ref())
                    .collect::<Vec<&str>>()
                    .join(", ");
                return Err(ParserError::new(ErrorKind::UnexpectedEOF(expected)));
            }
        };

        if expected.iter().any(|kind| kind == &token.kind) {
            return Ok(self.iterator.next().unwrap());
        }

        let expected = expected
            .iter()
            .map(|kind| kind.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");

        let kind = token.kind;
        let span = token.span;

        Err(ParserError::new(ErrorKind::DidntExpect(
            Spannable::new(kind.as_ref().to_string(), span),
            expected,
        )))
    }

    pub fn eat(&mut self, expected: TokenKind) -> Result<Token, ParserError> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => {
                return Err(ParserError::new(ErrorKind::UnexpectedEOF(
                    expected.as_ref().to_string(),
                )))
            }
        };

        if expected == token.kind {
            return Ok(self.iterator.next().unwrap());
        }

        let kind = token.kind;
        let span = token.span;

        Err(ParserError::new(ErrorKind::DidntExpect(
            Spannable::new(kind.as_ref().to_string(), span),
            expected.as_ref().to_string(),
        )))
    }
}
