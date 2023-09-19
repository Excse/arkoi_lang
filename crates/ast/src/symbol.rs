#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Spur;

use diagnostics::positional::Spannable;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    LocalVar,
    GlobalVar,
    Parameter,
    Function,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Symbol {
    pub name: Spannable<Spur>,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn new(name: Spannable<Spur>, kind: SymbolKind) -> Self {
        Symbol { name, kind }
    }
}
