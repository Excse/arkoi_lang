#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{ExpressionKind, Literal, Parameter, Program, StatementKind};

pub trait Visitable<'a> {
    fn accept<V: Visitor<'a>>(&self, visitor: &mut V) -> V::Result;
}

pub trait Visitor<'a> {
    type Result;

    fn visit_program(&mut self, program: &Program) -> Self::Result;
    fn visit_statement(&mut self, statement: &StatementKind) -> Self::Result;
    fn visit_expression(&mut self, expression: &ExpressionKind) -> Self::Result;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Result;
    fn visit_parameter(&mut self, argument: &Parameter) -> Self::Result;
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum StatementResult<'a, V: Visitor<'a>> {
    Expression(V::Result),
    LetDeclaration(Option<V::Result>),
    FunDeclaration(Vec<V::Result>),
    Block(Vec<V::Result>),
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
    Call(V::Result, Vec<V::Result>),
}

pub fn walk_statement<'a, V: Visitor<'a>>(
    visitor: &mut V,
    statement: &StatementKind,
) -> StatementResult<'a, V> {
    match *statement {
        StatementKind::Expression(ref expression) => {
            let expression = visitor.visit_expression(expression);
            StatementResult::Expression(expression)
        }
        StatementKind::LetDeclaration(_, Some(ref expression)) => {
            let expression = visitor.visit_expression(expression);
            StatementResult::LetDeclaration(Some(expression))
        }
        StatementKind::LetDeclaration(_, None) => StatementResult::LetDeclaration(None),
        StatementKind::FunDeclaration(_, ref parameters, _) => {
            let parameters = parameters
                .iter()
                .map(|parameter| visitor.visit_parameter(parameter))
                .collect();
            StatementResult::FunDeclaration(parameters)
        }
        StatementKind::Block(ref statements) => {
            let statements = statements
                .iter()
                .map(|statement| visitor.visit_statement(statement))
                .collect();
            StatementResult::Block(statements)
        }
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
        ExpressionKind::Call(ref callee, ref arguments) => {
            let callee = visitor.visit_expression(callee);
            let arguments = arguments
                .iter()
                .map(|argument| visitor.visit_expression(argument))
                .collect();
            ExpressionResult::Call(callee, arguments)
        }
    }
}
