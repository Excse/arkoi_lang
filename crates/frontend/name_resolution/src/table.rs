#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lasso::Spur;

use crate::{
    error::{NameAlreadyUsed, ResolutionError, SymbolNotFound},
    symbol::Symbol,
};
use diagnostics::positional::Spanned;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct Scope {
    symbols: HashMap<Spur, Rc<RefCell<Symbol>>>,
}

impl Scope {
    pub fn insert(
        &mut self,
        name: Spanned<Spur>,
        symbol: Symbol,
        shadow: bool,
    ) -> Result<Rc<RefCell<Symbol>>, ResolutionError> {
        if !shadow {
            if let Some(other) = self.lookup(*name) {
                return Err(NameAlreadyUsed::error(*name, other.borrow().name.span, name.span));
            }
        }

        let symbol = Rc::new(RefCell::new(symbol));
        self.symbols.insert(*name, symbol.clone());
        Ok(symbol)
    }

    pub fn lookup(&self, name: Spur) -> Option<Rc<RefCell<Symbol>>> {
        self.symbols.get(&name).cloned()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        let mut table = SymbolTable { scopes: Vec::new() };
        table.enter();
        table
    }
}

impl SymbolTable {
    pub fn global_scope(&mut self) -> &mut Scope {
        self.scopes.first_mut().unwrap()
    }

    pub fn enter(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn exit(&mut self) -> Option<Scope> {
        self.scopes.pop()
    }

    pub fn is_global(&self) -> bool {
        self.scopes.len() == 1
    }

    pub fn insert(
        &mut self,
        name: Spanned<Spur>,
        symbol: Symbol,
        shadow: bool,
    ) -> Result<Rc<RefCell<Symbol>>, ResolutionError> {
        let scope = self
            .scopes
            .last_mut()
            .expect("There should at least be one scope (global).");
        scope.insert(name, symbol, shadow)
    }

    pub fn lookup(&self, name: Spur) -> Result<Rc<RefCell<Symbol>>, ResolutionError> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Ok(symbol);
            }
        }

        Err(SymbolNotFound::error())
    }
}
