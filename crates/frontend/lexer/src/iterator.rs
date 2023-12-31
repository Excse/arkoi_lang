#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    error::LexerError,
    token::{Token, TokenKind},
    Lexer,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct TokenIterator<'a> {
    lexer: Lexer<'a>,
}

impl<'a> TokenIterator<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }
}

impl<'a> TokenIterator<'a> {
    fn next_token(&mut self) -> Option<Token> {
        let token_kind = match self.lexer.next_token_kind() {
            Ok(token_kind) => token_kind,
            Err(error) => match error {
                LexerError::InternalError(_) => {
                    self.lexer.errors.push(error);
                    return None;
                }
                _ => {
                    self.lexer.errors.push(error);
                    return self.next_token();
                }
            },
        };

        let content = self.lexer.cursor.as_str();
        let span = self.lexer.cursor.as_span();

        let value = match token_kind {
            TokenKind::Int => {
                let content = content.parse::<usize>().unwrap().into();
                Some(content)
            }
            TokenKind::Decimal => {
                let content = content.parse::<f64>().unwrap().into();
                Some(content)
            }
            TokenKind::Id => {
                let mut interner = self.lexer.interner.borrow_mut();
                let content = interner.get_or_intern(content).into();
                Some(content)
            }
            TokenKind::String => {
                let content = &content[1..content.len() - 1];
                let mut interner = self.lexer.interner.borrow_mut();
                let content = interner.get_or_intern(content).into();
                Some(content)
            }
            TokenKind::True => Some(true.into()),
            TokenKind::False => Some(false.into()),
            _ => None,
        };

        Some(Token::new(span, self.lexer.file_id, value, token_kind))
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> IntoIterator for Lexer<'a> {
    type Item = Token;
    type IntoIter = TokenIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenIterator::new(self)
    }
}
