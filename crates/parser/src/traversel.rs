#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{ExpressionKind, Literal, Parameter, Program, Statement};

pub trait Visitor<'a>: Sized {
    type Result;

    fn visit_program(&mut self, program: &'a Program) {
        walk_program(self, program)
    }

    fn visit_statement(&mut self, statement: &'a Statement) -> StatementResult<'a, Self> {
        statement.walk(self)
    }

    fn visit_expression(&mut self, expression: &'a ExpressionKind) -> ExpressionResult<'a, Self> {
        expression.walk(self)
    }

    fn visit_literal(&mut self, literal: &'a Literal);

    fn visit_parameter(&mut self, argument: &'a Parameter);
}

pub fn walk_program<'a, V: Visitor<'a>>(visitor: &mut V, program: &'a Program) {
    // program
    //     .0
    //     .iter()
    //     .for_each(|statement| visitor.visit_statement(statement));
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
    statement: &Statement,
) -> StatementResult<'a, V> {
    match *statement {
        Statement::Expression(ref expression) => {
            let expression = visitor.visit_expression(expression);
            StatementResult::Expression(expression)
        }
        Statement::LetDeclaration(_, Some(ref expression)) => {
            let expression = visitor.visit_expression(expression);
            StatementResult::LetDeclaration(Some(expression))
        }
        Statement::LetDeclaration(_, None) => StatementResult::LetDeclaration(None),
        Statement::FunDeclaration(_, ref parameters, _) => {
            let parameters = parameters
                .iter()
                .map(|parameter| visitor.visit_parameter(parameter))
                .collect();
            StatementResult::FunDeclaration(parameters)
        }
        Statement::Block(ref statements) => {
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

// #[cfg_attr(feature = "serialize", derive(Serialize))]
// #[derive(Debug)]
// pub enum StatementResult<'a, V: Visitor<'a>> {
//     Expression(V::Result),
//     LetDeclaration(Option<V::Result>),
//     FunDeclaration(Vec<V::Result>, Box<StatementResult<'a, V>>),
//     Block(Vec<V::Result>),
// }

// impl Statement {
//     pub fn walk<'a, V>(&self, visitor: &mut V) -> StatementResult<'a, V>
//     where
//         V: Visitor<'a>,
//     {
//         match *self {
//             Self::Expression(ref expression) => {
//                 let expression = visitor.visit_expression(expression);
//                 StatementResult::Expression(expression)
//             }
//             Self::LetDeclaration(_, Some(ref expression)) => {
//                 let expression = visitor.visit_expression(expression);
//                 StatementResult::LetDeclaration(Some(expression))
//             }
//             Self::LetDeclaration(_, None) => StatementResult::LetDeclaration(None),
//             Self::FunDeclaration(_, ref parameters, _, ref block) => {
//                 let parameters = parameters
//                     .iter()
//                     .map(|parameter| visitor.visit_parameter(parameter))
//                     .collect();
//                 let block = visitor.visit_statement(block);
//                 StatementResult::FunDeclaration(parameters, Box::new(block))
//             }
//             Self::Block(ref statements) => {
//                 let statements = statements
//                     .iter()
//                     .map(|statement| visitor.visit_statement(statement))
//                     .collect();
//                 StatementResult::Block(statements)
//             }
//         }
//     }
// }

// #[cfg_attr(feature = "serialize", derive(Serialize))]
// #[derive(Debug)]
// pub enum ExpressionResult<'a, V: Visitor<'a>> {
//     Equality(V::Result, V::Result),
//     Comparison(V::Result, V::Result),
//     Term(V::Result, V::Result),
//     Factor(V::Result, V::Result),
//     Unary(V::Result),
//     Grouping(V::Result),
//     Literal(V::Result),
//     Variable,
//     Call(V::Result, Vec<V::Result>),
// }

// impl ExpressionKind {
//     pub fn walk<'a, V>(&self, visitor: &mut V) -> ExpressionResult<'a, V>
//     where
//         V: Visitor<'a>,
//     {
//         match *self {
//             ExpressionKind::Equality(ref lhs, _, ref rhs) => {
//                 let lhs = visitor.visit_expression(lhs);
//                 let rhs = visitor.visit_expression(rhs);
//                 ExpressionResult::Equality(lhs, rhs)
//             }
//             ExpressionKind::Comparison(ref lhs, _, ref rhs) => {
//                 let lhs = visitor.visit_expression(lhs);
//                 let rhs = visitor.visit_expression(rhs);
//                 ExpressionResult::Comparison(lhs, rhs)
//             }
//             ExpressionKind::Term(ref lhs, _, ref rhs) => {
//                 let lhs = visitor.visit_expression(lhs);
//                 let rhs = visitor.visit_expression(rhs);
//                 ExpressionResult::Term(lhs, rhs)
//             }
//             ExpressionKind::Factor(ref lhs, _, ref rhs) => {
//                 let lhs = visitor.visit_expression(lhs);
//                 let rhs = visitor.visit_expression(rhs);
//                 ExpressionResult::Factor(lhs, rhs)
//             }
//             ExpressionKind::Unary(_, ref rhs) => {
//                 let rhs = visitor.visit_expression(rhs);
//                 ExpressionResult::Unary(rhs)
//             }
//             ExpressionKind::Grouping(ref expression) => {
//                 let expression = visitor.visit_expression(expression);
//                 ExpressionResult::Grouping(expression)
//             }
//             ExpressionKind::Literal(ref literal) => {
//                 let literal = visitor.visit_literal(literal);
//                 ExpressionResult::Literal(literal)
//             }
//             ExpressionKind::Variable(_) => ExpressionResult::Variable,
//             ExpressionKind::Call(ref callee, ref arguments) => {
//                 let callee = visitor.visit_expression(callee);
//                 let arguments = arguments
//                     .iter()
//                     .map(|argument| visitor.visit_expression(argument))
//                     .collect();
//                 ExpressionResult::Call(callee, arguments)
//             }
//         }
//     }
// }
