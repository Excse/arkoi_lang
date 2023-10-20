#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, rc::Rc};

use crate::error::{
    InvalidBinaryType, InvalidUnaryType, NoSymbolFound, NoTypeFound, NotMatching, Result, TypeError,
};
use ast::{
    traversal::{Visitable, Visitor},
    Binary, BinaryOperator, Block, Call, FunDecl, Id, LetDecl, Literal, LiteralKind, Parameter,
    Program, Return, Type, TypeKind, Unary, UnaryOperator,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct TypeChecker {
    current_function: Option<Type>,
    pub errors: Vec<TypeError>,
}

impl TypeChecker {
    fn check_equality(&self, lhs: &Type, operator: BinaryOperator, rhs: &Type) -> Option<TypeKind> {
        Some(match (lhs.kind, rhs.kind) {
            (TypeKind::Bool, TypeKind::Bool) => match operator {
                BinaryOperator::Eq | BinaryOperator::NotEq => TypeKind::Bool,
                _ => return None,
            },

            (TypeKind::Int(_, _), TypeKind::Int(_, _)) => match operator {
                BinaryOperator::Eq | BinaryOperator::NotEq => TypeKind::Bool,
                _ => return None,
            },

            (TypeKind::Decimal(_), TypeKind::Decimal(_)) => match operator {
                BinaryOperator::Eq | BinaryOperator::NotEq => TypeKind::Bool,
                _ => return None,
            },

            _ => return None,
        })
    }

    fn check_comparison(
        &self,
        lhs: &Type,
        operator: BinaryOperator,
        rhs: &Type,
    ) -> Option<TypeKind> {
        Some(match (lhs.kind, rhs.kind) {
            (TypeKind::Int(_, _), TypeKind::Int(_, _)) => match operator {
                BinaryOperator::Greater
                | BinaryOperator::GreaterEq
                | BinaryOperator::Less
                | BinaryOperator::LessEq => TypeKind::Bool,
                _ => return None,
            },

            (TypeKind::Decimal(_), TypeKind::Decimal(_)) => match operator {
                BinaryOperator::Greater
                | BinaryOperator::GreaterEq
                | BinaryOperator::Less
                | BinaryOperator::LessEq => TypeKind::Bool,
                _ => return None,
            },

            _ => return None,
        })
    }

    fn check_term(&self, lhs: &Type, operator: BinaryOperator, rhs: &Type) -> Option<TypeKind> {
        Some(match (lhs.kind, rhs.kind) {
            (TypeKind::Int(signed, size), TypeKind::Int(_, _)) => match operator {
                BinaryOperator::Add | BinaryOperator::Sub => TypeKind::Int(signed, size),
                _ => return None,
            },

            (TypeKind::Decimal(size), TypeKind::Decimal(_)) => match operator {
                BinaryOperator::Add | BinaryOperator::Sub => TypeKind::Decimal(size),
                _ => return None,
            },

            _ => return None,
        })
    }

    fn check_factor(&self, lhs: &Type, operator: BinaryOperator, rhs: &Type) -> Option<TypeKind> {
        Some(match (lhs.kind, rhs.kind) {
            (TypeKind::Int(signed, size), TypeKind::Int(_, _)) => match operator {
                BinaryOperator::Div | BinaryOperator::Mul => TypeKind::Int(signed, size),
                _ => return None,
            },

            (TypeKind::Decimal(size), TypeKind::Decimal(_)) => match operator {
                BinaryOperator::Div | BinaryOperator::Mul => TypeKind::Decimal(size),
                _ => return None,
            },

            _ => return None,
        })
    }
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
                    self.errors.push(NoTypeFound::new(argument.span()).into());
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

    fn visit_binary(&mut self, node: &mut Binary) -> Result {
        let lhs_span = node.lhs.span();
        let lhs = node.lhs.accept(self)?.ok_or(NoTypeFound::new(lhs_span))?;
        let rhs_span = node.lhs.span();
        let rhs = node.rhs.accept(self)?.ok_or(NoTypeFound::new(rhs_span))?;

        let result = match node.operator {
            operator if operator.is_equality() => self.check_equality(&lhs, operator, &rhs),
            operator if operator.is_comparison() => self.check_comparison(&lhs, operator, &rhs),
            operator if operator.is_term() => self.check_term(&lhs, operator, &rhs),
            operator if operator.is_factor() => self.check_factor(&lhs, operator, &rhs),
            _ => todo!(),
        };

        if let Some(kind) = result {
            return Ok(Some(Type::new(kind, node.span)));
        }

        Err(InvalidBinaryType::new(lhs.kind, node.operator.to_string(), rhs.kind, node.span).into())
    }

    fn visit_unary(&mut self, node: &mut Unary) -> Result {
        let expression_span = node.expression.span();
        let expression = node
            .expression
            .accept(self)?
            .ok_or(NoTypeFound::new(expression_span))?;

        let type_kind = match (node.operator, expression.kind) {
            (UnaryOperator::Neg, TypeKind::Int(true, size)) => TypeKind::Int(true, size),
            (UnaryOperator::Neg, TypeKind::Decimal(size)) => TypeKind::Decimal(size),
            (UnaryOperator::LogNeg, TypeKind::Bool) => TypeKind::Bool,
            (operator, expression) => {
                return Err(
                    InvalidUnaryType::new(operator.to_string(), expression, node.span).into(),
                );
            }
        };

        Ok(Some(Type::new(type_kind, node.span)))
    }

    fn visit_return(&mut self, node: &mut Return) -> Result {
        if let Some(ref mut expression) = node.expression {
            let function_type = self
                .current_function
                .clone()
                .ok_or(NoTypeFound::new(node.span))?;

            let type_ = expression
                .accept(self)?
                .ok_or(NoTypeFound::new(node.span))?;
            if function_type != type_ {
                return Err(NotMatching::new(type_, function_type).into());
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
        let id_span = node.id.span;
        let type_ = node.type_.accept(self)?.ok_or(NoTypeFound::new(id_span))?;

        if let Some(ref mut expression) = node.expression {
            expression.accept(self)?;
        }

        let symbol = node.symbol.clone().ok_or(NoSymbolFound::new(id_span))?;
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

        let id_span = node.borrow().id.span;
        let type_ = node
            .borrow_mut()
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::new(id_span))?;

        let symbol = node
            .borrow()
            .symbol
            .clone()
            .ok_or(NoSymbolFound::new(id_span))?;
        symbol.borrow_mut().type_ = Some(type_.clone());

        let last = self.current_function.clone();
        self.current_function = Some(type_);
        node.borrow_mut().block.accept(self)?;
        self.current_function = last;

        Self::default_result()
    }

    fn visit_parameter(&mut self, node: &mut Parameter) -> Result {
        let id_span = node.id.span;
        let type_ = node.type_.accept(self)?.ok_or(NoTypeFound::new(id_span))?;

        let symbol = node.symbol.clone().ok_or(NoSymbolFound::new(id_span))?;
        symbol.borrow_mut().type_ = Some(type_);

        Self::default_result()
    }

    fn visit_id(&mut self, node: &mut Id) -> Result {
        let id_span = node.id.span;
        let symbol = node.symbol.clone().ok_or(NoSymbolFound::new(id_span))?;

        let type_ = symbol
            .borrow()
            .type_
            .clone()
            .ok_or(NoTypeFound::new(id_span))?;
        Ok(Some(type_))
    }
}
