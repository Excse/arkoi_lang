#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    error::LexerError,
    token::{Token, TokenKind},
    Lexer,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct TokenIter<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> TokenIter<'a> {
    pub(crate) fn new(lexer: &'a mut Lexer<'a>) -> Self {
        TokenIter { lexer }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_kind = match self.lexer.next_token_kind() {
            Ok(token_kind) => token_kind,
            Err(error) => match error {
                LexerError::Internal(_) => {
                    self.lexer.errors.push(error);
                    return None;
                }
                _ => {
                    self.lexer.errors.push(error);
                    return self.next();
                }
            },
        };

        let content = self.lexer.cursor.as_str();
        let span = self.lexer.cursor.as_span();

        let value = match token_kind {
            TokenKind::Integer => {
                let content = content.parse::<usize>().unwrap().into();
                Some(content)
            }
            TokenKind::Decimal => {
                let content = content.parse::<f64>().unwrap().into();
                Some(content)
            }
            TokenKind::Identifier => {
                let content = self.lexer.interner.get_or_intern(content).into();
                Some(content)
            }
            TokenKind::String => {
                let content = &content[1..content.len() - 1];
                let content = self.lexer.interner.get_or_intern(content).into();
                Some(content)
            }
            TokenKind::True => Some(true.into()),
            TokenKind::False => Some(false.into()),
            _ => None,
        };

        Some(Token::new(span, self.lexer.file_id, value, token_kind))
    }
}
