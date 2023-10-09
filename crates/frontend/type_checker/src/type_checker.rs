#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, rc::Rc};

use crate::error::{
    InvalidBinaryType, InvalidUnaryType, NoSymbolFound, NoTypeFound, NotMatching, Result, TypeError,
};
use ast::{
    traversal::{Visitable, Visitor},
    Block, Call, Comparison, Equality, Factor, FunDecl, Id, LetDecl, Literal, LiteralKind,
    Parameter, Program, Return, Term, Type, TypeKind, Unary, UnaryOperator,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct TypeChecker {
    current_function: Option<Type>,
    pub errors: Vec<TypeError>,
}

impl Visitor for TypeChecker {
    type Return = Option<Type>;
    type Error = TypeError;

    fn default_result() -> Result {
        Ok(None)
    }

    fn visit_program(&mut self, node: &mut Program) -> Result {
        node.statements
            .iter_mut()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_block(&mut self, node: &mut Block) -> Result {
        node.statements
            .iter_mut()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_call(&mut self, node: &mut Call) -> Result {
        node.callee.accept(self)?;

        for argument in node.arguments.iter_mut() {
            let _type_ = match argument.accept(self) {
                Ok(Some(type_)) => type_,
                Ok(None) => {
                    self.errors.push(NoTypeFound::error(argument.span()));
                    continue;
                }
                Err(error) => {
                    self.errors.push(error);
                    continue;
                }
            };
        }

        Self::default_result()
    }

    fn visit_equality(&mut self, node: &mut Equality) -> Result {
        let lhs_span = node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_kind = match (lhs.kind, node.operator, rhs.kind) {
            (TypeKind::Bool, _, TypeKind::Bool)
            | (TypeKind::Int(_, _), _, TypeKind::Int(_, _))
            | (TypeKind::Decimal(_), _, TypeKind::Decimal(_)) => TypeKind::Bool,
            (lhs, operator, rhs) => {
                return Err(InvalidBinaryType::error(
                    lhs,
                    operator.to_string(),
                    rhs,
                    node.span,
                ))
            }
        };

        Ok(Some(Type::new(type_kind, node.span)))
    }

    fn visit_comparison(&mut self, node: &mut Comparison) -> Result {
        let lhs_span = node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_kind = match (lhs.kind, node.operator, rhs.kind) {
            // TODO: Check size and sign
            (TypeKind::Int(_, _), _, TypeKind::Int(_, _))
            // TODO: Check size and sign
            | (TypeKind::Decimal(_), _, TypeKind::Decimal(_)) => TypeKind::Bool,
            (lhs, operator, rhs) => {
                return Err(InvalidBinaryType::error(lhs, operator.to_string(), rhs, node.span));
            }
        };

        Ok(Some(Type::new(type_kind, node.span)))
    }

    fn visit_term(&mut self, node: &mut Term) -> Result {
        let lhs_span = node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_kind = match (lhs.kind, node.operator, rhs.kind) {
            // TODO: Check size and sign
            (TypeKind::Int(signed, size), _, TypeKind::Int(_, _)) => TypeKind::Int(signed, size),
            // TODO: Check size and sign
            (TypeKind::Decimal(size), _, TypeKind::Decimal(_)) => TypeKind::Decimal(size),
            (lhs, operator, rhs) => {
                return Err(InvalidBinaryType::error(
                    lhs,
                    operator.to_string(),
                    rhs,
                    node.span,
                ));
            }
        };

        Ok(Some(Type::new(type_kind, node.span)))
    }

    fn visit_factor(&mut self, node: &mut Factor) -> Result {
        let lhs_span = node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::error(lhs_span))?;
        let rhs_span = node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::error(rhs_span))?;

        let type_kind = match (lhs.kind, node.operator, rhs.kind) {
            // TODO: Check size and sign
            (TypeKind::Int(signed, size), _, TypeKind::Int(_, _)) => TypeKind::Int(signed, size),
            // TODO: Check size and sign
            (TypeKind::Decimal(size), _, TypeKind::Decimal(_)) => TypeKind::Decimal(size),
            (lhs, operator, rhs) => {
                return Err(InvalidBinaryType::error(
                    lhs,
                    operator.to_string(),
                    rhs,
                    node.span,
                ));
            }
        };

        Ok(Some(Type::new(type_kind, node.span)))
    }

    fn visit_unary(&mut self, node: &mut Unary) -> Result {
        let expression_span = node.expression.span();
        let expression = node
            .expression
            .accept(self)?
            .ok_or(NoTypeFound::error(expression_span))?;

        let type_kind = match (node.operator, expression.kind) {
            (UnaryOperator::Neg, TypeKind::Int(true, size)) => TypeKind::Int(true, size),
            (UnaryOperator::Neg, TypeKind::Decimal(size)) => TypeKind::Decimal(size),
            (UnaryOperator::LogNeg, TypeKind::Bool) => TypeKind::Bool,
            (operator, expression) => {
                return Err(InvalidUnaryType::error(
                    operator.to_string(),
                    expression,
                    node.span,
                ));
            }
        };

        Ok(Some(Type::new(type_kind, node.span)))
    }

    fn visit_return(&mut self, node: &mut Return) -> Result {
        if let Some(ref mut expression) = node.expression {
            let function_type = self
                .current_function
                .clone()
                .ok_or(NoTypeFound::error(node.span))?;

            let type_ = expression
                .accept(self)?
                .ok_or(NoTypeFound::error(node.span))?;
            if function_type != type_ {
                return Err(NotMatching::error(type_, function_type));
            }
        }

        Self::default_result()
    }

    fn visit_literal(&mut self, node: &mut Literal) -> Result {
        let type_kind = match node.kind {
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
        };

        Ok(Some(Type::new(type_kind, node.token.span)))
    }

    fn visit_type(&mut self, node: &mut Type) -> Result {
        Ok(Some(node.clone()))
    }

    fn visit_let_decl(&mut self, node: &mut LetDecl) -> Result {
        let name_span = node.name.span;
        let type_ = node
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::error(name_span))?;

        if let Some(ref mut expression) = node.expression {
            expression.accept(self)?;
        }

        let symbol = node.symbol.clone().ok_or(NoSymbolFound::error(name_span))?;
        symbol.borrow_mut().type_ = Some(type_);

        Self::default_result()
    }

    fn visit_fun_decl(&mut self, node: &mut Rc<RefCell<FunDecl>>) -> Result {
        node.borrow_mut()
            .parameters
            .iter_mut()
            .for_each(|parameter| match parameter.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        let name_span = node.borrow().name.span;
        let type_ = node
            .borrow_mut()
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::error(name_span))?;

        let symbol = node
            .borrow()
            .symbol
            .clone()
            .ok_or(NoSymbolFound::error(name_span))?;
        symbol.borrow_mut().type_ = Some(type_.clone());

        let last = self.current_function.clone();
        self.current_function = Some(type_);
        node.borrow_mut().block.accept(self)?;
        self.current_function = last;

        Self::default_result()
    }

    fn visit_parameter(&mut self, node: &mut Parameter) -> Result {
        let name_span = node.name.span;
        let type_ = node
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::error(name_span))?;

        let symbol = node.symbol.clone().ok_or(NoSymbolFound::error(name_span))?;
        symbol.borrow_mut().type_ = Some(type_);

        Self::default_result()
    }

    fn visit_id(&mut self, node: &mut Id) -> Result {
        let id_span = node.id.span;
        let symbol = node.symbol.clone().ok_or(NoSymbolFound::error(id_span))?;

        let type_ = symbol
            .borrow()
            .type_
            .clone()
            .ok_or(NoTypeFound::error(id_span))?;
        Ok(Some(type_))
    }
}
