#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{collections::HashMap, rc::Rc};

use crate::{
    error::{ResolutionError, Result, VariableMustBeAFunction},
    symbol::{Symbol, SymbolKind},
    table::SymbolTable,
};
use ast::{
    traversal::{Visitable, Visitor, Walkable},
    Block, Call, Comparison, Equality, Factor, FunDecl, Id,
    LetDecl, Parameter, Program, Return, Term, Unary,
};
use diagnostics::positional::{Span, Spannable, Spanned};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct ResolvedSymbols {
    resolved: HashMap<Span, Rc<Symbol>>,
}

impl ResolvedSymbols {
    pub fn insert(&mut self, span: impl Into<Span>, symbol: Rc<Symbol>) {
        let span = span.into();

        self.resolved.insert(span, symbol);
    }

    pub fn get(&self, span: impl Into<Span>) -> Option<Rc<Symbol>> {
        let span = span.into();

        self.resolved.get(&span).cloned()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct NameResolution {
    table: SymbolTable,
    resolved: ResolvedSymbols,
    pub errors: Vec<ResolutionError>,
}

impl<'a> Visitor<'a> for NameResolution {
    type Return = Option<Rc<Symbol>>;
    type Error = ResolutionError;

    fn default_result() -> Result {
        Ok(None)
    }

    fn visit_program(&mut self, node: &'a Program) -> Result {
        node.statements
            .iter()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_let_decl(&mut self, node: &'a LetDecl) -> Result {
        let should_shadow = !self.table.is_global();

        let name = node.name.get_spur().unwrap();
        let name = Spanned::new(name, *node.name.span());

        let kind = if should_shadow {
            SymbolKind::GlobalVar
        } else {
            SymbolKind::LocalVar
        };

        let result = node.walk(self);

        let symbol = Symbol::new(name.clone(), kind);
        let symbol = self.table.insert(name.clone(), symbol, should_shadow)?;
        self.resolved.insert(name.span, symbol);

        result
    }

    fn visit_fun_decl(&mut self, node: &'a FunDecl) -> Result {
        let global = self.table.global_scope();

        let name = node.name.get_spur().unwrap();
        let name = Spanned::new(name, *node.name.span());

        let function = SymbolKind::Function(node.block.clone());
        let symbol = Symbol::new(name.clone(), function);
        let symbol = global.insert(name.clone(), symbol, false)?;
        self.resolved.insert(name.span, symbol);

        self.table.enter();

        node.parameters
            .iter()
            .for_each(|parameter| match parameter.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        node.type_.accept(self)?;

        node.block.accept(self)?;

        self.table.exit();

        Self::default_result()
    }

    fn visit_parameter(&mut self, node: &'a Parameter) -> Result {
        let name = node.name.get_spur().unwrap();
        let name = Spanned::new(name, *node.name.span());

        let symbol = Symbol::new(name.clone(), SymbolKind::Parameter);
        let symbol = self.table.insert(name.clone(), symbol, false)?;
        self.resolved.insert(name.span, symbol);

        node.walk(self)
    }

    fn visit_block(&mut self, node: &'a Block) -> Result {
        self.table.enter();

        node.statements
            .iter()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        self.table.exit();

        Self::default_result()
    }

    fn visit_call(&mut self, node: &'a Call) -> Result {
        let symbol = node.callee.accept(self)?;
        self.is_potential_function_symbol(symbol)?;

        node.arguments
            .iter()
            .for_each(|argument| match argument.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_equality(&mut self, node: &'a Equality) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs)?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs)?;

        Self::default_result()
    }

    fn visit_comparison(&mut self, node: &'a Comparison) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs)?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs)?;

        Self::default_result()
    }

    fn visit_term(&mut self, node: &'a Term) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs)?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs)?;

        Self::default_result()
    }

    fn visit_factor(&mut self, node: &'a Factor) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs)?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs)?;

        Self::default_result()
    }

    fn visit_unary(&mut self, node: &'a Unary) -> Result {
        let symbol = node.expression.accept(self)?;
        self.is_potential_variable_symbol(symbol)?;

        Self::default_result()
    }

    fn visit_return(&mut self, node: &'a Return) -> Result {
        if let Some(ref expression) = node.expression {
            let symbol = expression.accept(self)?;
            self.is_potential_variable_symbol(symbol)?;
        }

        Self::default_result()
    }

    fn visit_id(&mut self, node: &'a Id) -> Result {
        let name = node.id.get_spur().unwrap();
        let symbol = self.table.lookup(name)?;
        self.resolved.insert(*node.id.span(), symbol.clone());

        Ok(Some(symbol))
    }
}

impl NameResolution {
    fn is_potential_function_symbol(
        &self,
        symbol: Option<Rc<Symbol>>,
    ) -> std::result::Result<(), ResolutionError> {
        let symbol = match symbol {
            Some(symbol) => symbol,
            None => return Ok(()),
        };

        match symbol.kind {
            SymbolKind::Function(_) => Ok(()),
            _ => Err(VariableMustBeAFunction::error()),
        }
    }

    fn is_potential_variable_symbol(
        &self,
        symbol: Option<Rc<Symbol>>,
    ) -> std::result::Result<(), ResolutionError> {
        let symbol = match symbol {
            Some(symbol) => symbol,
            None => return Ok(()),
        };

        match symbol.kind {
            SymbolKind::LocalVar | SymbolKind::GlobalVar | SymbolKind::Parameter => Ok(()),
            _ => Err(VariableMustBeAFunction::error()),
        }
    }
}
