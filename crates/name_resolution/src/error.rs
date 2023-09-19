use lasso::Spur;

use diagnostics::{positional::Span, report::Report};

pub type Result = std::result::Result<(), ResolutionError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ResolutionError {
    Report(Report),
    VariableCantBeAFunction,
    VariableMustBeAFunction,
    SymbolNotFound,
    NameAlreadyUsed(Spur, Span, Span),
}
