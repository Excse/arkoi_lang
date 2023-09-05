use serde::Serialize;
use serdebug::SerDebug;

use crate::traversel::{Visitable, Visitor};
use lexer::token::Token;

#[derive(SerDebug, Serialize)]
pub enum LiteralKind<'a> {
    String(Token<'a>),
    Integer(Token<'a>),
    Decimal(Token<'a>),
    Boolean(Token<'a>),
}

#[derive(SerDebug, Serialize)]
pub enum StatementKind<'a> {
    Expression(ExpressionKind<'a>),
    LetDeclaration(Token<'a>, Option<ExpressionKind<'a>>),
}

#[derive(SerDebug, Serialize)]
pub enum ExpressionKind<'a> {
    Equality(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Comparison(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Term(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Factor(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Unary(Token<'a>, Box<ExpressionKind<'a>>),
    Grouping(Box<ExpressionKind<'a>>),
    Literal(LiteralKind<'a>),
    Variable(Token<'a>),
}

impl<'a> Visitable<'a> for LiteralKind<'a> {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_literal(self)
    }
}

impl<'a> Visitable<'a> for ExpressionKind<'a> {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(self)
    }
}

impl<'a> Visitable<'a> for StatementKind<'a> {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_statement(self)
    }
}

impl<'a> LiteralKind<'a> {
    pub fn get_token(&self) -> &Token<'a> {
        match self {
            LiteralKind::String(ref token) => token,
            LiteralKind::Integer(ref token) => token,
            LiteralKind::Decimal(ref token) => token,
            LiteralKind::Boolean(ref token) => token,
            _ => todo!("Literal is not implemented yet."),
        }
    }
}

impl<'a> ExpressionKind<'a> {
    pub fn get_operator_token(&self) -> &Token<'a> {
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
