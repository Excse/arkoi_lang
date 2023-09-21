use std::rc::Rc;

use lasso::Spur;

use ast::symbol::Symbol;
use diagnostics::{
    file::Files,
    positional::Span,
    report::{Report, Reportable},
};

pub type Result = std::result::Result<Option<Rc<Symbol>>, ResolutionError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct VariableCantBeAFunction;

impl VariableCantBeAFunction {
    pub fn error() -> ResolutionError {
        ResolutionError::VariableCantBeAFunction(VariableCantBeAFunction)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct VariableMustBeAFunction;

impl VariableMustBeAFunction {
    pub fn error() -> ResolutionError {
        ResolutionError::VariableMustBeAFunction(VariableMustBeAFunction)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct SymbolNotFound;

impl SymbolNotFound {
    pub fn error() -> ResolutionError {
        ResolutionError::SymbolNotFound(SymbolNotFound)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct NameAlreadyUsed {
    name: Spur,
    original: Span,
    other: Span,
}

impl NameAlreadyUsed {
    pub fn error(name: Spur, original: Span, other: Span) -> ResolutionError {
        ResolutionError::NameAlreadyUsed(NameAlreadyUsed {
            name,
            original,
            other,
        })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ResolutionError {
    VariableCantBeAFunction(VariableCantBeAFunction),
    VariableMustBeAFunction(VariableMustBeAFunction),
    SymbolNotFound(SymbolNotFound),
    NameAlreadyUsed(NameAlreadyUsed),
}

impl Reportable for ResolutionError {
    fn into_report(self) -> Report {
        match self {
            Self::VariableCantBeAFunction(error) => todo!("{:?}", error),
            Self::VariableMustBeAFunction(error) => todo!("{:?}", error),
            Self::SymbolNotFound(error) => todo!("{:?}", error),
            Self::NameAlreadyUsed(error) => todo!("{:?}", error),
        }
    }
}
