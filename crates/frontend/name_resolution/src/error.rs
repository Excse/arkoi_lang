#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{cell::RefCell, rc::Rc};

use lasso::{Rodeo, Spur};

use crate::symbol::{Symbol, SymbolKind};
use diagnostics::{
    positional::LabelSpan,
    report::{LabelBuilder, Report, ReportBuilder, Reportable, Serverity},
};

pub type Result = std::result::Result<Option<Rc<RefCell<Symbol>>>, ResolutionError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct InvalidSymbolKind {
    got: SymbolKind,
    expected: String,
    span: LabelSpan,
}

impl InvalidSymbolKind {
    pub fn error(got: SymbolKind, expected: impl Into<String>, span: LabelSpan) -> ResolutionError {
        ResolutionError::InvalidSymbolKind(InvalidSymbolKind {
            got,
            expected: expected.into(),
            span,
        })
    }
}

impl Reportable for InvalidSymbolKind {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let message = format!(
            "Expected symbol of type '{}' but instead got '{}'.",
            self.expected, self.got
        );

        ReportBuilder::default()
            .message(message)
            .code(2)
            .serverity(Serverity::Bug)
            .label(LabelBuilder::default().span(self.span).build().unwrap())
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NameAlreadyUsed {
    name: Spur,
    original: LabelSpan,
    other: LabelSpan,
}

impl NameAlreadyUsed {
    pub fn error(name: Spur, original: LabelSpan, other: LabelSpan) -> ResolutionError {
        ResolutionError::NameAlreadyUsed(NameAlreadyUsed {
            name,
            original,
            other,
        })
    }
}

impl Reportable for NameAlreadyUsed {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let name = _interner.resolve(&self.name);
        let message = format!("There is already a node with the same name '{}'.", name);

        ReportBuilder::default()
            .message(message)
            .code(2)
            .serverity(Serverity::Bug)
            .label(
                LabelBuilder::default()
                    .message("First occurance")
                    .span(self.original)
                    .build()
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .message("Second occurance")
                    .span(self.original)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum ResolutionError {
    InvalidSymbolKind(InvalidSymbolKind),
    NameAlreadyUsed(NameAlreadyUsed),
    InternalError(InternalError),
}

impl Reportable for ResolutionError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::InvalidSymbolKind(error) => error.into_report(interner),
            Self::NameAlreadyUsed(error) => error.into_report(interner),
            Self::InternalError(error) => error.into_report(interner),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct SymbolNotFound {
    span: LabelSpan,
}

impl SymbolNotFound {
    pub fn error(span: LabelSpan) -> ResolutionError {
        ResolutionError::InternalError(InternalError::SymbolNotFound(SymbolNotFound { span }))
    }
}

impl Reportable for SymbolNotFound {
    fn into_report(self, _interner: &Rodeo) -> Report {
        ReportBuilder::default()
            .message("Couldn't find a symbol for this node.")
            .code(2)
            .serverity(Serverity::Bug)
            .label(
                LabelBuilder::default()
                    .message("No symbol found for this")
                    .span(self.span)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum InternalError {
    SymbolNotFound(SymbolNotFound),
}

impl Reportable for InternalError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::SymbolNotFound(error) => error.into_report(interner),
        }
    }
}
