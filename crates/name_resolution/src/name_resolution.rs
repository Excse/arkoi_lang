use std::rc::Rc;

use crate::{
    error::{ResolutionError, Result, VariableMustBeAFunction},
    symbol_table::SymbolTable,
};
use ast::{
    symbol::{Symbol, SymbolKind},
    traversal::{Visitable, Visitor, Walkable},
    BlockNode, CallNode, ComparisonNode, EqualityNode, FactorNode, FunDeclarationNode,
    LetDeclarationNode, ParameterNode, ProgramNode, ReturnNode, TermNode, UnaryNode, VariableNode,
};
use diagnostics::positional::Spannable;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct NameResolution {
    table: SymbolTable,
    pub errors: Vec<ResolutionError>,
}

impl<'a> Visitor<'a> for NameResolution {
    type Return = Option<Rc<Symbol>>;
    type Error = ResolutionError;

    fn default_result() -> Result {
        Ok(None)
    }

    fn visit_program(&mut self, node: &'a mut ProgramNode) -> Result {
        node.statements
            .iter_mut()
            .for_each(|statement| match statement.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
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

        let result = node.walk(self);

        let symbol = Rc::new(Symbol::new(name.clone(), kind));
        self.table
            .insert(name.clone(), symbol.clone(), should_shadow)?;
        node.symbol = Some(symbol);

        result
    }

    fn visit_fun_declaration(&mut self, node: &'a mut FunDeclarationNode) -> Result {
        let global = self.table.global_scope();

        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let symbol = Rc::new(Symbol::new(name.clone(), SymbolKind::Function));
        global.insert(name.clone(), symbol.clone(), false)?;
        node.symbol = Some(symbol);

        self.table.enter();

        node.parameters
            .iter_mut()
            .for_each(|parameter| match parameter.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        node.type_.accept(self)?;

        node.block.accept(self)?;

        self.table.exit();

        Self::default_result()
    }

    fn visit_parameter(&mut self, node: &'a mut ParameterNode) -> Result {
        let name = node.name.get_spur().unwrap();
        let name = Spannable::new(name, node.name.span);

        let symbol = Rc::new(Symbol::new(name.clone(), SymbolKind::Parameter));
        self.table.insert(name.clone(), symbol.clone(), false)?;
        node.symbol = Some(symbol);

        node.walk(self)
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

        Self::default_result()
    }

    fn visit_call(&mut self, node: &'a mut CallNode) -> Result {
        let symbol = node.callee.accept(self)?;
        self.is_potential_function_symbol(symbol)?;

        node.arguments
            .iter_mut()
            .for_each(|argument| match argument.accept(self) {
                Ok(_) => {}
                Err(error) => self.errors.push(error),
            });

        Self::default_result()
    }

    fn visit_equality(&mut self, node: &'a mut EqualityNode) -> Result {
        let lhs_symbol = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs_symbol)?;

        let rhs_symbol = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs_symbol)?;

        Self::default_result()
    }

    fn visit_comparison(&mut self, node: &'a mut ComparisonNode) -> Result {
        let lhs_symbol = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs_symbol)?;

        let rhs_symbol = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs_symbol)?;

        Self::default_result()
    }

    fn visit_term(&mut self, node: &'a mut TermNode) -> Result {
        let lhs_symbol = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs_symbol)?;

        let rhs_symbol = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs_symbol)?;

        Self::default_result()
    }

    fn visit_factor(&mut self, node: &'a mut FactorNode) -> Result {
        let lhs_symbol = node.lhs.accept(self)?;
        self.is_potential_variable_symbol(lhs_symbol)?;

        let rhs_symbol = node.rhs.accept(self)?;
        self.is_potential_variable_symbol(rhs_symbol)?;

        Self::default_result()
    }

    fn visit_unary(&mut self, node: &'a mut UnaryNode) -> Result {
        let symbol = node.expression.accept(self)?;
        self.is_potential_variable_symbol(symbol)?;

        Self::default_result()
    }

    fn visit_return(&mut self, node: &'a mut ReturnNode) -> Result {
        if let Some(ref mut expression) = node.expression {
            let symbol = expression.accept(self)?;
            self.is_potential_variable_symbol(symbol)?;
        }

        Self::default_result()
    }

    fn visit_variable(&mut self, node: &'a mut VariableNode) -> Result {
        let name = node.identifier.get_spur().unwrap();
        let symbol = self.table.lookup(name)?;

        node.target = Some(symbol.clone());

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
            SymbolKind::Function => Ok(()),
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
