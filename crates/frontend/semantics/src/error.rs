#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::report::{Report, Reportable};
use name_resolution::error::ResolutionError;
use type_checker::error::TypeError;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum SemanticError {
    NameResolution(ResolutionError),
    TypeChecker(TypeError),
}

impl Reportable for SemanticError {
    fn into_report(self) -> Report {
        match self {
            Self::NameResolution(error) => error.into_report(),
            Self::TypeChecker(error) => error.into_report(),
        }
    }
}
