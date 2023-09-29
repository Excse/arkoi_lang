#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Rodeo;

use crate::cursor::Cursor;
use crate::error::{InternalError, LexerError, Result};
use crate::token::TokenKind;
use diagnostics::file::{FileID, Files};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Lexer<'a> {
    pub(crate) cursor: Cursor<'a>,
    pub(crate) interner: &'a mut Rodeo,
    pub(crate) file_id: FileID,
    pub errors: Vec<LexerError>,
}

impl<'a> Lexer<'a> {
    pub fn new(files: &'a Files, file_id: FileID, interner: &'a mut Rodeo) -> Lexer<'a> {
        Lexer {
            cursor: Cursor::new(file_id, files),
            interner,
            errors: Vec::new(),
            file_id,
        }
    }

    pub(crate) fn next_token_kind(&mut self) -> Result<TokenKind> {
        let current = match self.cursor.peek() {
            Some(char) => char,
            None => return Err(LexerError::Internal(InternalError::UnexpectedEOF)),
        };

        self.cursor.mark_start();
        match current {
            char if char.is_alphabetic() => self.read_identifier(),
            char if char.is_numeric() => self.read_number(),
            '"' => self.read_string(),
            _ => self.read_symbol(),
        }
    }

    fn read_symbol(&mut self) -> Result<TokenKind> {
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
            None => return Err(LexerError::Internal(InternalError::UnexpectedEOF)),
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

    fn read_identifier(&mut self) -> Result<TokenKind> {
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

    fn read_number(&mut self) -> Result<TokenKind> {
        self.cursor.eat_if(char::is_numeric, "0-9")?;

        self.cursor.eat_while(char::is_numeric);

        if self.cursor.try_eat('.').is_ok() {
            self.cursor.eat_while(char::is_numeric);
            Ok(TokenKind::Decimal)
        } else {
            Ok(TokenKind::Int)
        }
    }

    fn read_string(&mut self) -> Result<TokenKind> {
        self.cursor.try_eat('"')?;

        self.cursor
            .eat_windowed_while(|prev, curr| curr != '"' || prev == '\\');

        self.cursor.try_eat('"')?;

        Ok(TokenKind::String)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    macro_rules! test_token {
        (FAIL: $name:ident, $func:ident, $source:expr) => {
            #[test]
            fn $name() {
                let mut files = Files::default();
                let file_id = files.add("test.ark", $source);

                let mut interner = Rodeo::default();

                let mut lexer = Lexer::new(&files, file_id, &mut interner);
                let token = lexer.$func();
                assert!(token.is_err(), "{:?} should be an error", token);
            }
        };
        ($name:ident, $source:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let mut files = Files::default();
                let file_id = files.add("test.ark", $source);

                let mut interner = Rodeo::default();

                let mut lexer = Lexer::new(&files, file_id, &mut interner);
                let expected = TokenKind::from($expected);
                let token = lexer.next_token_kind().unwrap();
                assert!(token == expected, "Input was {:?}", $source);
            }
        };
    }

    test_token!(success_decimal, "4.2" => TokenKind::Decimal);
    test_token!(success_integer, "42" => TokenKind::Int);
    test_token!(FAIL: fail_number, read_number, "number");

    test_token!(success_string, "\"Hello World!\"" => TokenKind::String);
    test_token!(FAIL: fail_string, read_string, "Hello World!");

    test_token!(success_true, "true" => TokenKind::True);
    test_token!(success_false, "false" => TokenKind::False);

    test_token!(success_obracket, "{" => TokenKind::Brace(true));
    test_token!(success_cbracket, "}" => TokenKind::Brace(false));
    test_token!(success_oparent, "(" => TokenKind::Parent(true));
    test_token!(success_cparent, ")" => TokenKind::Parent(false));
    test_token!(success_at, "@" => TokenKind::At);
    test_token!(success_apostrophe, "!" => TokenKind::Apostrophe);
    test_token!(success_comma, "," => TokenKind::Comma);
    test_token!(success_period, "." => TokenKind::Period);
    test_token!(success_semicolon, ";" => TokenKind::Semicolon);
    test_token!(success_addassign, "+=" => TokenKind::PlusEq);
    test_token!(success_plus, "+" => TokenKind::Plus);
    test_token!(success_minusassing, "-=" => TokenKind::MinusEq);
    test_token!(success_minus, "-" => TokenKind::Minus);
    test_token!(success_mulassign, "*=" => TokenKind::AsteriskEq);
    test_token!(success_asterisk, "*" => TokenKind::Asterisk);
    test_token!(success_divassign, "/=" => TokenKind::SlashEq);
    test_token!(success_slash, "/" => TokenKind::Slash);
    test_token!(success_lessequal, "<=" => TokenKind::LessEq);
    test_token!(success_less, "<" => TokenKind::Less);
    test_token!(success_greaterequal, ">=" => TokenKind::GreaterEq);
    test_token!(success_greater, ">" => TokenKind::Greater);
    test_token!(success_equal, "==" => TokenKind::EqEq);
    test_token!(success_notequal, "!=" => TokenKind::NotEq);
    test_token!(success_assign, "=" => TokenKind::Eq);

    test_token!(success_self, "self" => TokenKind::Self_);
    test_token!(success_u8, "u8" => TokenKind::U8);
    test_token!(success_i8, "i8" => TokenKind::I8);
    test_token!(success_u16, "u16" => TokenKind::U16);
    test_token!(success_i16, "i16" => TokenKind::I16);
    test_token!(success_u32, "u32" => TokenKind::U32);
    test_token!(success_i32, "i32" => TokenKind::I32);
    test_token!(success_u64, "u64" => TokenKind::U64);
    test_token!(success_i64, "i64" => TokenKind::I64);
    test_token!(success_usize, "usize" => TokenKind::USize);
    test_token!(success_isize, "isize" => TokenKind::ISize);
    test_token!(success_f32, "f32" => TokenKind::F32);
    test_token!(success_f64, "f64" => TokenKind::F64);
    test_token!(success_bool, "bool" => TokenKind::Bool);

    macro_rules! insta_test {
        ($name:ident, $path:expr) => {
            #[derive(Serialize)]
            struct InstaSnapshot<'a> {
                tokens: &'a [Token],
                interner: &'a Rodeo,
            }

            #[test]
            fn $name() {
                let mut files = Files::default();

                let source = std::fs::read_to_string($path).expect("Couldn't read the file.");
                let file_id = files.add($path, &source);

                let mut interner = Rodeo::default();

                let lexer = Lexer::new(&files, file_id, &mut interner);
                let iterator = lexer.into_iter();
                let tokens = iterator.collect::<Vec<Token>>();

                insta::assert_yaml_snapshot!(InstaSnapshot {
                    tokens: &tokens,
                    interner: &interner,
                });
            }
        };
    }

    insta_test!(insta_test, "test_files/insta_test.ark");
}
