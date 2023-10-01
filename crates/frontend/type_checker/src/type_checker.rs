#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::error::{InvalidBinaryType, InvalidUnaryType, NoTypeFound, Result, TypeError};
use ast::{
    traversal::{Visitable, Visitor},
    BlockNode, CallNode, ComparisonNode, EqualityNode, FactorNode, LiteralKind, LiteralNode,
    ProgramNode, TermNode, TypeKind, UnaryNode, UnaryOperator,
};
use diagnostics::positional::{Spannable, Spanned};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Type {
    kind: TypeKind,
}

impl Type {
    pub fn new(kind: TypeKind) -> Self {
        Type { kind }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct TypeChecker {
    pub errors: Vec<TypeError>,
}

impl<'a> Visitor<'a> for TypeChecker {
    type Return = Option<Type>;
    type Error = TypeError;

    fn default_result() -> Result {
        Ok(None)
    }

    fn visit_program(&mut self, node: &'a ProgramNode) -> Result {
        node.statements
            .iter()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_block(&mut self, node: &'a BlockNode) -> Result {
        node.statements
            .iter()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_call(&mut self, node: &'a CallNode) -> Result {
        node.callee.accept(self)?;

        node.arguments
            .iter()
            .for_each(|argument| match argument.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_equality(&mut self, node: &'a EqualityNode) -> Result {
        let lhs_span = *node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = *node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_ = Type::new(match (lhs.kind, node.operator, rhs.kind) {
            (TypeKind::Bool, _, TypeKind::Bool)
            | (TypeKind::Int(_, _), _, TypeKind::Int(_, _))
            | (TypeKind::Decimal(_), _, TypeKind::Decimal(_)) => TypeKind::Bool,
            (lhs, operator, rhs) => {
                let lhs = Spanned::new(lhs, lhs_span);
                let rhs = Spanned::new(rhs, rhs_span);
                return Err(InvalidBinaryType::error(lhs, operator.to_string(), rhs));
            }
        });

        Ok(Some(type_))
    }

    fn visit_comparison(&mut self, node: &'a ComparisonNode) -> Result {
        let lhs_span = *node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = *node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_ = Type::new(match (lhs.kind, node.operator, rhs.kind) {
            // TODO: Check size and sign
            (TypeKind::Int(_, _), _, TypeKind::Int(_, _))
            // TODO: Check size and sign
            | (TypeKind::Decimal(_), _, TypeKind::Decimal(_)) => TypeKind::Bool,
            (lhs, operator, rhs) => {
                let lhs = Spanned::new(lhs, lhs_span);
                let rhs = Spanned::new(rhs, rhs_span);
                return Err(InvalidBinaryType::error(lhs, operator.to_string(), rhs));
            }
        });

        Ok(Some(type_))
    }

    fn visit_term(&mut self, node: &'a TermNode) -> Result {
        let lhs_span = *node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = *node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_ = Type::new(match (lhs.kind, node.operator, rhs.kind) {
            // TODO: Check size and sign
            (TypeKind::Int(signed, size), _, TypeKind::Int(_, _)) => TypeKind::Int(signed, size),
            // TODO: Check size and sign
            (TypeKind::Decimal(size), _, TypeKind::Decimal(_)) => TypeKind::Decimal(size),
            (lhs, operator, rhs) => {
                let lhs = Spanned::new(lhs, lhs_span);
                let rhs = Spanned::new(rhs, rhs_span);
                return Err(InvalidBinaryType::error(lhs, operator.to_string(), rhs));
            }
        });

        Ok(Some(type_))
    }

    fn visit_factor(&mut self, node: &'a FactorNode) -> Result {
        let lhs_span = *node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = *node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_ = Type::new(match (lhs.kind, node.operator, rhs.kind) {
            // TODO: Check size and sign
            (TypeKind::Int(signed, size), _, TypeKind::Int(_, _)) => TypeKind::Int(signed, size),
            // TODO: Check size and sign
            (TypeKind::Decimal(size), _, TypeKind::Decimal(_)) => TypeKind::Decimal(size),
            (lhs, operator, rhs) => {
                let lhs = Spanned::new(lhs, lhs_span);
                let rhs = Spanned::new(rhs, rhs_span);
                return Err(InvalidBinaryType::error(lhs, operator.to_string(), rhs));
            }
        });

        Ok(Some(type_))
    }

    fn visit_unary(&mut self, node: &'a UnaryNode) -> Result {
        let expression_span = *node.expression.span();
        let expression = node
            .expression
            .accept(self)?
            .ok_or(NoTypeFound::error(expression_span))?;

        let type_ = Type::new(match (node.operator, expression.kind) {
            (UnaryOperator::Neg, TypeKind::Int(true, size)) => TypeKind::Int(true, size),
            (UnaryOperator::Neg, TypeKind::Decimal(size)) => TypeKind::Decimal(size),
            (UnaryOperator::LogNeg, TypeKind::Bool) => TypeKind::Bool,
            (operator, expression) => {
                let expression = Spanned::new(expression, expression_span);
                return Err(InvalidUnaryType::error(operator.to_string(), expression));
            }
        });

        Ok(Some(type_))
    }

    fn visit_literal(&mut self, node: &'a LiteralNode) -> Result {
        let type_ = Type::new(match node.kind {
            LiteralKind::Int => TypeKind::Int(
                false,
                match node.token.get_int().unwrap() {
                    value if value <= u8::MAX as usize => 8,
                    value if value <= u16::MAX as usize => 16,
                    value if value <= u32::MAX as usize => 32,
                    value if value <= u64::MAX as usize => 64,
                    _ => panic!("Invalid int size"),
                },
            ),
            LiteralKind::Decimal => {
                let value = node.token.get_dec().unwrap();
                // TODO: Maybe error prone.
                if value >= f32::MIN as f64 && value <= f32::MAX as f64 {
                    TypeKind::Decimal(32)
                } else {
                    TypeKind::Decimal(64)
                }
            }
            LiteralKind::Bool => TypeKind::Bool,
            _ => todo!(),
        });

        Ok(Some(type_))
    }
}
