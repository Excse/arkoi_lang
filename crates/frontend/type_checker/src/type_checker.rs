#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::error::{
    InvalidBinaryType, InvalidUnaryType, NoSymbolFound, NoTypeFound, NotMatching, Result, TypeError,
};
use ast::{
    traversal::{Visitable, Visitor},
    Block, Call, Comparison, Equality, Factor, FunDecl, Id, LetDecl, Literal, LiteralKind,
    Parameter, Program, Return, Term, Type, TypeKind, Unary, UnaryOperator,
};
use name_resolution::ResolvedSymbols;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct TypeChecker {
    resolved: ResolvedSymbols,
    current_function: Option<Type>,
    pub errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new(resolved: ResolvedSymbols) -> Self {
        TypeChecker {
            resolved,
            current_function: None,
            errors: Vec::new(),
        }
    }
}

impl Visitor for TypeChecker {
    type Return = Option<Type>;
    type Error = TypeError;

    fn default_result() -> Result {
        Ok(None)
    }

    fn visit_program(&mut self, node: &Program) -> Result {
        node.statements
            .iter()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_block(&mut self, node: &Block) -> Result {
        node.statements
            .iter()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_call(&mut self, node: &Call) -> Result {
        node.callee.accept(self)?;

        for argument in node.arguments.iter() {
            let type_ = match argument.accept(self) {
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

    fn visit_equality(&mut self, node: &Equality) -> Result {
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

    fn visit_comparison(&mut self, node: &Comparison) -> Result {
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

    fn visit_term(&mut self, node: &Term) -> Result {
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

    fn visit_factor(&mut self, node: &Factor) -> Result {
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

    fn visit_unary(&mut self, node: &Unary) -> Result {
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

    fn visit_return(&mut self, node: &Return) -> Result {
        if let Some(ref expression) = node.expression {
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

    fn visit_literal(&mut self, node: &Literal) -> Result {
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

    fn visit_type(&mut self, node: &Type) -> Result {
        Ok(Some(node.clone()))
    }

    fn visit_let_decl(&mut self, node: &LetDecl) -> Result {
        let name_span = node.name.span;
        let type_ = node
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::error(name_span))?;

        if let Some(ref expression) = node.expression {
            expression.accept(self)?;
        }

        let symbol = self
            .resolved
            .get(node)
            .ok_or(NoSymbolFound::error(name_span))?;
        let mut symbol = symbol.borrow_mut();
        symbol.type_ = Some(type_);

        Self::default_result()
    }

    fn visit_fun_decl(&mut self, node: &FunDecl) -> Result {
        node.parameters
            .iter()
            .for_each(|parameter| match parameter.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        let name_span = node.name.span;
        let type_ = node
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::error(name_span))?;

        let symbol = self
            .resolved
            .get(node)
            .ok_or(NoSymbolFound::error(name_span))?;
        let mut symbol = symbol.borrow_mut();
        symbol.type_ = Some(type_.clone());

        let last = self.current_function.clone();
        self.current_function = Some(type_);
        node.block.accept(self)?;
        self.current_function = last;

        Self::default_result()
    }

    fn visit_parameter(&mut self, node: &Parameter) -> Result {
        let name_span = node.name.span;
        let type_ = node
            .type_
            .accept(self)?
            .ok_or(NoTypeFound::error(name_span))?;

        let symbol = self
            .resolved
            .get(node)
            .ok_or(NoSymbolFound::error(name_span))?;
        let mut symbol = symbol.borrow_mut();
        symbol.type_ = Some(type_);

        Self::default_result()
    }

    fn visit_id(&mut self, node: &Id) -> Result {
        let id_span = node.id.span;
        let symbol = self
            .resolved
            .get(node)
            .ok_or(NoSymbolFound::error(id_span))?;
        let symbol = symbol.borrow_mut();

        let type_ = symbol.type_.clone().ok_or(NoTypeFound::error(id_span))?;
        Ok(Some(type_))
    }
}
