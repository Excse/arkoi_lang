#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::traversel::{Visitable, Visitor};
use lexer::token::Token;

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
pub enum LiteralKind {
    String(Token),
    Integer(Token),
    Decimal(Token),
    Boolean(Token),
}

impl ASTNode for LiteralKind {}

impl<'a> Visitable<'a> for LiteralKind {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_literal(self)
    }
}

impl LiteralKind {
    pub fn get_token(&self) -> &Token {
        match self {
            LiteralKind::String(ref token) => token,
            LiteralKind::Integer(ref token) => token,
            LiteralKind::Decimal(ref token) => token,
            LiteralKind::Boolean(ref token) => token,
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
pub enum Type {
    U8, I8,
    U16, I16,
    U32, I32,
    U64, I64,
    F32, F64,
    Bool,
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
    Literal(LiteralKind),
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
