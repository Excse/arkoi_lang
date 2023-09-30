use std::fmt::Display;

#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Spur;

use diagnostics::{file::FileID, positional::Span, report::Labelable};

impl From<&Token> for Labelable<String> {
    fn from(value: &Token) -> Self {
        Labelable::new(value.kind.to_string(), value.span, value.file_id)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: Span,
    pub file_id: FileID,
    pub value: Option<TokenValue>,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(span: Span, file_id: FileID, value: Option<TokenValue>, kind: TokenKind) -> Token {
        Token {
            span,
            file_id,
            value,
            kind,
        }
    }

    pub fn get_spur(&self) -> Option<Spur> {
        match self.value {
            Some(TokenValue::String(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<usize> {
        match self.value {
            Some(TokenValue::Integer(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_dec(&self) -> Option<f64> {
        match self.value {
            Some(TokenValue::Decimal(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self.value {
            Some(TokenValue::Bool(value)) => Some(value),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Integer(usize),
    Decimal(f64),
    String(Spur),
    Bool(bool),
}

impl From<usize> for TokenValue {
    fn from(value: usize) -> Self {
        TokenValue::Integer(value)
    }
}

impl From<f64> for TokenValue {
    fn from(value: f64) -> Self {
        TokenValue::Decimal(value)
    }
}

impl From<Spur> for TokenValue {
    fn from(value: Spur) -> Self {
        TokenValue::String(value)
    }
}

impl From<bool> for TokenValue {
    fn from(value: bool) -> Self {
        TokenValue::Bool(value)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Int,
    Decimal,
    Id,
    String,
    True,
    False,

    Struct,
    Fun,
    Let,
    Return,

    Brace(bool),
    Parent(bool),
    Bracket(bool),
    At,
    Apostrophe,
    Comma,
    Period,
    Semicolon,

    PlusEq,
    Plus,
    MinusEq,
    Minus,
    AsteriskEq,
    Asterisk,
    SlashEq,
    Slash,
    LessEq,
    Less,
    GreaterEq,
    Greater,
    EqEq,
    NotEq,
    Eq,

    Self_,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    USize,
    ISize,
    F32,
    F64,
    Bool,

    Unknown(char),
}

impl Serialize for TokenKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Decimal => write!(f, "decimal"),
            Self::Id => write!(f, "identifier"),
            Self::String => write!(f, "string"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),

            Self::Struct => write!(f, "struct"),
            Self::Fun => write!(f, "fun"),
            Self::Let => write!(f, "let"),
            Self::Return => write!(f, "return"),

            Self::Bracket(opening) => write!(f, "{}", if *opening { "[" } else { "}" }),
            Self::Parent(opening) => write!(f, "{}", if *opening { "(" } else { ")" }),
            Self::Brace(opening) => write!(f, "{}", if *opening { "{" } else { "}" }),
            Self::At => write!(f, "@"),
            Self::Apostrophe => write!(f, "!"),
            Self::Comma => write!(f, ","),
            Self::Period => write!(f, "."),
            Self::Semicolon => write!(f, ";"),

            Self::PlusEq => write!(f, "+="),
            Self::Plus => write!(f, "+"),
            Self::MinusEq => write!(f, "-="),
            Self::Minus => write!(f, "-"),
            Self::AsteriskEq => write!(f, "*="),
            Self::Asterisk => write!(f, "*"),
            Self::SlashEq => write!(f, "/="),
            Self::Slash => write!(f, "/"),
            Self::LessEq => write!(f, "<="),
            Self::Less => write!(f, "<"),
            Self::GreaterEq => write!(f, ">="),
            Self::Greater => write!(f, ">"),
            Self::EqEq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),
            Self::Eq => write!(f, "="),

            Self::Self_ => write!(f, "self"),
            Self::U8 => write!(f, "u8"),
            Self::I8 => write!(f, "i8"),
            Self::U16 => write!(f, "u16"),
            Self::I16 => write!(f, "i16"),
            Self::U32 => write!(f, "u32"),
            Self::I32 => write!(f, "i32"),
            Self::U64 => write!(f, "u64"),
            Self::I64 => write!(f, "i64"),
            Self::USize => write!(f, "usize"),
            Self::ISize => write!(f, "isize"),
            Self::F32 => write!(f, "f323"),
            Self::F64 => write!(f, "f64"),
            Self::Bool => write!(f, "bool"),

            Self::Unknown(char) => write!(f, "{}", char),
        }
    }
}
