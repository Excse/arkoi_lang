use std::{collections::HashMap, rc::Rc, borrow::BorrowMut, cell::RefCell};

use parser::{
    ast::{ASTNode, ExpressionKind, LiteralKind, StatementKind},
    traversel::{walk_statement, Visitor},
};

struct Symbol<'a> {
    name: &'a str,
    kind: SymbolKind,
    reference: &'a dyn ASTNode,
}

enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
}

struct SymbolTable<'a> {
    symbols: HashMap<&'a str, Symbol<'a>>,
    parent: Option<Rc<SymbolTable<'a>>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<SymbolTable<'a>>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn insert(
        &mut self,
        name: &'a str,
        kind: SymbolKind,
        reference: &'a dyn ASTNode,
    ) -> Result<&Symbol<'a>, ()> {
        let symbol = Symbol {
            name,
            kind,
            reference,
        };

        if self.symbols.get(name).is_some() {
            return Err(());
        }

        self.symbols.insert(name, symbol);
        Ok(self.symbols.get(name).unwrap())
    }
}

pub struct NameResolution<'a> {
    scopes: Vec<RefCell<SymbolTable<'a>>>,
}

pub enum ResolutionError {}

impl<'a> Visitor<'a> for NameResolution<'a> {
    type Result = Result<(), ResolutionError>;

    fn visit_literal(&mut self, literal: &LiteralKind) -> Self::Result {
        Ok(())
    }

    fn visit_statement(&mut self, statement: &StatementKind) -> Self::Result {
        // walk_statement(self, statement);

        // let current = self.current_scope();
        // match statement {
        //     StatementKind::LetDeclaration(name, _) => {
        //         let name = name.get_str().unwrap();
        //         let kind = if self.is_global_scope() {
        //             SymbolKind::GlobalVar
        //         } else {
        //             SymbolKind::LocalVar
        //         };
        //         current.borrow_mut().insert(name, kind, statement);
        //         println!("{}", name);
        //     }
        //     _ => {}
        // }

        Ok(())
    }

    fn visit_expression(&mut self, expression: &ExpressionKind) -> Self::Result {
        Ok(())
    }
}

impl<'a> NameResolution<'a> {
    pub fn new() -> Self {
        NameResolution { scopes: Vec::new() }
    }

    // fn enter_scope(&mut self) -> &SymbolTable<'a> {
    //     let current = Rc::clone(self.current_scope());
    //     let new_scope = Rc::new(SymbolTable::with_parent(current));
    //     self.scopes.push(new_scope);
    //     self.current_scope()
    // }

    // fn exit_scope(&'a mut self) -> RefCell<SymbolTable<'a>> {
    //     self.scopes
    //         .pop()
    //         .expect("Couldn't find any available scope.")
    // }

    // fn current_scope(&self) -> &Rc<SymbolTable<'a>> {
    //     self.scopes
    //         .last()
    //         .expect("Couldn't find any available scope.")
    // }

    // fn is_global_scope(&self) -> bool {
    //     self.scopes.len() == 1
    // }
}
