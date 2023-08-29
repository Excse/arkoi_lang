pub mod token; 

use crate::token::TokenKind;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

#[derive(Debug)]
pub enum LexerError {
    DidntExpect(char, &'static str),
    InternalError(&'static str),
    UnexpectedEOF,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<TokenKind> {
        let mut tokens = Vec::new();

        while let Ok(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Result<TokenKind, LexerError> {
        match self.current()? {
            char if char.is_alphabetic() => self.read_identifier(),
            char if char.is_numeric() => self.read_number(),
            '"' => self.read_string(),
            _ => self.read_symbol(),
        }
    }

    fn read_symbol(&mut self) -> Result<TokenKind, LexerError> {
        let mut token = match self.consume()? {
            char if char.is_whitespace() => TokenKind::Whitespace,
            '{' => TokenKind::OBracket,
            '}' => TokenKind::CBracket,
            '(' => TokenKind::OParent,
            ')' => TokenKind::CParent,
            '@' => TokenKind::At,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Period,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            '=' => TokenKind::Assign,
            '!' => TokenKind::Apostrophe,
            ';' => TokenKind::Semicolon,
            _ => TokenKind::Unknown,
        };

        let current = match self.current() {
            Ok(char) => char,
            Err(_) => return Ok(token),
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

        self.consume()?;

        Ok(token)
    }

    fn read_identifier(&mut self) -> Result<TokenKind, LexerError> {
        let mut identifier_name = String::new();

        match self.consume() {
            Ok(char) if !char.is_alphabetic() => {
                return Err(LexerError::DidntExpect(char, "a-zA-Z"))
            }
            Ok(char) => identifier_name.push(char),
            Err(error) => return Err(error),
        }

        identifier_name.push_str(&self.consume_while(char::is_alphanumeric));

        Ok(match identifier_name.as_str() {
            "struct" => TokenKind::Struct,
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

    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        let mut number = String::new();

        match self.consume() {
            Ok(char) if char.is_numeric() => number.push(char),
            Ok(char) => return Err(LexerError::DidntExpect(char, "0-9")),
            Err(error) => return Err(error),
        }

        number.push_str(&self.consume_while(char::is_numeric));

        match self.current() {
            Ok('.') => {
                number.push(self.consume()?);

                number.push_str(&self.consume_while(char::is_numeric));

                number
                    .parse::<f64>()
                    .map(TokenKind::Decimal)
                    .map_err(|_| LexerError::InternalError("Couldn't parse the string to a f64."))
            }
            _ => number
                .parse::<usize>()
                .map(TokenKind::Integer)
                .map_err(|_| LexerError::InternalError("Couldn't parse the string to a usize.")),
        }
    }

    fn read_string(&mut self) -> Result<TokenKind, LexerError> {
        let mut string = String::new();

        match self.current() {
            Ok('"') => self.consume()?,
            Ok(char) => return Err(LexerError::DidntExpect(char, "\"")),
            Err(error) => return Err(error),
        };

        string.push_str(&self.consume_windowed_while(|prev, curr| curr != '"' || prev == '\\'));

        match self.consume() {
            Ok('"') => {}
            Ok(char) => return Err(LexerError::DidntExpect(char, "\"")),
            Err(error) => return Err(error),
        };

        Ok(TokenKind::QuotedString(string))
    }

    fn current(&self) -> Result<char, LexerError> {
        self.source
            .get(self.position)
            .map(|char| *char)
            .ok_or(LexerError::UnexpectedEOF)
    }

    fn consume_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        self.consume_windowed_while(|_, current| predicate(current))
    }

    fn consume_windowed_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char, char) -> bool,
    {
        let mut result = String::new();
        let mut last: char = '\0';

        while let Ok(char) = self.consume() {
            if !predicate(last, char) {
                self.position -= 1;
                break;
            }

            last = char;
            result.push(char);
        }

        result
    }

    fn consume(&mut self) -> Result<char, LexerError> {
        let current = self.current()?;
        self.position += 1;
        Ok(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_token {
        (FAIL: $name:ident, $func:ident, $source:expr) => {
            #[test]
            fn $name() {
                let mut lexer = Lexer::new($source);
                let token = lexer.$func();
                assert!(token.is_err(), "{:?} should be an error", token);
            }
        };
        ($name:ident, $source:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let mut lexer = Lexer::new($source);
                let expected = TokenKind::from($expected);
                let token = lexer.next_token().unwrap();
                assert_eq!(token, expected, "Input was {:?}", $source);
            }
        };
    }

    test_token!(success_decimal, "4.2" => 4.2);
    test_token!(success_integer, "42" => 42);
    test_token!(FAIL: fail_number, read_number, "number");

    test_token!(success_string, "\"Hello World!\"" => TokenKind::QuotedString("Hello World!".to_string()));
    test_token!(FAIL: fail_string, read_string, "Hello World!");

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
                let source_code = match std::fs::read_to_string($path) {
                    Ok(code) => code,
                    Err(err) => panic!("{err}"),
                };

                let mut lexer = Lexer::new(&source_code);
                let mut tokens = lexer.tokenize();
                tokens.retain(|token| !matches!(token, TokenKind::Whitespace));

                insta::assert_yaml_snapshot!(&tokens);
            }
        }
    }

    insta_test!(insta_test, "test_files/insta_test.ark");
}
