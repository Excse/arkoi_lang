#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, rc::Rc};

use crate::{
    error::{InvalidSymbolKind, ResolutionError, Result},
    table::SymbolTable,
};
use ast::{
    symbol::{Symbol, SymbolKind},
    traversal::{Visitable, Visitor, Walkable},
    Block, Call, Comparison, Equality, Factor, FunDecl, Id, LetDecl, Parameter, Program, Return,
    Term, Unary,
};
use diagnostics::positional::LabelSpan;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct NameResolution {
    table: SymbolTable,
    pub errors: Vec<ResolutionError>,
}

impl Visitor for NameResolution {
    type Return = Option<Rc<RefCell<Symbol>>>;
    type Error = ResolutionError;

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

    fn visit_let_decl(&mut self, node: &mut LetDecl) -> Result {
        let should_shadow = !self.table.is_global();

        let name = node.name.get_spur().unwrap();

        let kind = if should_shadow {
            SymbolKind::GlobalVar
        } else {
            SymbolKind::LocalVar
        };

        let result = node.walk(self);

        let symbol = Symbol::new(name, node.name.span, kind);
        let symbol = self
            .table
            .insert(name, node.name.span, symbol, should_shadow)?;
        node.symbol = Some(symbol);

        result
    }

    fn visit_fun_decl(&mut self, node: Rc<RefCell<FunDecl>>) -> Result {
        let global = self.table.global_scope();

        let function = SymbolKind::Function(node.clone());

        let name = node.borrow().name.get_spur().unwrap();
        let symbol = Symbol::new(name, node.borrow().name.span, function);
        let symbol = global.insert(name, node.borrow().name.span, symbol, false)?;

        node.borrow_mut().symbol = Some(symbol);

        self.table.enter();

        node.borrow_mut()
            .parameters
            .iter_mut()
            .for_each(|parameter| match parameter.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        node.borrow_mut().type_.accept(self)?;

        node.borrow_mut().block.accept(self)?;

        self.table.exit();

        Self::default_result()
    }

    fn visit_parameter(&mut self, node: &mut Parameter) -> Result {
        let name = node.name.get_spur().unwrap();

        let symbol = Symbol::new(name, node.name.span, SymbolKind::Parameter);
        let symbol = self.table.insert(name, node.name.span, symbol, false)?;

        node.symbol = Some(symbol);

        node.walk(self)
    }

    fn visit_block(&mut self, node: &mut Block) -> Result {
        self.table.enter();

        node.statements
            .iter_mut()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        self.table.exit();

        Self::default_result()
    }

    fn visit_call(&mut self, node: &mut Call) -> Result {
        let symbol = node.callee.accept(self)?;
        self.is_potential_function_symbol(symbol, node.span)?;

        node.arguments
            .iter_mut()
            .for_each(|argument| match argument.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_equality(&mut self, node: &mut Equality) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_comparison(&mut self, node: &mut Comparison) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_term(&mut self, node: &mut Term) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_factor(&mut self, node: &mut Factor) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_unary(&mut self, node: &mut Unary) -> Result {
        let symbol = node.expression.accept(self)?;
        self.is_potential_variable_symbol(symbol, node.span)?;

        Self::default_result()
    }

    fn visit_return(&mut self, node: &mut Return) -> Result {
        if let Some(ref mut expression) = node.expression {
            let symbol = expression.accept(self)?;
            self.is_potential_variable_symbol(symbol, expression.span())?;
        }

        Self::default_result()
    }

    fn visit_id(&mut self, node: &mut Id) -> Result {
        let name = node.id.get_spur().unwrap();
        let symbol = self.table.lookup(name, node.id.span)?;

        node.symbol = Some(symbol.clone());

        Ok(Some(symbol))
    }
}

impl NameResolution {
    fn is_potential_function_symbol(
        &self,
        symbol: Option<Rc<RefCell<Symbol>>>,
        span: LabelSpan,
    ) -> std::result::Result<(), ResolutionError> {
        let symbol = match symbol {
            Some(symbol) => symbol,
            None => return Ok(()),
        };

        let kind = symbol.borrow().kind.clone();
        match kind {
            SymbolKind::Function(_) => Ok(()),
            _ => Err(InvalidSymbolKind::error(kind, "function", span)),
        }
    }

    fn is_potential_variable_symbol(
        &self,
        symbol: Option<Rc<RefCell<Symbol>>>,
        span: LabelSpan,
    ) -> std::result::Result<(), ResolutionError> {
        let symbol = match symbol {
            Some(symbol) => symbol,
            None => return Ok(()),
        };

        let kind = symbol.borrow().kind.clone();
        match kind {
            SymbolKind::LocalVar | SymbolKind::GlobalVar | SymbolKind::Parameter => Ok(()),
            _ => Err(InvalidSymbolKind::error(kind, "variable/parameter", span)),
        }
    }
}
