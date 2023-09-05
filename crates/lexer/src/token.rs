use serde::Serialize;
use serdebug::SerDebug;
use strum::AsRefStr;

use diagnostics::Span;

#[derive(SerDebug, Serialize)]
pub struct Token<'a> {
    pub span: Span<'a>,
    pub value: Option<TokenValue<'a>>,
    pub kind: TokenKind,
}

#[derive(SerDebug, Serialize)]
pub enum TokenValue<'a> {
    Integer(usize),
    Decimal(f64),
    String(&'a str),
    Boolean(bool),
}

#[derive(SerDebug, Serialize, Eq, PartialEq, Copy, Clone, AsRefStr)]
pub enum TokenKind {
    #[serde(rename = "int")]
    #[strum(serialize = "int")]
    Integer,
    #[serde(rename = "decimal")]
    #[strum(serialize = "decimal")]
    Decimal,
    #[serde(rename = "identifier")]
    #[strum(serialize = "identifier")]
    Identifier,
    #[serde(rename = "string")]
    #[strum(serialize = "string")]
    String,
    #[serde(rename = "true")]
    #[strum(serialize = "true")]
    True,
    #[serde(rename = "false")]
    #[strum(serialize = "false")]
    False,

    #[serde(rename = "struct")]
    #[strum(serialize = "struct")]
    Struct,
    #[serde(rename = "fun")]
    #[strum(serialize = "fun")]
    Fun,
    #[serde(rename = "let")]
    #[strum(serialize = "let")]
    Let,
    #[serde(rename = "return")]
    #[strum(serialize = "return")]
    Return,

    #[serde(rename = "{")]
    #[strum(serialize = "{")]
    OBracket,
    #[serde(rename = "}")]
    #[strum(serialize = "}")]
    CBracket,
    #[serde(rename = "(")]
    #[strum(serialize = "(")]
    OParent,
    #[serde(rename = ")")]
    #[strum(serialize = ")")]
    CParent,
    #[serde(rename = "@")]
    #[strum(serialize = "@")]
    At,
    #[serde(rename = "!")]
    #[strum(serialize = "|")]
    Apostrophe,
    #[serde(rename = ",")]
    #[strum(serialize = ",")]
    Comma,
    #[serde(rename = ".")]
    #[strum(serialize = ".")]
    Period,
    #[serde(rename = ";")]
    #[strum(serialize = ";")]
    Semicolon,

    #[serde(rename = "+=")]
    #[strum(serialize = "+=")]
    AddAssign,
    #[serde(rename = "+")]
    #[strum(serialize = "+=")]
    Plus,
    #[serde(rename = "-=")]
    #[strum(serialize = "-=")]
    MinusAssign,
    #[serde(rename = "-")]
    #[strum(serialize = "-")]
    Minus,
    #[serde(rename = "*=")]
    #[strum(serialize = "*=")]
    MulAssign,
    #[serde(rename = "*")]
    #[strum(serialize = "*")]
    Asterisk,
    #[serde(rename = "/=")]
    #[strum(serialize = "/=")]
    DivAssign,
    #[serde(rename = "/")]
    #[strum(serialize = "/")]
    Slash,
    #[serde(rename = "<=")]
    #[strum(serialize = "<=")]
    LessEqual,
    #[serde(rename = "<")]
    #[strum(serialize = "<")]
    Less,
    #[serde(rename = ">=")]
    #[strum(serialize = ">=")]
    GreaterEqual,
    #[serde(rename = ">")]
    #[strum(serialize = ">")]
    Greater,
    #[serde(rename = "==")]
    #[strum(serialize = "==")]
    Equal,
    #[serde(rename = "!=")]
    #[strum(serialize = "!=")]
    NotEqual,
    #[serde(rename = "=")]
    #[strum(serialize = "=")]
    Assign,

    #[serde(rename = "self")]
    #[strum(serialize = "self")]
    Self_,
    #[serde(rename = "u8")]
    #[strum(serialize = "u8")]
    U8,
    #[serde(rename = "i8")]
    #[strum(serialize = "i8")]
    I8,
    #[serde(rename = "u16")]
    #[strum(serialize = "u16")]
    U16,
    #[serde(rename = "i16")]
    #[strum(serialize = "i16")]
    I16,
    #[serde(rename = "u32")]
    #[strum(serialize = "u32")]
    U32,
    #[serde(rename = "i32")]
    #[strum(serialize = "i32")]
    I32,
    #[serde(rename = "u64")]
    #[strum(serialize = "u64")]
    U64,
    #[serde(rename = "i64")]
    #[strum(serialize = "i64")]
    I64,
    #[serde(rename = "usize")]
    #[strum(serialize = "usize")]
    USize,
    #[serde(rename = "isize")]
    #[strum(serialize = "isize")]
    ISize,
    #[serde(rename = "f32")]
    #[strum(serialize = "f32")]
    F32,
    #[serde(rename = "f64")]
    #[strum(serialize = "f64")]
    F64,

    #[serde(rename = "unknown")]
    #[strum(serialize = "unknown")]
    Unknown,
}

impl<'a> Token<'a> {
    pub fn new(span: Span<'a>, value: Option<TokenValue<'a>>, kind: TokenKind) -> Token<'a> {
        Token { span, value, kind }
    }
}
