use lasso::Spur;

use crate::symbol_table::SymbolTable;
use ast::{
    symbol::{Symbol, SymbolKind},
    traversal::{Visitor, Walkable},
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ResolutionError {
    Report(Report),
    VariableCantBeAFunction,
    VariableMustBeAFunction,
    SymbolNotFound,
    NameAlreadyUsed(Spur, Span, Span),
}

impl<'a> Visitor<'a> for NameResolution {
    type Return = ();
    type Error = ResolutionError;

    fn default_result() -> Result<Self::Return, Self::Error> {
        Ok(())
    }

    fn visit_program(&mut self, node: &'a mut ProgramNode) -> Result<Self::Return, Self::Error> {
        node.statements
            .iter_mut()
            .for_each(|statement| match statement.walk(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Ok(())
    }

    fn visit_let_declaration(
        &mut self,
        node: &'a mut LetDeclarationNode,
    ) -> Result<Self::Return, Self::Error> {
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

        node.type_.walk(self);

        if let Some(ref mut expression) = node.expression {
            expression.walk(self);
        }

        Ok(())
    }

    fn visit_fun_declaration(
        &mut self,
        node: &'a mut FunDeclarationNode,
    ) -> Result<Self::Return, Self::Error> {
        let mut global = self.table.global_scope();

        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let symbol = Symbol::new(name.clone(), SymbolKind::Function);
        global.insert(name.clone(), symbol, false)?;

        self.table.enter();

        node.type_.walk(self);

        node.parameters
            .iter_mut()
            .for_each(|parameter| match parameter.walk(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        node.block.walk(self);

        self.table.exit();

        Ok(())
    }

    fn visit_parameter(
        &mut self,
        node: &'a mut ParameterNode,
    ) -> Result<Self::Return, Self::Error> {
        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let symbol = Symbol::new(name.clone(), SymbolKind::Parameter);
        self.table.insert(name.clone(), symbol, false)?;

        node.type_.walk(self);

        Ok(())
    }

    fn visit_block(&mut self, node: &'a mut BlockNode) -> Result<Self::Return, Self::Error> {
        self.table.enter();

        node.statements
            .iter_mut()
            .for_each(|statement| match statement.walk(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        self.table.exit();

        Ok(())
    }

    fn visit_variable(&mut self, node: &'a mut VariableNode) -> Result<Self::Return, Self::Error> {
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
