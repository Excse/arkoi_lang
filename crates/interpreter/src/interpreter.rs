#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, rc::Rc};

use lasso::Rodeo;

use crate::error::{InterpreterError, Result};
use ast::{
    traversal::{Visitable, Visitor},
    Call, Comparison, ComparisonOperator, Equality, EqualityOperator, Factor, FactorOperator, Id,
    Literal, Return, Term, TermOperator, Unary, UnaryOperator,
};
use lexer::token::TokenValue;
use name_resolution::symbol::Symbol;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Interpreter {
    interner: Rc<RefCell<Rodeo>>,
}

#[derive(Debug)]
pub enum Output {
    String(String),
    Integer(usize),
    Decimal(f64),
    Bool(bool),
    Function(Rc<Symbol>),
}

impl Visitor for Interpreter {
    type Return = Output;
    type Error = InterpreterError;

    fn default_result() -> Result {
        Err(InterpreterError::Undefined)
    }

    fn visit_literal(&mut self, node: &mut Literal) -> Result {
        Ok(match node.token.value {
            Some(TokenValue::String(value)) => {
                let interner = self.interner.borrow();
                Output::String(interner.resolve(&value).to_string())
            }
            Some(TokenValue::Bool(value)) => Output::Bool(value),
            Some(TokenValue::Integer(value)) => Output::Integer(value),
            Some(TokenValue::Decimal(value)) => Output::Decimal(value),
            None => panic!("This shouldn't have happened."),
        })
    }

    fn visit_equality(&mut self, node: &mut Equality) -> Result {
        let lhs = node.lhs.accept(self)?;
        let rhs = node.rhs.accept(self)?;

        let (lhs, rhs) = self.convert_numerical_operands(lhs, rhs);
        Ok(match (lhs, node.operator, rhs) {
            (Output::Integer(lhs), EqualityOperator::Eq, Output::Integer(rhs)) => {
                Output::Bool(lhs == rhs)
            }
            (Output::Decimal(lhs), EqualityOperator::Eq, Output::Decimal(rhs)) => {
                Output::Bool(lhs == rhs)
            }
            (Output::Bool(lhs), EqualityOperator::Eq, Output::Bool(rhs)) => {
                Output::Bool(lhs == rhs)
            }
            (Output::Integer(lhs), EqualityOperator::NotEq, Output::Integer(rhs)) => {
                Output::Bool(lhs != rhs)
            }
            (Output::Decimal(lhs), EqualityOperator::NotEq, Output::Decimal(rhs)) => {
                Output::Bool(lhs != rhs)
            }
            (Output::Bool(lhs), EqualityOperator::NotEq, Output::Bool(rhs)) => {
                Output::Bool(lhs != rhs)
            }
            _ => todo!("Equality for those types not implemented yet."),
        })
    }

    fn visit_comparison(&mut self, node: &mut Comparison) -> Result {
        let lhs = node.lhs.accept(self)?;
        let rhs = node.rhs.accept(self)?;

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
            (Output::Integer(lhs), ComparisonOperator::GreaterEq, Output::Integer(rhs)) => {
                Output::Bool(lhs >= rhs)
            }
            (Output::Decimal(lhs), ComparisonOperator::GreaterEq, Output::Decimal(rhs)) => {
                Output::Bool(lhs >= rhs)
            }
            (Output::Integer(lhs), ComparisonOperator::LessEq, Output::Integer(rhs)) => {
                Output::Bool(lhs <= rhs)
            }
            (Output::Decimal(lhs), ComparisonOperator::LessEq, Output::Decimal(rhs)) => {
                Output::Bool(lhs <= rhs)
            }
            _ => todo!("Comparison for those types not implemented yet."),
        })
    }

    fn visit_term(&mut self, node: &mut Term) -> Result {
        let lhs = node.lhs.accept(self)?;
        let rhs = node.rhs.accept(self)?;

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

    fn visit_factor(&mut self, node: &mut Factor) -> Result {
        let lhs = node.lhs.accept(self)?;
        let rhs = node.rhs.accept(self)?;

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

    fn visit_unary(&mut self, node: &mut Unary) -> Result {
        let expression = node.expression.accept(self)?;

        Ok(match (node.operator, expression) {
            // (TokenKind::Minus, Result::Integer(rhs)) => Result::Integer(-rhs),
            (UnaryOperator::Neg, Output::Decimal(rhs)) => Output::Decimal(-rhs),
            (UnaryOperator::LogNeg, Output::Bool(rhs)) => Output::Bool(!rhs),
            _ => todo!("Comparison for those types not implemented yet."),
        })
    }

    fn visit_id(&mut self, _node: &mut Id) -> Result {
        todo!()
    }

    fn visit_call(&mut self, _node: &mut Call) -> Result {
        todo!()
    }

    fn visit_return(&mut self, _node: &mut Return) -> Result {
        todo!()
    }
}

impl Interpreter {
    pub fn new(interner: Rc<RefCell<Rodeo>>) -> Self {
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
