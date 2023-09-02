use std::iter::Peekable;

use lexer::{
    token::{Token, TokenKind},
    Lexer, TokenIter,
};
use crate::ParserError;
use crate::reports;

pub struct Cursor<'a> {
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

    pub fn consume(&mut self) -> Option<Token<'a>> {
        self.iterator.next()
    }

    pub fn peek(&mut self) -> Result<&Token<'a>, ParserError<'a>> {
        self.iterator.peek().ok_or_else(|| ParserError::EndOfFile)
    }

    pub fn matches(
        &mut self,
        expected: &'static [TokenKind],
    ) -> Result<Token<'a>, ParserError<'a>> {
        let token = match self.peek() {
            Ok(token) => token,
            Err(_) => return Err(reports::unexpected_eof(expected)),
        };

        if expected.iter().any(|kind| kind == &token.kind) {
            return Ok(self.iterator.next().unwrap());
        }

        Err(reports::didnt_expect(token, expected))
    }
}
