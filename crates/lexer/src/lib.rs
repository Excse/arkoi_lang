pub mod cursor;
pub mod token;

use diagnostics::SourceDetails;
use serdebug::SerDebug;
use serde::Serialize;
use token::{Token, TokenKind};
use cursor::Cursor;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

#[derive(SerDebug, Serialize)]
pub enum LexerError {
    DidntExpect(char, &'static str),
    InternalError(&'static str),
    EndOfFile,
}

pub struct TokenIter<'a> {
    lexer: &'a mut Lexer<'a>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next_token() {
            Ok(token_kind) => Some(Token::new(self.lexer.cursor.as_span(), token_kind)),
            Err(_) => None,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source_details: &'a SourceDetails) -> Lexer<'a> {
        Lexer {
            cursor: Cursor::new(source_details),
        }
    }

    pub fn iter(&'a mut self) -> TokenIter<'a> {
        TokenIter { lexer: self }
    }

    fn next_token(&mut self) -> Result<TokenKind<'a>, LexerError> {
        self.cursor.mark_start();
        match self.cursor.peek() {
            Some(char) if char.is_alphabetic() => self.read_identifier(),
            Some(char) if char.is_numeric() => self.read_number(),
            Some('"') => self.read_string(),
            Some(_) => self.read_symbol(),
            None => Err(LexerError::EndOfFile),
        }
    }

    fn read_symbol(&mut self) -> Result<TokenKind<'a>, LexerError> {
        let mut token = match self.cursor.consume() {
            Some(char) if char.is_whitespace() => self.next_token()?,
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

    fn read_identifier(&mut self) -> Result<TokenKind<'a>, LexerError> {
        match self.cursor.consume() {
            Some(char) if !char.is_alphabetic() => {
                return Err(LexerError::DidntExpect(char, "a-zA-Z"))
            }
            Some(_) => {}
            None => return Err(LexerError::EndOfFile),
        }

        self.cursor.eat_while(char::is_alphanumeric);

        let identifier_name = self.cursor.as_str();
        Ok(match identifier_name {
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
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
            _ => TokenKind::Identifier(identifier_name),
        })
    }

    fn read_number(&mut self) -> Result<TokenKind<'a>, LexerError> {
        match self.cursor.consume() {
            Some(char) if char.is_numeric() => {}
            Some(char) => return Err(LexerError::DidntExpect(char, "0-9")),
            None => return Err(LexerError::EndOfFile),
        }

        self.cursor.eat_while(char::is_numeric);

        match self.cursor.peek() {
            Some('.') => {
                self.cursor.consume();
                self.cursor.eat_while(char::is_numeric);

                let number = self.cursor.as_str();
                number
                    .parse::<f64>()
                    .map(TokenKind::Decimal)
                    .map_err(|_| LexerError::InternalError("Couldn't parse the string to a f64."))
            }
            _ => {
                let number = self.cursor.as_str();
                number
                    .parse::<usize>()
                    .map(TokenKind::Integer)
                    .map_err(|_| LexerError::InternalError("Couldn't parse the string to a usize."))
            }
        }
    }

    fn read_string(&mut self) -> Result<TokenKind<'a>, LexerError> {
        match self.cursor.consume() {
            Some('"') => {}
            Some(char) => return Err(LexerError::DidntExpect(char, "\"")),
            None => return Err(LexerError::EndOfFile),
        };

        self.cursor
            .eat_windowed_while(|prev, curr| curr != '"' || prev == '\\');

        match self.cursor.consume() {
            Some('"') => {}
            Some(char) => return Err(LexerError::DidntExpect(char, "\"")),
            None => return Err(LexerError::EndOfFile),
        };

        let string_content = self.cursor.as_str();
        let string_content = &string_content[1..string_content.len() - 1];
        Ok(TokenKind::String(string_content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_token {
        (FAIL: $name:ident, $func:ident, $source:expr) => {
            #[test]
            fn $name() {
                let source_details = SourceDetails::new($source, "test.ark");
                let mut lexer = Lexer::new(&source_details);
                let token = lexer.$func();
                assert!(token.is_err(), "{:?} should be an error", token);
            }
        };
        ($name:ident, $source:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let source_details = SourceDetails::new($source, "test.ark");
                let mut lexer = Lexer::new(&source_details);
                let expected = TokenKind::from($expected);
                let token = lexer.next_token().unwrap();
                assert!(token.same_variant(&expected), "Input was {:?}", $source);
            }
        };
    }

    test_token!(success_decimal, "4.2" => 4.2);
    test_token!(success_integer, "42" => 42);
    test_token!(FAIL: fail_number, read_number, "number");

    test_token!(success_string, "\"Hello World!\"" => TokenKind::String("Hello World!"));
    test_token!(FAIL: fail_string, read_string, "Hello World!");

    test_token!(success_true, "true" => TokenKind::Boolean(true));
    test_token!(success_false, "false" => TokenKind::Boolean(false));

    test_token!(success_bool, "true" => TokenKind::Boolean(true));
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
            #[test]
            fn $name() {
                let source_details = match SourceDetails::read($path) {
                    Ok(source_details) => source_details,
                    Err(err) => panic!("{err}"),
                };

                let mut lexer = Lexer::new(&source_details);
                let tokens = lexer.iter().collect::<Vec<Token>>();

                insta::assert_yaml_snapshot!(&tokens);
            }
        };
    }

    insta_test!(insta_test, "test_files/insta_test.ark");
}
