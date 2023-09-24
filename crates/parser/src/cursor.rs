#[cfg(feature = "serialize")]
use serde::Serialize;

use std::iter::Peekable;

use crate::error::{DidntExpect, EndOfFile, Result, UnexpectedEOF};
use diagnostics::report::Labelable;
use lexer::{
    iterator::TokenIterator,
    token::{Token, TokenKind},
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub(crate) struct Cursor<'a> {
    #[serde(skip)]
    iterator: Peekable<TokenIterator<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(iterator: TokenIterator<'a>) -> Cursor<'a> {
        Cursor {
            iterator: iterator.peekable(),
        }
    }

    // TODO: Improve this method
    pub fn synchronize_program(&mut self) {
        if let Some(token) = self.consume() {
            if token.kind == TokenKind::Semicolon {
                return;
            }
        }

        while let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::Fun | TokenKind::Struct | TokenKind::Let => return,
                TokenKind::Semicolon | TokenKind::CBracket => {
                    self.consume();
                    return;
                }
                _ => {}
            };

            self.consume();
        }
    }

    // TODO: Improve this method
    pub fn synchronize_block(&mut self) {
        if let Some(token) = self.consume() {
            if token.kind == TokenKind::Semicolon {
                return;
            }
        }

        while let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::Let | TokenKind::Return => return,
                TokenKind::Semicolon | TokenKind::CBracket => {
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

    pub fn peek(&mut self) -> Result<&Token> {
        self.iterator.peek().ok_or(EndOfFile::error())
    }

    pub fn eat_any(&mut self, expected: &[TokenKind]) -> Result<Token> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => {
                let expected = expected
                    .iter()
                    .map(|kind| kind.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                return Err(UnexpectedEOF::error(expected));
            }
        };

        if expected.iter().any(|kind| kind == &token.kind) {
            return Ok(self.iterator.next().unwrap());
        }

        let expected = expected
            .iter()
            .map(|kind| kind.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        Err(DidntExpect::error(
            Labelable::new(token.kind.to_string(), token.span, token.file_id),
            expected,
        ))
    }

    pub fn eat(&mut self, expected: TokenKind) -> Result<Token> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => {
                return Err(UnexpectedEOF::error(expected.to_string()));
            }
        };

        if expected == token.kind {
            return Ok(self.iterator.next().unwrap());
        }

        Err(DidntExpect::error(
            Labelable::new(token.kind.to_string(), token.span, token.file_id),
            expected.to_string(),
        ))
    }
}
