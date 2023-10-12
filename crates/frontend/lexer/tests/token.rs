use std::cell::RefCell;
use std::rc::Rc;

use lasso::Rodeo;

use diagnostics::file::Files;
use lexer::{token::TokenKind, Lexer};

macro_rules! test_token {
    (FAIL: $name:ident, $func:ident, $source:expr) => {
        #[test]
        fn $name() {
            let mut files = Files::default();
            let file_id = files.add("test.ark", $source);

            let interner = Rc::new(RefCell::new(Rodeo::default()));

            let mut lexer = Lexer::new(&files, file_id, interner);
            let token = lexer.$func();
            assert!(token.is_err(), "{:?} should be an error", token);
        }
    };
    ($name:ident, $source:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let mut files = Files::default();
            let file_id = files.add("test.ark", $source);

            let interner = Rc::new(RefCell::new(Rodeo::default()));

            let mut lexer = Lexer::new(&files, file_id, interner);
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
