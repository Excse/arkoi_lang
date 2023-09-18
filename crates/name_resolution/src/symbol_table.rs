use std::collections::HashMap;

use lasso::Spur;

use crate::name_resolution::ResolutionError;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Symbol {
    name: Spur,
    kind: SymbolKind,
}

impl Symbol {
    pub fn new(name: Spur, kind: SymbolKind) -> Self {
        Symbol { name, kind }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct Scope {
    symbols: HashMap<Spur, Symbol>,
}

impl Scope {
    fn insert(&mut self, name: Spur, symbol: Symbol, shadow: bool) -> Result<(), ResolutionError> {
        if !shadow && self.lookup(name).is_some() {
            // TODO: Pass symbol and other into the CannotShadow to give a more clearer report
            return Err(ResolutionError::CannotShadow(name));
        }

        self.symbols.insert(name, symbol);
        Ok(())
    }

    fn lookup(&self, name: Spur) -> Option<&Symbol> {
        self.symbols.get(&name)
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
        name: Spur,
        symbol: Symbol,
        shadow: bool,
    ) -> Result<(), ResolutionError> {
        let scope = self
            .scopes
            .last_mut()
            .expect("There should at least be one scope (global).");
        scope.insert(name, symbol, shadow)
    }

    pub fn lookup(&self, name: Spur) -> Result<&Symbol, ResolutionError> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Ok(symbol);
            }
        }

        Err(ResolutionError::SymbolNotFound)
    }
}
