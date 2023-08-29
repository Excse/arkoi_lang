use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum TokenKind {
    #[serde(rename = "int")]
    Integer(usize),
    #[serde(rename = "decimal")]
    Decimal(f64),
    #[serde(rename = "identifier")]
    Identifier(String),
    #[serde(rename = "string")]
    QuotedString(String),

    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "fun")]
    Fun,
    
    #[serde(rename = "{")]
    OBracket,
    #[serde(rename = "}")]
    CBracket,
    #[serde(rename = "(")]
    OParent,
    #[serde(rename = ")")]
    CParent,
    #[serde(rename = "@")]
    At,
    #[serde(rename = "!")]
    Apostrophe,
    #[serde(rename = ",")]
    Comma,
    #[serde(rename = ".")]
    Period,
    #[serde(rename = ";")]
    Semicolon,

    #[serde(rename = "+=")]
    AddAssign,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-=")]
    MinusAssign,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "*=")]
    MulAssign,
    #[serde(rename = "*")]
    Asterisk,
    #[serde(rename = "/=")]
    DivAssign,
    #[serde(rename = "/")]
    Slash,
    #[serde(rename = "<=")]
    LessEqual,
    #[serde(rename = "<")]
    Less,
    #[serde(rename = ">=")]
    GreaterEqual,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "=")]
    Assign,

    #[serde(rename = "self")]
    Self_,
    #[serde(rename = "u8")]
    U8,
    #[serde(rename = "i8")]
    I8,
    #[serde(rename = "u16")]
    U16,
    #[serde(rename = "i16")]
    I16,
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "u64")]
    U64,
    #[serde(rename = "i64")]
    I64,
    #[serde(rename = "usize")]
    USize,
    #[serde(rename = "isize")]
    ISize,
    #[serde(rename = "f32")]
    F32,
    #[serde(rename = "f64")]
    F64,

    #[serde(rename = "whitespace")]
    Whitespace,
    #[serde(rename = "unknown")]
    Unknown,
}

impl From<String> for TokenKind {
    fn from(value: String) -> Self {
        TokenKind::Identifier(value)
    }
}

impl<'a> From<&'a str> for TokenKind {
    fn from(value: &'a str) -> Self {
        TokenKind::Identifier(value.to_string())
    }
}

impl From<usize> for TokenKind {
    fn from(value: usize) -> Self {
        TokenKind::Integer(value)
    }
}

impl From<f64> for TokenKind {
    fn from(value: f64) -> Self {
        TokenKind::Decimal(value)
    }
}


