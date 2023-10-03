#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, rc::Rc};

use lasso::Spur;

use crate::symbol::Symbol;
use diagnostics::{
    positional::LabelSpan,
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
pub struct NameAlreadyUsed {
    _name: Spur,
    _original: LabelSpan,
    _other: LabelSpan,
}

impl NameAlreadyUsed {
    pub fn error(name: Spur, original: LabelSpan, other: LabelSpan) -> ResolutionError {
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
    NameAlreadyUsed(NameAlreadyUsed),
    InternalError(InternalError),
}

impl Reportable for ResolutionError {
    fn into_report(self) -> Report {
        match self {
            Self::VariableCantBeAFunction(error) => todo!("{:?}", error),
            Self::VariableMustBeAFunction(error) => todo!("{:?}", error),
            Self::NameAlreadyUsed(error) => todo!("{:?}", error),
            Self::InternalError(error) => panic!("Internal error: {:#?}", error),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SymbolNotFound;

impl SymbolNotFound {
    pub fn error() -> ResolutionError {
        ResolutionError::InternalError(InternalError::SymbolNotFound(SymbolNotFound))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum InternalError {
    SymbolNotFound(SymbolNotFound),
}
