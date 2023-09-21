#[cfg(feature = "serialize")]
use serde::Serialize;

use std::iter::Peekable;

use crate::error::{DidntExpect, EndOfFile, Result, UnexpectedEOF};
use diagnostics::report::Labelable;
use lexer::{
    iter::TokenIter,
    token::{Token, TokenKind},
    Lexer,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub(crate) struct Cursor<'a> {
    #[serde(skip)]
    iterator: Peekable<TokenIter<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Cursor<'a> {
        Cursor {
            iterator: lexer.iter().peekable(),
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
                    .map(|kind| kind.as_ref())
                    .collect::<Vec<&str>>()
                    .join(", ");
                return Err(UnexpectedEOF::error(expected));
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

        Err(DidntExpect::error(
            Labelable::new(token.kind.as_ref().to_string(), token.span, token.file_id),
            expected,
        ))
    }

    pub fn eat(&mut self, expected: TokenKind) -> Result<Token> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => {
                return Err(UnexpectedEOF::error(expected.as_ref().to_string()));
            }
        };

        if expected == token.kind {
            return Ok(self.iterator.next().unwrap());
        }

        Err(DidntExpect::error(
            Labelable::new(token.kind.as_ref().to_string(), token.span, token.file_id),
            expected.as_ref().to_string(),
        ))
    }
}
