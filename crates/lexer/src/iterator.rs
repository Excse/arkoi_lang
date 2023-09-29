#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    error::LexerError,
    token::{Token, TokenKind},
    Lexer,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct TokenIterator<'a>(Lexer<'a>);

impl<'a> TokenIterator<'a> {
    fn next_token(&mut self) -> Option<Token> {
        let token_kind = match self.0.next_token_kind() {
            Ok(token_kind) => token_kind,
            Err(error) => match error {
                LexerError::Internal(_) => {
                    self.0.errors.push(error);
                    return None;
                }
                _ => {
                    self.0.errors.push(error);
                    return self.next_token();
                }
            },
        };

        let content = self.0.cursor.as_str();
        let span = self.0.cursor.as_span();

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
                let content = self.0.interner.get_or_intern(content).into();
                Some(content)
            }
            TokenKind::String => {
                let content = &content[1..content.len() - 1];
                let content = self.0.interner.get_or_intern(content).into();
                Some(content)
            }
            TokenKind::True => Some(true.into()),
            TokenKind::False => Some(false.into()),
            _ => None,
        };

        Some(Token::new(span, self.0.file_id, value, token_kind))
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
        TokenIterator(self)
    }
}


