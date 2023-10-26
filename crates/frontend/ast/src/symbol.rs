#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{
    cell::{OnceCell, RefCell},
    fmt::{Display, Formatter},
    rc::Rc,
};

use lasso::Spur;

use diagnostics::positional::LabelSpan;

use crate::{FunDecl, Type};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
    Function(Rc<RefCell<FunDecl>>),
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalVar => write!(f, "local variable"),
            Self::GlobalVar => write!(f, "global variable"),
            Self::Parameter => write!(f, "parameter"),
            Self::Function(_) => write!(f, "function"),
        }
    }
}

impl PartialEq for SymbolKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LocalVar, Self::LocalVar) => true,
            (Self::GlobalVar, Self::GlobalVar) => true,
            (Self::Parameter, Self::Parameter) => true,
            (Self::Function(first), Self::Function(second)) => Rc::ptr_eq(first, second),
            _ => false,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: Spur,
    pub kind: SymbolKind,
    #[serde(skip)]
    pub type_: OnceCell<Type>,
    pub span: LabelSpan,
}

impl Symbol {
    pub fn new(name: Spur, span: LabelSpan, kind: SymbolKind) -> Self {
        Self {
            name,
            span,
            kind,
            type_: OnceCell::new(),
        }
    }
}
