#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::traversel::Visitor;
use lexer::token::{Token, TokenKind};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct Program(pub Vec<Statement>);

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Literal {
    String(Token),
    Integer(Token),
    Decimal(Token),
    Boolean(Token),
}

impl Literal {
    pub fn get_token(&self) -> &Token {
        match self {
            Literal::String(ref token)
            | Literal::Integer(ref token)
            | Literal::Decimal(ref token)
            | Literal::Boolean(ref token) => token,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionKind),
    LetDeclaration(Token, Option<ExpressionKind>),
    FunDeclaration(Token, Vec<Parameter>, Type, Box<Statement>),
    Block(Vec<Statement>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Parameter(Token, Type);

impl Parameter {
    pub fn new(identifier: Token, type_: Type) -> Self {
        Parameter(identifier, type_)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum TypeKind {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
}

impl From<TokenKind> for TypeKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::U8 => TypeKind::U8,
            TokenKind::I8 => TypeKind::I8,
            TokenKind::U16 => TypeKind::U16,
            TokenKind::I16 => TypeKind::I16,
            TokenKind::U32 => TypeKind::U32,
            TokenKind::I32 => TypeKind::I32,
            TokenKind::U64 => TypeKind::U64,
            TokenKind::I64 => TypeKind::I64,
            TokenKind::F32 => TypeKind::F32,
            TokenKind::F64 => TypeKind::F64,
            TokenKind::Bool => TypeKind::Bool,
            _ => panic!("This tokenkind can't be converted to a typekind."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Type {
    kind: TypeKind,
}

impl Type {
    pub fn new(kind: impl Into<TypeKind>) -> Self {
        Type { kind: kind.into() }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ExpressionKind {
    Equality(Box<ExpressionKind>, Token, Box<ExpressionKind>),
    Comparison(Box<ExpressionKind>, Token, Box<ExpressionKind>),
    Term(Box<ExpressionKind>, Token, Box<ExpressionKind>),
    Factor(Box<ExpressionKind>, Token, Box<ExpressionKind>),
    Unary(Token, Box<ExpressionKind>),
    Call(Box<ExpressionKind>, Vec<ExpressionKind>),
    Grouping(Box<ExpressionKind>),
    Literal(Literal),
    Variable(Token),
}

pub enum EqualityOperator {
    Equal,
    NotEqual,
}

pub struct EqualityNode {
    lhs: ExpressionKind,
    operator: EqualityOperator,
    rhs: ExpressionKind,
}

impl ExpressionKind {
    pub fn get_operator_token(&self) -> &Token {
        match self {
            ExpressionKind::Comparison(_, ref token, _)
            | ExpressionKind::Term(_, ref token, _)
            | ExpressionKind::Factor(_, ref token, _)
            | ExpressionKind::Unary(ref token, _)
            | ExpressionKind::Equality(_, ref token, _) => token,
            _ => todo!("Operator token for this expression not implemented yet."),
        }
    }
}
