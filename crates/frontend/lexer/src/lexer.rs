#[cfg(feature = "serialize")]
use serde::Serialize;

use std::cell::RefCell;
use std::rc::Rc;

use lasso::Rodeo;

use crate::cursor::Cursor;
use crate::error::{EndOfFile, LexerError, Result};
use crate::token::TokenKind;
use diagnostics::file::{FileID, Files};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Lexer<'a> {
    pub(crate) cursor: Cursor<'a>,
    pub(crate) interner: Rc<RefCell<Rodeo>>,
    pub(crate) file_id: FileID,
    pub errors: Vec<LexerError>,
}

impl<'a> Lexer<'a> {
    pub fn new(files: &'a Files, file_id: FileID, interner: Rc<RefCell<Rodeo>>) -> Lexer<'a> {
        Self {
            cursor: Cursor::new(file_id, files),
            interner,
            errors: Vec::new(),
            file_id,
        }
    }

    pub fn next_token_kind(&mut self) -> Result<TokenKind> {
        let current = match self.cursor.peek() {
            Some(char) => char,
            None => return Err(EndOfFile.into()),
        };

        self.cursor.mark_start();
        match current {
            char if char.is_alphabetic() => self.read_identifier(),
            char if char.is_numeric() => self.read_number(),
            '"' => self.read_string(),
            _ => self.read_symbol(),
        }
    }

    pub fn read_symbol(&mut self) -> Result<TokenKind> {
        let mut token = match self.cursor.try_consume() {
            Some(char) if char.is_whitespace() => self.next_token_kind()?,
            Some('{') => TokenKind::Brace(true),
            Some('}') => TokenKind::Brace(false),
            Some('(') => TokenKind::Parent(true),
            Some(')') => TokenKind::Parent(false),
            Some('[') => TokenKind::Bracket(true),
            Some(']') => TokenKind::Bracket(false),
            Some('@') => TokenKind::At,
            Some(',') => TokenKind::Comma,
            Some('.') => TokenKind::Period,
            Some('+') => TokenKind::Plus,
            Some('-') => TokenKind::Minus,
            Some('*') => TokenKind::Asterisk,
            Some('/') => TokenKind::Slash,
            Some('<') => TokenKind::Less,
            Some('>') => TokenKind::Greater,
            Some('=') => TokenKind::Eq,
            Some('!') => TokenKind::Apostrophe,
            Some(';') => TokenKind::Semicolon,
            Some(char) => TokenKind::Unknown(char),
            None => return Err(EndOfFile.into()),
        };

        let current = match self.cursor.peek() {
            Some(char) => char,
            None => return Ok(token),
        };

        token = match (token, current) {
            (TokenKind::Plus, '=') => TokenKind::PlusEq,
            (TokenKind::Minus, '=') => TokenKind::MinusEq,
            (TokenKind::Asterisk, '=') => TokenKind::AsteriskEq,
            (TokenKind::Slash, '=') => TokenKind::SlashEq,
            (TokenKind::Less, '=') => TokenKind::LessEq,
            (TokenKind::Greater, '=') => TokenKind::GreaterEq,
            (TokenKind::Eq, '=') => TokenKind::EqEq,
            (TokenKind::Apostrophe, '=') => TokenKind::NotEq,
            (token, _) => return Ok(token),
        };

        self.cursor.try_consume();

        Ok(token)
    }

    pub fn read_identifier(&mut self) -> Result<TokenKind> {
        self.cursor.eat_if(char::is_alphabetic, "a-zA-Z")?;

        self.cursor
            .eat_while(|char| char.is_alphanumeric() || char == '_');

        Ok(match self.cursor.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "struct" => TokenKind::Struct,
            "return" => TokenKind::Return,
            "let" => TokenKind::Let,
            "self" => TokenKind::Self_,
            "fun" => TokenKind::Fun,
            "u8" => TokenKind::U8,
            "i8" => TokenKind::I8,
            "u16" => TokenKind::U16,
            "i16" => TokenKind::I16,
            "u32" => TokenKind::U32,
            "i32" => TokenKind::I32,
            "u64" => TokenKind::U64,
            "i64" => TokenKind::I64,
            "usize" => TokenKind::USize,
            "isize" => TokenKind::ISize,
            "f32" => TokenKind::F32,
            "f64" => TokenKind::F64,
            "bool" => TokenKind::Bool,
            _ => TokenKind::Id,
        })
    }

    pub fn read_number(&mut self) -> Result<TokenKind> {
        self.cursor.eat_if(char::is_numeric, "0-9")?;

        self.cursor.eat_while(char::is_numeric);

        if self.cursor.try_eat('.').is_ok() {
            self.cursor.eat_while(char::is_numeric);
            Ok(TokenKind::Decimal)
        } else {
            Ok(TokenKind::Int)
        }
    }

    pub fn read_string(&mut self) -> Result<TokenKind> {
        self.cursor.try_eat('"')?;

        self.cursor
            .eat_windowed_while(|prev, curr| curr != '"' || prev == '\\');

        self.cursor.try_eat('"')?;

        Ok(TokenKind::String)
    }
}
