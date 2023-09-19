use diagnostics::report::Report;
use lasso::Spur;
use parser::{
    ast::{ExpressionKind, LiteralKind, ParameterNode, ProgramNode, StatementKind},
    traversel::{Visitor, Walkable},
};

use crate::symbol_table::{Symbol, SymbolKind, SymbolTable};

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

    fn default_result() -> Self::Result {
        Ok(())
    }

    fn visit_let_declaration(
        &mut self,
        node: &'a mut parser::ast::LetDeclarationNode,
    ) -> Self::Result {
        let is_global = self.table.is_global();

        let name = node.name.get_str().unwrap();
        let kind = if is_global {
            SymbolKind::GlobalVar
        } else {
            SymbolKind::LocalVar
        };

        self.table
            .insert(name, Symbol::new(name, kind), !is_global)?;

        Ok(())
    }
}
