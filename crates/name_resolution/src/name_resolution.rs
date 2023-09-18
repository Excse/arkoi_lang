use diagnostics::report::Report;
use lasso::Spur;
use parser::{
    ast::{ExpressionKind, Literal, Parameter, Program, StatementKind},
    traversel::{walk_statement, Visitable, Visitor},
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

    fn visit_parameter(&mut self, argument: &Parameter) -> Self::Result {
        Ok(())
    }
}
