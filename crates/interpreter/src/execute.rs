use lexer::token::TokenKind;
use parser::{
    ast::{ExpressionKind, LiteralKind, StatementKind},
    traversel::{walk_expression, walk_statement, ExpressionResult, StatementResult, Visitor},
};
use serdebug::SerDebug;
use serde::Serialize;

pub struct Interpreter;

#[derive(SerDebug, Serialize)]
pub enum Result<'a> {
    String(&'a str),
    Integer(usize),
    Decimal(f64),
    Boolean(bool),
}

impl<'a> Visitor<'a> for Interpreter {
    type Result = Result<'a>;

    fn visit_literal(&mut self, literal: &LiteralKind<'a>) -> Self::Result {
        let token = literal.get_token();
        match token.kind {
            TokenKind::String(value) => Result::String(value),
            TokenKind::Integer(value) => Result::Integer(value),
            TokenKind::Decimal(value) => Result::Decimal(value),
            TokenKind::Boolean(value) => Result::Boolean(value),
            _ => todo!("Literal kind not implemented yet."),
        }
    }

    fn visit_statement(&mut self, statement: &StatementKind<'a>) -> Self::Result {
        match walk_statement(self, statement) {
            StatementResult::Expression(result) => result,
            StatementResult::LetDeclaration(result) => result,
        }
    }

    fn visit_expression(&mut self, expression: &ExpressionKind<'a>) -> Self::Result {
        match walk_expression(self, expression) {
            ExpressionResult::Equality(lhs, rhs) => {
                let operator = expression.get_operator_token().kind;
                self.execute_equality(lhs, operator, rhs)
            }
            ExpressionResult::Comparison(lhs, rhs) => {
                let operator = expression.get_operator_token().kind;
                self.execute_comparison(lhs, operator, rhs)
            }
            ExpressionResult::Term(lhs, rhs) => {
                let operator = expression.get_operator_token().kind;
                self.execute_term(lhs, operator, rhs)
            }
            ExpressionResult::Factor(lhs, rhs) => {
                let operator = expression.get_operator_token().kind;
                self.execute_factor(lhs, operator, rhs)
            }
            ExpressionResult::Unary(rhs) => {
                let operator = expression.get_operator_token().kind;
                self.execute_unary(operator, rhs)
            },
            ExpressionResult::Grouping(result) => result,
            ExpressionResult::Literal(result) => result,
            ExpressionResult::Variable => todo!(),
        }
    }
}

impl<'a> Interpreter {
    fn convert_numerical_operands(
        &self,
        lhs: Result<'a>,
        rhs: Result<'a>,
    ) -> (Result<'a>, Result<'a>) {
        match (&lhs, &rhs) {
            (Result::Integer(_), Result::Integer(_)) => (lhs, rhs),
            (Result::Decimal(_), Result::Decimal(_)) => (lhs, rhs),
            (Result::Decimal(_), Result::Integer(rhs)) => {
                let rhs = Result::Decimal(*rhs as f64);
                (lhs, rhs)
            }
            (Result::Integer(lhs), Result::Decimal(_)) => {
                let lhs = Result::Decimal(*lhs as f64);
                (lhs, rhs)
            }
            _ => (lhs, rhs),
        }
    }

    fn execute_equality(&self, lhs: Result, operator: TokenKind, rhs: Result) -> Result<'a> {
        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, operator, rhs) {
            (Result::Integer(lhs), TokenKind::Equal, Result::Integer(rhs)) => {
                Result::Boolean(lhs == rhs)
            }
            (Result::Decimal(lhs), TokenKind::Equal, Result::Decimal(rhs)) => {
                Result::Boolean(lhs == rhs)
            }
            (Result::Boolean(lhs), TokenKind::Equal, Result::Boolean(rhs)) => {
                Result::Boolean(lhs == rhs)
            }
            (Result::Integer(lhs), TokenKind::NotEqual, Result::Integer(rhs)) => {
                Result::Boolean(lhs != rhs)
            }
            (Result::Decimal(lhs), TokenKind::NotEqual, Result::Decimal(rhs)) => {
                Result::Boolean(lhs != rhs)
            }
            (Result::Boolean(lhs), TokenKind::NotEqual, Result::Boolean(rhs)) => {
                Result::Boolean(lhs != rhs)
            }
            _ => todo!("Equality for those types not implemented yet."),
        }
    }

    fn execute_comparison(&self, lhs: Result, operator: TokenKind, rhs: Result) -> Result<'a> {
        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, operator, rhs) {
            (Result::Integer(lhs), TokenKind::Greater, Result::Integer(rhs)) => {
                Result::Boolean(lhs > rhs)
            }
            (Result::Decimal(lhs), TokenKind::Greater, Result::Decimal(rhs)) => {
                Result::Boolean(lhs > rhs)
            }
            (Result::Integer(lhs), TokenKind::Less, Result::Integer(rhs)) => {
                Result::Boolean(lhs < rhs)
            }
            (Result::Decimal(lhs), TokenKind::Less, Result::Decimal(rhs)) => {
                Result::Boolean(lhs < rhs)
            }
            (Result::Integer(lhs), TokenKind::GreaterEqual, Result::Integer(rhs)) => {
                Result::Boolean(lhs >= rhs)
            }
            (Result::Decimal(lhs), TokenKind::GreaterEqual, Result::Decimal(rhs)) => {
                Result::Boolean(lhs >= rhs)
            }
            (Result::Integer(lhs), TokenKind::LessEqual, Result::Integer(rhs)) => {
                Result::Boolean(lhs <= rhs)
            }
            (Result::Decimal(lhs), TokenKind::LessEqual, Result::Decimal(rhs)) => {
                Result::Boolean(lhs <= rhs)
            }
            _ => todo!("Comparison for those types not implemented yet."),
        }
    }

    fn execute_term(&self, lhs: Result, operator: TokenKind, rhs: Result) -> Result<'a> {
        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, operator, rhs) {
            (Result::Integer(lhs), TokenKind::Plus, Result::Integer(rhs)) => {
                Result::Integer(lhs + rhs)
            }
            (Result::Decimal(lhs), TokenKind::Plus, Result::Decimal(rhs)) => {
                Result::Decimal(lhs + rhs)
            }
            (Result::Integer(lhs), TokenKind::Minus, Result::Integer(rhs)) => {
                Result::Integer(lhs - rhs)
            }
            (Result::Decimal(lhs), TokenKind::Minus, Result::Decimal(rhs)) => {
                Result::Decimal(lhs - rhs)
            }
            _ => todo!("Term for those types not implemented yet."),
        }
    }

    fn execute_factor(&self, lhs: Result, operator: TokenKind, rhs: Result) -> Result<'a> {
        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, operator, rhs) {
            (Result::Integer(lhs), TokenKind::Asterisk, Result::Integer(rhs)) => {
                Result::Integer(lhs * rhs)
            }
            (Result::Decimal(lhs), TokenKind::Asterisk, Result::Decimal(rhs)) => {
                Result::Decimal(lhs * rhs)
            }
            (Result::Integer(lhs), TokenKind::Slash, Result::Integer(rhs)) => {
                Result::Integer(lhs / rhs)
            }
            (Result::Decimal(lhs), TokenKind::Slash, Result::Decimal(rhs)) => {
                Result::Decimal(lhs / rhs)
            }
            _ => todo!("Factor for those types not implemented yet."),
        }
    }

    fn execute_unary(&self, operator: TokenKind, rhs: Result) -> Result<'a> {
        match (operator, rhs) {
            // (TokenKind::Minus, Result::Integer(rhs)) => Result::Integer(-rhs),
            (TokenKind::Minus, Result::Decimal(rhs)) => Result::Decimal(-rhs),
            (TokenKind::Apostrophe, Result::Boolean(rhs)) => Result::Boolean(!rhs),
            _ => todo!("Comparison for those types not implemented yet."),
        }
    }
}