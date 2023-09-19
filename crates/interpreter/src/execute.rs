#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Rodeo;

use lexer::token::{TokenKind, TokenValue};
use parser::{
    ast::{
        ComparisonNode, ComparisonOperator, EqualityOperator, ExpressionKind, FactorOperator,
        LiteralKind, LiteralNode, ProgramNode, StatementKind, TermOperator, UnaryOperator,
    },
    traversel::Visitor,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Interpreter<'a> {
    interner: &'a mut Rodeo,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Result {
    String(String),
    Integer(usize),
    Decimal(f64),
    Bool(bool),
    Undefined,
}

impl<'a> Visitor<'a> for Interpreter<'a> {
    type Result = Result;

    fn default_result() -> Self::Result {
        Result::Undefined
    }

    fn visit_literal(&mut self, node: &'a mut LiteralNode) -> Self::Result {
        match node.token.value {
            Some(TokenValue::String(value)) => {
                Result::String(self.interner.resolve(&value).to_string())
            }
            Some(TokenValue::Bool(value)) => Result::Bool(value),
            Some(TokenValue::Integer(value)) => Result::Integer(value),
            Some(TokenValue::Decimal(value)) => Result::Decimal(value),
            None => panic!("This shouldn't have happened."),
        }
    }

    fn visit_equality(&mut self, node: &'a mut parser::ast::EqualityNode) -> Self::Result {
        let lhs = self.visit_expression(&mut node.lhs);
        let rhs = self.visit_expression(&mut node.rhs);

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, node.operator, rhs) {
            (Result::Integer(lhs), EqualityOperator::Equal, Result::Integer(rhs)) => {
                Result::Bool(lhs == rhs)
            }
            (Result::Decimal(lhs), EqualityOperator::Equal, Result::Decimal(rhs)) => {
                Result::Bool(lhs == rhs)
            }
            (Result::Bool(lhs), EqualityOperator::Equal, Result::Bool(rhs)) => {
                Result::Bool(lhs == rhs)
            }
            (Result::Integer(lhs), EqualityOperator::NotEqual, Result::Integer(rhs)) => {
                Result::Bool(lhs != rhs)
            }
            (Result::Decimal(lhs), EqualityOperator::NotEqual, Result::Decimal(rhs)) => {
                Result::Bool(lhs != rhs)
            }
            (Result::Bool(lhs), EqualityOperator::NotEqual, Result::Bool(rhs)) => {
                Result::Bool(lhs != rhs)
            }
            _ => todo!("Equality for those types not implemented yet."),
        }
    }

    fn visit_comparison(&mut self, node: &'a mut ComparisonNode) -> Self::Result {
        let lhs = self.visit_expression(&mut node.lhs);
        let rhs = self.visit_expression(&mut node.rhs);

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, node.operator, rhs) {
            (Result::Integer(lhs), ComparisonOperator::Greater, Result::Integer(rhs)) => {
                Result::Bool(lhs > rhs)
            }
            (Result::Decimal(lhs), ComparisonOperator::Greater, Result::Decimal(rhs)) => {
                Result::Bool(lhs > rhs)
            }
            (Result::Integer(lhs), ComparisonOperator::Less, Result::Integer(rhs)) => {
                Result::Bool(lhs < rhs)
            }
            (Result::Decimal(lhs), ComparisonOperator::Less, Result::Decimal(rhs)) => {
                Result::Bool(lhs < rhs)
            }
            (Result::Integer(lhs), ComparisonOperator::GreaterEqual, Result::Integer(rhs)) => {
                Result::Bool(lhs >= rhs)
            }
            (Result::Decimal(lhs), ComparisonOperator::GreaterEqual, Result::Decimal(rhs)) => {
                Result::Bool(lhs >= rhs)
            }
            (Result::Integer(lhs), ComparisonOperator::LessEqual, Result::Integer(rhs)) => {
                Result::Bool(lhs <= rhs)
            }
            (Result::Decimal(lhs), ComparisonOperator::LessEqual, Result::Decimal(rhs)) => {
                Result::Bool(lhs <= rhs)
            }
            _ => todo!("Comparison for those types not implemented yet."),
        }
    }

    fn visit_term(&mut self, node: &'a mut parser::ast::TermNode) -> Self::Result {
        let lhs = self.visit_expression(&mut node.lhs);
        let rhs = self.visit_expression(&mut node.rhs);

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, node.operator, rhs) {
            (Result::Integer(lhs), TermOperator::Add, Result::Integer(rhs)) => {
                Result::Integer(lhs + rhs)
            }
            (Result::Decimal(lhs), TermOperator::Add, Result::Decimal(rhs)) => {
                Result::Decimal(lhs + rhs)
            }
            (Result::Integer(lhs), TermOperator::Sub, Result::Integer(rhs)) => {
                Result::Integer(lhs - rhs)
            }
            (Result::Decimal(lhs), TermOperator::Sub, Result::Decimal(rhs)) => {
                Result::Decimal(lhs - rhs)
            }
            _ => todo!("Term for those types not implemented yet."),
        }
    }

    fn visit_factor(&mut self, node: &'a mut parser::ast::FactorNode) -> Self::Result {
        let lhs = self.visit_expression(&mut node.lhs);
        let rhs = self.visit_expression(&mut node.rhs);

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        match (lhs, node.operator, rhs) {
            (Result::Integer(lhs), FactorOperator::Mul, Result::Integer(rhs)) => {
                Result::Integer(lhs * rhs)
            }
            (Result::Decimal(lhs), FactorOperator::Mul, Result::Decimal(rhs)) => {
                Result::Decimal(lhs * rhs)
            }
            (Result::Integer(lhs), FactorOperator::Div, Result::Integer(rhs)) => {
                Result::Integer(lhs / rhs)
            }
            (Result::Decimal(lhs), FactorOperator::Div, Result::Decimal(rhs)) => {
                Result::Decimal(lhs / rhs)
            }
            _ => todo!("Factor for those types not implemented yet."),
        }
    }

    fn visit_unary(&mut self, node: &'a mut parser::ast::UnaryNode) -> Self::Result {
        let expression = self.visit_expression(&mut node.expression);

        match (node.operator, expression) {
            // (TokenKind::Minus, Result::Integer(rhs)) => Result::Integer(-rhs),
            (UnaryOperator::Neg, Result::Decimal(rhs)) => Result::Decimal(-rhs),
            (UnaryOperator::LogNeg, Result::Bool(rhs)) => Result::Bool(!rhs),
            _ => todo!("Comparison for those types not implemented yet."),
        }
    }

    fn visit_variable(&mut self, node: &'a mut parser::ast::VariableNode) -> Self::Result {
        todo!()
    }

    fn visit_call(&mut self, node: &'a mut parser::ast::CallNode) -> Self::Result {
        todo!()
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(interner: &'a mut Rodeo) -> Self {
        Interpreter { interner }
    }

    fn convert_numerical_operands(&self, lhs: Result, rhs: Result) -> (Result, Result) {
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
}
