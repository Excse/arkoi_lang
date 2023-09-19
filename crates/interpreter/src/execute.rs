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
pub enum Output {
    String(String),
    Integer(usize),
    Decimal(f64),
    Bool(bool),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum InterpreterError {
    Undefined,
}

impl<'a> Visitor<'a> for Interpreter<'a> {
    type Return = Output;
    type Error = InterpreterError;

    fn default_result() -> Result<Self::Return, Self::Error> {
        Err(InterpreterError::Undefined)
    }

    fn visit_literal(&mut self, node: &'a mut LiteralNode) -> Result<Self::Return, Self::Error> {
        Ok(match node.token.value {
            Some(TokenValue::String(value)) => {
                Output::String(self.interner.resolve(&value).to_string())
            }
            Some(TokenValue::Bool(value)) => Output::Bool(value),
            Some(TokenValue::Integer(value)) => Output::Integer(value),
            Some(TokenValue::Decimal(value)) => Output::Decimal(value),
            None => panic!("This shouldn't have happened."),
        })
    }

    fn visit_equality(
        &mut self,
        node: &'a mut parser::ast::EqualityNode,
    ) -> Result<Self::Return, Self::Error> {
        let lhs = self.visit_expression(&mut node.lhs)?;
        let rhs = self.visit_expression(&mut node.rhs)?;

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        Ok(match (lhs, node.operator, rhs) {
            (Output::Integer(lhs), EqualityOperator::Equal, Output::Integer(rhs)) => {
                Output::Bool(lhs == rhs)
            }
            (Output::Decimal(lhs), EqualityOperator::Equal, Output::Decimal(rhs)) => {
                Output::Bool(lhs == rhs)
            }
            (Output::Bool(lhs), EqualityOperator::Equal, Output::Bool(rhs)) => {
                Output::Bool(lhs == rhs)
            }
            (Output::Integer(lhs), EqualityOperator::NotEqual, Output::Integer(rhs)) => {
                Output::Bool(lhs != rhs)
            }
            (Output::Decimal(lhs), EqualityOperator::NotEqual, Output::Decimal(rhs)) => {
                Output::Bool(lhs != rhs)
            }
            (Output::Bool(lhs), EqualityOperator::NotEqual, Output::Bool(rhs)) => {
                Output::Bool(lhs != rhs)
            }
            _ => todo!("Equality for those types not implemented yet."),
        })
    }

    fn visit_comparison(
        &mut self,
        node: &'a mut ComparisonNode,
    ) -> Result<Self::Return, Self::Error> {
        let lhs = self.visit_expression(&mut node.lhs)?;
        let rhs = self.visit_expression(&mut node.rhs)?;

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        Ok(match (lhs, node.operator, rhs) {
            (Output::Integer(lhs), ComparisonOperator::Greater, Output::Integer(rhs)) => {
                Output::Bool(lhs > rhs)
            }
            (Output::Decimal(lhs), ComparisonOperator::Greater, Output::Decimal(rhs)) => {
                Output::Bool(lhs > rhs)
            }
            (Output::Integer(lhs), ComparisonOperator::Less, Output::Integer(rhs)) => {
                Output::Bool(lhs < rhs)
            }
            (Output::Decimal(lhs), ComparisonOperator::Less, Output::Decimal(rhs)) => {
                Output::Bool(lhs < rhs)
            }
            (Output::Integer(lhs), ComparisonOperator::GreaterEqual, Output::Integer(rhs)) => {
                Output::Bool(lhs >= rhs)
            }
            (Output::Decimal(lhs), ComparisonOperator::GreaterEqual, Output::Decimal(rhs)) => {
                Output::Bool(lhs >= rhs)
            }
            (Output::Integer(lhs), ComparisonOperator::LessEqual, Output::Integer(rhs)) => {
                Output::Bool(lhs <= rhs)
            }
            (Output::Decimal(lhs), ComparisonOperator::LessEqual, Output::Decimal(rhs)) => {
                Output::Bool(lhs <= rhs)
            }
            _ => todo!("Comparison for those types not implemented yet."),
        })
    }

    fn visit_term(
        &mut self,
        node: &'a mut parser::ast::TermNode,
    ) -> Result<Self::Return, Self::Error> {
        let lhs = self.visit_expression(&mut node.lhs)?;
        let rhs = self.visit_expression(&mut node.rhs)?;

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        Ok(match (lhs, node.operator, rhs) {
            (Output::Integer(lhs), TermOperator::Add, Output::Integer(rhs)) => {
                Output::Integer(lhs + rhs)
            }
            (Output::Decimal(lhs), TermOperator::Add, Output::Decimal(rhs)) => {
                Output::Decimal(lhs + rhs)
            }
            (Output::Integer(lhs), TermOperator::Sub, Output::Integer(rhs)) => {
                Output::Integer(lhs - rhs)
            }
            (Output::Decimal(lhs), TermOperator::Sub, Output::Decimal(rhs)) => {
                Output::Decimal(lhs - rhs)
            }
            _ => todo!("Term for those types not implemented yet."),
        })
    }

    fn visit_factor(
        &mut self,
        node: &'a mut parser::ast::FactorNode,
    ) -> Result<Self::Return, Self::Error> {
        let lhs = self.visit_expression(&mut node.lhs)?;
        let rhs = self.visit_expression(&mut node.rhs)?;

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        Ok(match (lhs, node.operator, rhs) {
            (Output::Integer(lhs), FactorOperator::Mul, Output::Integer(rhs)) => {
                Output::Integer(lhs * rhs)
            }
            (Output::Decimal(lhs), FactorOperator::Mul, Output::Decimal(rhs)) => {
                Output::Decimal(lhs * rhs)
            }
            (Output::Integer(lhs), FactorOperator::Div, Output::Integer(rhs)) => {
                Output::Integer(lhs / rhs)
            }
            (Output::Decimal(lhs), FactorOperator::Div, Output::Decimal(rhs)) => {
                Output::Decimal(lhs / rhs)
            }
            _ => todo!("Factor for those types not implemented yet."),
        })
    }

    fn visit_unary(
        &mut self,
        node: &'a mut parser::ast::UnaryNode,
    ) -> Result<Self::Return, Self::Error> {
        let expression = self.visit_expression(&mut node.expression)?;

        Ok(match (node.operator, expression) {
            // (TokenKind::Minus, Result::Integer(rhs)) => Result::Integer(-rhs),
            (UnaryOperator::Neg, Output::Decimal(rhs)) => Output::Decimal(-rhs),
            (UnaryOperator::LogNeg, Output::Bool(rhs)) => Output::Bool(!rhs),
            _ => todo!("Comparison for those types not implemented yet."),
        })
    }

    fn visit_variable(
        &mut self,
        node: &'a mut parser::ast::VariableNode,
    ) -> Result<Self::Return, Self::Error> {
        todo!()
    }

    fn visit_call(
        &mut self,
        node: &'a mut parser::ast::CallNode,
    ) -> Result<Self::Return, Self::Error> {
        todo!()
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(interner: &'a mut Rodeo) -> Self {
        Interpreter { interner }
    }

    fn convert_numerical_operands(&self, lhs: Output, rhs: Output) -> (Output, Output) {
        match (&lhs, &rhs) {
            (Output::Integer(_), Output::Integer(_)) => (lhs, rhs),
            (Output::Decimal(_), Output::Decimal(_)) => (lhs, rhs),
            (Output::Decimal(_), Output::Integer(rhs)) => {
                let rhs = Output::Decimal(*rhs as f64);
                (lhs, rhs)
            }
            (Output::Integer(lhs), Output::Decimal(_)) => {
                let lhs = Output::Decimal(*lhs as f64);
                (lhs, rhs)
            }
            _ => (lhs, rhs),
        }
    }
}
