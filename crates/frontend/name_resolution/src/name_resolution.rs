use diagnostics::positional::LabelSpan;
#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
    rc::Rc,
};

use crate::{
    error::{InvalidSymbolKind, ResolutionError, Result},
    symbol::{Symbol, SymbolKind},
    table::SymbolTable,
};
use ast::{
    traversal::{Visitable, Visitor, Walkable},
    Block, Call, Comparison, Equality, Factor, FunDecl, Id, LetDecl, Node, Parameter, Program,
    Return, Term, Unary,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct ResolvedSymbols {
    resolved: HashMap<u64, Rc<RefCell<Symbol>>>,
}

impl ResolvedSymbols {
    pub fn insert<'a>(&mut self, span: &impl Node<'a>, symbol: Rc<RefCell<Symbol>>) {
        let mut hasher = DefaultHasher::new();
        span.hash(&mut hasher);

        let id = hasher.finish();
        self.resolved.insert(id, symbol);
    }

    pub fn get<'a>(&self, span: &impl Node<'a>) -> Option<Rc<RefCell<Symbol>>> {
        let mut hasher = DefaultHasher::new();
        span.hash(&mut hasher);

        let id = hasher.finish();
        self.resolved.get(&id).cloned()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct NameResolution {
    table: SymbolTable,
    pub resolved: ResolvedSymbols,
    pub errors: Vec<ResolutionError>,
}

impl<'a> Visitor<'a> for NameResolution {
    type Return = Option<Rc<RefCell<Symbol>>>;
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
        self.resolved.insert(node, symbol);

        result
    }

    fn visit_fun_decl(&mut self, node: &'a FunDecl) -> Result {
        let global = self.table.global_scope();

        let name = node.name.get_spur().unwrap();

        let function = SymbolKind::Function(node.block.clone());
        let symbol = Symbol::new(name, node.name.span, function);
        let symbol = global.insert(name, node.name.span, symbol, false)?;
        self.resolved.insert(node, symbol);

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

        let symbol = Symbol::new(name, node.name.span, SymbolKind::Parameter);
        let symbol = self.table.insert(name, node.name.span, symbol, false)?;
        self.resolved.insert(node, symbol);

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
        self.is_potential_function_symbol(symbol, node.span)?;

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
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_comparison(&mut self, node: &'a Comparison) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_term(&mut self, node: &'a Term) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_factor(&mut self, node: &'a Factor) -> Result {
        let lhs = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs, node.lhs.span())?;

        let rhs = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs, node.rhs.span())?;

        Self::default_result()
    }

    fn visit_unary(&mut self, node: &'a Unary) -> Result {
        let symbol = node.expression.accept(self)?;
        self.is_potential_variable_symbol(symbol, node.span)?;

        Self::default_result()
    }

    fn visit_return(&mut self, node: &'a Return) -> Result {
        if let Some(ref expression) = node.expression {
            let symbol = expression.accept(self)?;
            self.is_potential_variable_symbol(symbol, expression.span())?;
        }

        Self::default_result()
    }

    fn visit_id(&mut self, node: &'a Id) -> Result {
        let name = node.id.get_spur().unwrap();
        let symbol = self.table.lookup(name, node.id.span)?;
        self.resolved.insert(node, symbol.clone());

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
