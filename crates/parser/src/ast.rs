#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::traversel::{Visitable, Visitor};
use lexer::token::{Token, TokenKind};

pub trait ASTNode {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct Program(pub Vec<StatementKind>);

impl ASTNode for Program {}

impl<'a> Visitable<'a> for Program {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_program(self)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Literal {
    String(Token),
    Integer(Token),
    Decimal(Token),
    Boolean(Token),
}

impl ASTNode for Literal {}

impl<'a> Visitable<'a> for Literal {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_literal(self)
    }
}

impl Literal {
    pub fn get_token(&self) -> &Token {
        match self {
            Literal::String(ref token) => token,
            Literal::Integer(ref token) => token,
            Literal::Decimal(ref token) => token,
            Literal::Boolean(ref token) => token,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum StatementKind {
    Expression(ExpressionKind),
    LetDeclaration(Token, Option<ExpressionKind>),
    FunDeclaration(Token, Vec<Parameter>, Type),
    Block(Vec<StatementKind>),
}

impl ASTNode for StatementKind {}

impl<'a> Visitable<'a> for StatementKind {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_statement(self)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Parameter(Token, Type);

impl ASTNode for Parameter {}

impl<'a> Visitable<'a> for Parameter {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_parameter(self)
    }
}

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

impl ASTNode for ExpressionKind {}

impl<'a> Visitable<'a> for ExpressionKind {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(self)
    }
}

impl ExpressionKind {
    pub fn get_operator_token(&self) -> &Token {
        match self {
            ExpressionKind::Comparison(_, ref token, _) => token,
            ExpressionKind::Term(_, ref token, _) => token,
            ExpressionKind::Factor(_, ref token, _) => token,
            ExpressionKind::Unary(ref token, _) => token,
            ExpressionKind::Equality(_, ref token, _) => token,
            _ => todo!("Operator token for this expression not implemented yet."),
        }
    }
}
