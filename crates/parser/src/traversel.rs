#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{ExpressionKind, LiteralKind, Program, StatementKind};

pub trait Visitable<'a> {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result;
}

pub trait Visitor<'a> {
    type Result;

    fn visit_program(&mut self, program: &Program) -> Self::Result;
    fn visit_statement(&mut self, statement: &StatementKind) -> Self::Result;
    fn visit_expression(&mut self, expression: &ExpressionKind) -> Self::Result;
    fn visit_literal(&mut self, literal: &LiteralKind) -> Self::Result;
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum StatementResult<'a, V: Visitor<'a>> {
    Expression(V::Result),
    LetDeclaration(V::Result),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ExpressionResult<'a, V: Visitor<'a>> {
    Equality(V::Result, V::Result),
    Comparison(V::Result, V::Result),
    Term(V::Result, V::Result),
    Factor(V::Result, V::Result),
    Unary(V::Result),
    Grouping(V::Result),
    Literal(V::Result),
    Variable,
}

pub fn walk_statement<'a, V: Visitor<'a>>(
    visitor: &mut V,
    statement: &StatementKind,
) -> StatementResult<'a, V> {
    match *statement {
        StatementKind::Expression(ref expression) => {
            StatementResult::Expression(visitor.visit_expression(expression))
        }
        StatementKind::LetDeclaration(_, Some(ref expression)) => {
            StatementResult::LetDeclaration(visitor.visit_expression(expression))
        }
        _ => todo!("This statement walk is not implemented yet."),
    }
}

pub fn walk_expression<'a, V: Visitor<'a>>(
    visitor: &mut V,
    expression: &ExpressionKind,
) -> ExpressionResult<'a, V> {
    match *expression {
        ExpressionKind::Equality(ref lhs, _, ref rhs) => {
            let lhs = visitor.visit_expression(lhs);
            let rhs = visitor.visit_expression(rhs);
            ExpressionResult::Equality(lhs, rhs)
        }
        ExpressionKind::Comparison(ref lhs, _, ref rhs) => {
            let lhs = visitor.visit_expression(lhs);
            let rhs = visitor.visit_expression(rhs);
            ExpressionResult::Comparison(lhs, rhs)
        }
        ExpressionKind::Term(ref lhs, _, ref rhs) => {
            let lhs = visitor.visit_expression(lhs);
            let rhs = visitor.visit_expression(rhs);
            ExpressionResult::Term(lhs, rhs)
        }
        ExpressionKind::Factor(ref lhs, _, ref rhs) => {
            let lhs = visitor.visit_expression(lhs);
            let rhs = visitor.visit_expression(rhs);
            ExpressionResult::Factor(lhs, rhs)
        }
        ExpressionKind::Unary(_, ref rhs) => {
            let rhs = visitor.visit_expression(rhs);
            ExpressionResult::Unary(rhs)
        }
        ExpressionKind::Grouping(ref expression) => {
            let expression = visitor.visit_expression(expression);
            ExpressionResult::Grouping(expression)
        }
        ExpressionKind::Literal(ref literal) => {
            let literal = visitor.visit_literal(literal);
            ExpressionResult::Literal(literal)
        }
        ExpressionKind::Variable(_) => ExpressionResult::Variable,
    }
}
