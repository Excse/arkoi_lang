#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{rc::Rc, cell::RefCell};

use lasso::Spur;

use crate::symbol::Symbol;
use diagnostics::{
    positional::Span,
    report::{Report, Reportable},
};

pub type Result = std::result::Result<Option<Rc<RefCell<Symbol>>>, ResolutionError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct VariableCantBeAFunction;

impl VariableCantBeAFunction {
    pub fn error() -> ResolutionError {
        ResolutionError::VariableCantBeAFunction(VariableCantBeAFunction)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct VariableMustBeAFunction;

impl VariableMustBeAFunction {
    pub fn error() -> ResolutionError {
        ResolutionError::VariableMustBeAFunction(VariableMustBeAFunction)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SymbolNotFound;

impl SymbolNotFound {
    pub fn error() -> ResolutionError {
        ResolutionError::SymbolNotFound(SymbolNotFound)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NameAlreadyUsed {
    _name: Spur,
    _original: Span,
    _other: Span,
}

impl NameAlreadyUsed {
    pub fn error(name: Spur, original: Span, other: Span) -> ResolutionError {
        ResolutionError::NameAlreadyUsed(NameAlreadyUsed {
            _name: name,
            _original: original,
            _other: other,
        })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
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
