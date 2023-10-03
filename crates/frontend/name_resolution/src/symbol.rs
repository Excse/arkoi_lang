#[cfg(feature = "serialize")]
use serde::Serialize;

use std::rc::Rc;

use lasso::Spur;

use ast::{Block, Type};
use diagnostics::positional::LabelSpan;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
    // TODO: Find a way to prevent the clone
    Function(Rc<Block>),
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
    pub type_: Option<Type>,
    pub span: LabelSpan,
}

impl Symbol {
    pub fn new(name: Spur, span: LabelSpan, kind: SymbolKind) -> Self {
        Symbol {
            name,
            span,
            kind,
            type_: None,
        }
    }
}
