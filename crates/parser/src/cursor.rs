use std::iter::Peekable;

use crate::reports;
use crate::ParserError;
use lexer::{
    token::{Token, TokenKind},
    Lexer, TokenIter,
};

pub(crate) struct Cursor<'a> {
    iterator: Peekable<TokenIter<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Cursor<'a> {
        Cursor {
            iterator: lexer.iter().peekable(),
        }
    }

    pub fn synchronize(&mut self) {
        if let Some(token) = self.consume() {
            match token.kind {
                TokenKind::Semicolon => return,
                _ => {}
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

    pub fn consume(&mut self) -> Option<Token<'a>> {
        self.iterator.next()
    }

    pub fn peek(&mut self) -> Result<&Token<'a>, ParserError<'a>> {
        self.iterator.peek().ok_or_else(|| ParserError::EndOfFile)
    }

    pub fn is_peek(&mut self, expected: TokenKind) -> Option<&Token<'a>> {
        let current = self.peek().ok()?;

        if expected == current.kind {
            Some(current)
        } else {
            None
        }
    }

    pub fn eat_all(&mut self, expected: &[TokenKind]) -> Result<Token<'a>, ParserError<'a>> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => return reports::unexpected_eof(expected),
        };

        if expected.iter().any(|kind| kind == &token.kind) {
            return Ok(self.iterator.next().unwrap());
        }

        reports::didnt_expect(token, expected)
    }

    pub fn eat(&mut self, expected: TokenKind) -> Result<Token<'a>, ParserError<'a>> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => return reports::unexpected_eof(&[expected]),
        };

        if expected == token.kind {
            return Ok(self.iterator.next().unwrap());
        }

        reports::didnt_expect(token, &[expected])
    }
}
