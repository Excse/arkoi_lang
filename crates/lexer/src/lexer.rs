#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Rodeo;

use crate::cursor::Cursor;
use crate::token::{Token, TokenKind, TokenValue};
use diagnostics::{
    file::{FileID, Files},
    report::Report,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    interner: &'a mut Rodeo,
    pub errors: Vec<LexerError>,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum LexerError {
    Diagnostic(Report),
    EndOfFile,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct TokenIter<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_kind = match self.lexer.next_token_kind() {
            Ok(token_kind) => token_kind,
            Err(error) => match error {
                LexerError::Diagnostic(_) => {
                    self.lexer.errors.push(error);
                    return self.next();
                }
                _ => return None,
            },
        };

        let content = self.lexer.cursor.as_str();
        let span = self.lexer.cursor.as_span();

        let value = match token_kind {
            TokenKind::Integer => Some(TokenValue::Integer(content.parse::<usize>().unwrap())),
            TokenKind::Decimal => Some(TokenValue::Decimal(content.parse::<f64>().unwrap())),
            TokenKind::Identifier => Some(TokenValue::String(
                self.lexer.interner.get_or_intern(content),
            )),
            TokenKind::String => Some(TokenValue::String(
                self.lexer
                    .interner
                    .get_or_intern(&content[1..content.len() - 1]),
            )),
            TokenKind::True => Some(TokenValue::Boolean(true)),
            TokenKind::False => Some(TokenValue::Boolean(false)),
            _ => None,
        };

        Some(Token::new(span, value, token_kind))
    }
}

impl<'a> Lexer<'a> {
    pub fn new(files: &'a Files, file_id: FileID, interner: &'a mut Rodeo) -> Lexer<'a> {
        Lexer {
            cursor: Cursor::new(file_id, files),
            interner,
            errors: Vec::new(),
        }
    }

    pub fn iter(&'a mut self) -> TokenIter<'a> {
        TokenIter { lexer: self }
    }

    fn next_token_kind(&mut self) -> Result<TokenKind, LexerError> {
        self.cursor.mark_start();
        match self.cursor.peek() {
            Some(char) if char.is_alphabetic() => self.read_identifier(),
            Some(char) if char.is_numeric() => self.read_number(),
            Some('"') => self.read_string(),
            Some(_) => self.read_symbol(),
            None => Err(LexerError::EndOfFile),
        }
    }

    fn read_symbol(&mut self) -> Result<TokenKind, LexerError> {
        let mut token = match self.cursor.consume() {
            Some(char) if char.is_whitespace() => self.next_token_kind()?,
            Some('{') => TokenKind::OBracket,
            Some('}') => TokenKind::CBracket,
            Some('(') => TokenKind::OParent,
            Some(')') => TokenKind::CParent,
            Some('@') => TokenKind::At,
            Some(',') => TokenKind::Comma,
            Some('.') => TokenKind::Period,
            Some('+') => TokenKind::Plus,
            Some('-') => TokenKind::Minus,
            Some('*') => TokenKind::Asterisk,
            Some('/') => TokenKind::Slash,
            Some('<') => TokenKind::Less,
            Some('>') => TokenKind::Greater,
            Some('=') => TokenKind::Assign,
            Some('!') => TokenKind::Apostrophe,
            Some(';') => TokenKind::Semicolon,
            _ => TokenKind::Unknown,
        };

        let current = match self.cursor.peek() {
            Some(char) => char,
            None => return Ok(token),
        };

        token = match (token, current) {
            (TokenKind::Plus, '=') => TokenKind::AddAssign,
            (TokenKind::Minus, '=') => TokenKind::MinusAssign,
            (TokenKind::Asterisk, '=') => TokenKind::MulAssign,
            (TokenKind::Slash, '=') => TokenKind::DivAssign,
            (TokenKind::Less, '=') => TokenKind::LessEqual,
            (TokenKind::Greater, '=') => TokenKind::GreaterEqual,
            (TokenKind::Assign, '=') => TokenKind::Equal,
            (TokenKind::Apostrophe, '=') => TokenKind::NotEqual,
            (token, _) => return Ok(token),
        };

        self.cursor.consume();

        Ok(token)
    }

    fn read_identifier(&mut self) -> Result<TokenKind, LexerError> {
        self.cursor.eat_if(char::is_alphabetic, "a-zA-Z")?;

        self.cursor.eat_while(char::is_alphanumeric);

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
            _ => TokenKind::Identifier,
        })
    }

    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        self.cursor.eat_if(char::is_numeric, "0-9")?;

        self.cursor.eat_while(char::is_numeric);

        if self.cursor.eat('.').is_ok() {
            self.cursor.eat_while(char::is_numeric);
            Ok(TokenKind::Decimal)
        } else {
            Ok(TokenKind::Integer)
        }
    }

    fn read_string(&mut self) -> Result<TokenKind, LexerError> {
        self.cursor.eat('"')?;

        self.cursor
            .eat_windowed_while(|prev, curr| curr != '"' || prev == '\\');

        self.cursor.eat('"')?;

        Ok(TokenKind::String)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    test_token!(success_integer, "42" => TokenKind::Integer);
    test_token!(FAIL: fail_number, read_number, "number");

    test_token!(success_string, "\"Hello World!\"" => TokenKind::String);
    test_token!(FAIL: fail_string, read_string, "Hello World!");

    test_token!(success_true, "true" => TokenKind::True);
    test_token!(success_false, "false" => TokenKind::False);

    test_token!(success_obracket, "{" => TokenKind::OBracket);
    test_token!(success_cbracket, "}" => TokenKind::CBracket);
    test_token!(success_oparent, "(" => TokenKind::OParent);
    test_token!(success_cparent, ")" => TokenKind::CParent);
    test_token!(success_at, "@" => TokenKind::At);
    test_token!(success_apostrophe, "!" => TokenKind::Apostrophe);
    test_token!(success_comma, "," => TokenKind::Comma);
    test_token!(success_period, "." => TokenKind::Period);
    test_token!(success_semicolon, ";" => TokenKind::Semicolon);
    test_token!(success_addassign, "+=" => TokenKind::AddAssign);
    test_token!(success_plus, "+" => TokenKind::Plus);
    test_token!(success_minusassing, "-=" => TokenKind::MinusAssign);
    test_token!(success_minus, "-" => TokenKind::Minus);
    test_token!(success_mulassign, "*=" => TokenKind::MulAssign);
    test_token!(success_asterisk, "*" => TokenKind::Asterisk);
    test_token!(success_divassign, "/=" => TokenKind::DivAssign);
    test_token!(success_slash, "/" => TokenKind::Slash);
    test_token!(success_lessequal, "<=" => TokenKind::LessEqual);
    test_token!(success_less, "<" => TokenKind::Less);
    test_token!(success_greaterequal, ">=" => TokenKind::GreaterEqual);
    test_token!(success_greater, ">" => TokenKind::Greater);
    test_token!(success_equal, "==" => TokenKind::Equal);
    test_token!(success_notequal, "!=" => TokenKind::NotEqual);
    test_token!(success_assign, "=" => TokenKind::Assign);

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

                let mut lexer = Lexer::new(&files, file_id, &mut interner);
                let tokens = lexer.iter().collect::<Vec<Token>>();

                insta::assert_yaml_snapshot!(InstaSnapshot {
                    tokens: &tokens,
                    interner: &interner,
                });
            }
        };
    }

    insta_test!(insta_test, "test_files/insta_test.ark");
}
