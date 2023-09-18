#![allow(unused)]

use std::collections::HashMap;

use diagnostics::report::Report;
use lasso::Spur;
use parser::{
    ast::{ExpressionKind, Literal, Program, StatementKind},
    traversel::{walk_statement, Visitable, Visitor},
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
struct Symbol {
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
enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
struct Scope {
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
    fn enter(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn exit(&mut self) -> Option<Scope> {
        self.scopes.pop()
    }

    fn is_global(&self) -> bool {
        self.scopes.len() == 1
    }

    fn insert(&mut self, name: Spur, symbol: Symbol, shadow: bool) -> Result<(), ResolutionError> {
        let scope = self
            .scopes
            .last_mut()
            .expect("There should at least be one scope (global).");
        scope.insert(name, symbol, shadow)
    }

    fn lookup(&self, name: Spur) -> Result<&Symbol, ResolutionError> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Ok(symbol);
            }
        }

        Err(ResolutionError::SymbolNotFound)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct NameResolution {
    table: SymbolTable,
    pub errors: Vec<ResolutionError>,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ResolutionError {
    Report(Report),
    SymbolNotFound,
    CannotShadow(Spur),
}

impl<'a> Visitor<'a> for NameResolution {
    type Result = Result<(), ResolutionError>;

    fn visit_program(&mut self, program: &Program) -> Self::Result {
        for statement in program.0.iter() {
            if let Err(error) = statement.accept(self) {
                self.errors.push(error);
            }
        }

        Ok(())
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Result {
        Ok(())
    }

    fn visit_statement(&mut self, statement: &StatementKind) -> Self::Result {
        walk_statement(self, statement);

        let is_global = self.table.is_global();
        match statement {
            StatementKind::LetDeclaration(name, _) => {
                let name = name.get_str().unwrap();
                let kind = if is_global {
                    SymbolKind::GlobalVar
                } else {
                    SymbolKind::LocalVar
                };

                self.table
                    .insert(name, Symbol::new(name, kind), !is_global)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_expression(&mut self, expression: &ExpressionKind) -> Self::Result {
        Ok(())
    }

    fn visit_parameter(&mut self, argument: &parser::ast::Parameter) -> Self::Result {
        Ok(())
    }
}
