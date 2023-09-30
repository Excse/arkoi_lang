#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Spur;

use ast::BlockNode;
use diagnostics::positional::Spannable;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
    // TODO: Find a way to prevent the clone
    Function(BlockNode),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: Spannable<Spur>,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn new(name: Spannable<Spur>, kind: SymbolKind) -> Self {
        Symbol { name, kind }
    }
}
