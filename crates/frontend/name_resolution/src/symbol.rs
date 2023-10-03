#[cfg(feature = "serialize")]
use serde::Serialize;

use std::rc::Rc;

use lasso::Spur;

use ast::{Block, Type};
use diagnostics::positional::Spanned;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
    // TODO: Find a way to prevent the clone
    Function(Rc<Block>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: Spanned<Spur>,
    pub kind: SymbolKind,
    pub type_: Option<Type>,
}

impl Symbol {
    pub fn new(name: Spanned<Spur>, kind: SymbolKind) -> Self {
        Symbol {
            name,
            kind,
            type_: None,
        }
    }
}
