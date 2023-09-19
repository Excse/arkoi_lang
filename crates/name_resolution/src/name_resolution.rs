use lasso::Spur;

use crate::{
    error::{ResolutionError, Result},
    symbol_table::SymbolTable,
};
use ast::{
    symbol::{Symbol, SymbolKind},
    traversal::{Visitable, Visitor},
    BlockNode, CallNode, ExpressionKind, FunDeclarationNode, LetDeclarationNode, LiteralKind,
    ParameterNode, ProgramNode, StatementKind, VariableNode,
};
use diagnostics::{
    positional::{Span, Spannable},
    report::Report,
};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct NameResolution {
    table: SymbolTable,
    pub errors: Vec<ResolutionError>,
}

impl<'a> Visitor<'a> for NameResolution {
    type Return = ();
    type Error = ResolutionError;

    fn default_result() -> Result {
        Ok(())
    }

    fn visit_program(&mut self, node: &'a mut ProgramNode) -> Result {
        node.statements
            .iter_mut()
            .for_each(|statement| match self.visit_statement(statement) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Ok(())
    }

    fn visit_let_declaration(&mut self, node: &'a mut LetDeclarationNode) -> Result {
        let should_shadow = !self.table.is_global();

        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let kind = if should_shadow {
            SymbolKind::GlobalVar
        } else {
            SymbolKind::LocalVar
        };

        let symbol = Symbol::new(name.clone(), kind);
        self.table.insert(name.clone(), symbol, should_shadow)?;

        node.type_.accept(self)?;

        if let Some(ref mut expression) = node.expression {
            expression.accept(self)?;
        }

        Ok(())
    }

    fn visit_fun_declaration(&mut self, node: &'a mut FunDeclarationNode) -> Result {
        let mut global = self.table.global_scope();

        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let symbol = Symbol::new(name.clone(), SymbolKind::Function);
        global.insert(name.clone(), symbol, false)?;

        self.table.enter();

        node.type_.accept(self)?;

        node.parameters
            .iter_mut()
            .for_each(|parameter| match parameter.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        node.block.accept(self)?;

        self.table.exit();

        Ok(())
    }

    fn visit_parameter(&mut self, node: &'a mut ParameterNode) -> Result {
        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let symbol = Symbol::new(name.clone(), SymbolKind::Parameter);
        self.table.insert(name.clone(), symbol, false)?;

        node.type_.accept(self)?;

        Ok(())
    }

    fn visit_block(&mut self, node: &'a mut BlockNode) -> Result {
        self.table.enter();

        node.statements
            .iter_mut()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        self.table.exit();

        Ok(())
    }

    fn visit_variable(&mut self, node: &'a mut VariableNode) -> Result {
        let name = node.identifier.get_spur().unwrap();
        let symbol = self.table.lookup(name)?;

        if node.is_function && symbol.kind != SymbolKind::Function {
            return Err(ResolutionError::VariableMustBeAFunction);
        } else if !node.is_function && symbol.kind == SymbolKind::Function {
            return Err(ResolutionError::VariableCantBeAFunction);
        }

        Ok(())
    }
}
