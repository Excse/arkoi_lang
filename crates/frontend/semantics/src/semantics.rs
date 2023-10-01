#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::error::SemanticError;
use ast::{traversal::Visitable, Program};
use name_resolution::NameResolution;
use type_checker::TypeChecker;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Semantics<'a> {
    program: &'a Program,
    pub errors: Vec<SemanticError>,
}

impl<'a> Semantics<'a> {
    pub fn new(program: &'a Program) -> Self {
        Semantics {
            program,
            errors: Vec::new(),
        }
    }

    pub fn run_all(&mut self) {
        let mut name_resolution = NameResolution::default();
        let _ = self.program.accept(&mut name_resolution);

        if !name_resolution.errors.is_empty() {
            return self.errors.extend(
                name_resolution
                    .errors
                    .iter()
                    .map(|error| SemanticError::NameResolution(error.clone())),
            );
        }

        let mut type_checker = TypeChecker::default();
        let _ = self.program.accept(&mut type_checker);

        if !type_checker.errors.is_empty() {
            return self.errors.extend(
                type_checker
                    .errors
                    .iter()
                    .map(|error| SemanticError::TypeChecker(error.clone())),
            );
        }
    }
}
