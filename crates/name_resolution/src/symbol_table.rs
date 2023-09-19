use std::collections::HashMap;

use diagnostics::positional::Spannable;
use lasso::Spur;

use crate::name_resolution::ResolutionError;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Symbol {
    pub name: Spannable<Spur>,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn new(name: Spannable<Spur>, kind: SymbolKind) -> Self {
        Symbol { name, kind }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
    Function,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct Scope {
    symbols: HashMap<Spur, Symbol>,
}

impl Scope {
    pub fn insert(
        &mut self,
        name: Spannable<Spur>,
        symbol: Symbol,
        shadow: bool,
    ) -> Result<(), ResolutionError> {
        if !shadow {
            if let Some(other) = self.lookup(name.content) {
                return Err(ResolutionError::NameAlreadyUsed(
                    name.content,
                    other.name.span,
                    name.span,
                ));
            }
        }

        self.symbols.insert(name.content, symbol);
        Ok(())
    }

    pub fn lookup(&self, name: Spur) -> Option<&Symbol> {
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
        name: Spannable<Spur>,
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
