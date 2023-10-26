use name_resolution::error::InvalidSymbolKind;
#[cfg(feature = "serialize")]
use serde::Serialize;

use lasso::Rodeo;

use ast::{Type, TypeKind};
use diagnostics::{
    positional::LabelSpan,
    report::{LabelBuilder, Report, ReportBuilder, Reportable, Serverity},
};

pub type Result = std::result::Result<Option<Type>, TypeError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct InvalidBinaryType {
    lhs: TypeKind,
    operator: String,
    rhs: TypeKind,
    span: LabelSpan,
}

impl InvalidBinaryType {
    pub fn new(lhs: TypeKind, operator: impl Into<String>, rhs: TypeKind, span: LabelSpan) -> Self {
        Self {
            rhs,
            operator: operator.into(),
            lhs,
            span,
        }
    }
}

impl From<InvalidBinaryType> for TypeError {
    fn from(value: InvalidBinaryType) -> Self {
        Self::InvalidBinaryType(value)
    }
}

impl Reportable for InvalidBinaryType {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message = format!(
            "There is no binary operator that supports: {} {} {}",
            self.lhs, self.operator, self.rhs
        );

        ReportBuilder::default()
            .message(report_message)
            .code(1)
            .serverity(Serverity::Error)
            .label(LabelBuilder::default().span(self.span).build().unwrap())
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct InvalidUnaryType {
    operator: String,
    expression: TypeKind,
    span: LabelSpan,
}

impl InvalidUnaryType {
    pub fn new(operator: impl Into<String>, expression: TypeKind, span: LabelSpan) -> Self {
        Self {
            operator: operator.into(),
            expression,
            span,
        }
    }
}

impl From<InvalidUnaryType> for TypeError {
    fn from(value: InvalidUnaryType) -> Self {
        Self::InvalidUnaryType(value)
    }
}

impl Reportable for InvalidUnaryType {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message = format!(
            "There is no unary operator that supports: {} {}",
            self.operator, self.expression
        );

        ReportBuilder::default()
            .message(report_message)
            .code(1)
            .serverity(Serverity::Error)
            .label(LabelBuilder::default().span(self.span).build().unwrap())
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NotMatching {
    expected: Type,
    got: Type,
}

impl NotMatching {
    pub fn new(got: Type, expected: Type) -> Self {
        Self { got, expected }
    }
}

impl From<NotMatching> for TypeError {
    fn from(value: NotMatching) -> Self {
        Self::NotMatching(value)
    }
}

impl Reportable for NotMatching {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message = format!(
            "Expected to find the type '{}' but instead got '{}'",
            self.expected.kind, self.got.kind
        );

        let instead_message = format!(
            "Got '{}' but instead expected '{}'",
            self.got.kind, self.expected.kind
        );

        ReportBuilder::default()
            .message(report_message)
            .code(1)
            .serverity(Serverity::Error)
            .label(
                LabelBuilder::default()
                    .message(instead_message)
                    .span(self.got.span)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct InvalidArity {
    expected: usize,
    expected_span: LabelSpan,
    got: usize,
    got_span: LabelSpan,
}

impl InvalidArity {
    pub fn new(got: usize, got_span: LabelSpan, expected: usize, expected_span: LabelSpan) -> Self {
        Self {
            got,
            got_span,
            expected,
            expected_span,
        }
    }
}

impl From<InvalidArity> for TypeError {
    fn from(value: InvalidArity) -> Self {
        Self::InvalidArity(value)
    }
}

impl Reportable for InvalidArity {
    fn into_report(self, _interner: &Rodeo) -> Report {
        let report_message =
            "The amount of arguments provided doesn't match with the arity of this function.";

        let expected_message = format!("This function expected '{}' arguments.", self.expected);
        let got_message = format!("But instead got '{}' arguments.", self.got);

        ReportBuilder::default()
            .message(report_message)
            .code(1)
            .serverity(Serverity::Error)
            .label(
                LabelBuilder::default()
                    .message(expected_message)
                    .span(self.expected_span)
                    .build()
                    .unwrap(),
            )
            .label(
                LabelBuilder::default()
                    .message(got_message)
                    .span(self.got_span)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidBinaryType(InvalidBinaryType),
    InvalidUnaryType(InvalidUnaryType),
    NotMatching(NotMatching),
    InvalidArity(InvalidArity),
    InternalError(InternalError),
}

impl Reportable for TypeError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::InvalidBinaryType(error) => error.into_report(interner),
            Self::InvalidUnaryType(error) => error.into_report(interner),
            Self::NotMatching(error) => error.into_report(interner),
            Self::InvalidArity(error) => error.into_report(interner),
            Self::InternalError(error) => error.into_report(interner),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NoTypeFound {
    span: LabelSpan,
}

impl NoTypeFound {
    pub fn new(span: LabelSpan) -> Self {
        Self { span }
    }
}

impl From<NoTypeFound> for TypeError {
    fn from(value: NoTypeFound) -> Self {
        Self::InternalError(InternalError::NoTypeFound(value))
    }
}

impl Reportable for NoTypeFound {
    fn into_report(self, _interner: &Rodeo) -> Report {
        ReportBuilder::default()
            .message("Couldn't find a type for this node:")
            .code(1)
            .serverity(Serverity::Bug)
            .label(LabelBuilder::default().span(self.span).build().unwrap())
            .build()
            .unwrap()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NoSymbolFound {
    span: LabelSpan,
}

impl NoSymbolFound {
    pub fn new(span: LabelSpan) -> Self {
        Self { span }
    }
}

impl From<NoSymbolFound> for TypeError {
    fn from(value: NoSymbolFound) -> Self {
        Self::InternalError(InternalError::NoSymbolFound(value))
    }
}

impl Reportable for NoSymbolFound {
    fn into_report(self, _interner: &Rodeo) -> Report {
        ReportBuilder::default()
            .message("Couldn't find a symbol for this node:")
            .code(1)
            .serverity(Serverity::Bug)
            .label(LabelBuilder::default().span(self.span).build().unwrap())
            .build()
            .unwrap()
    }
}

impl From<InvalidSymbolKind> for TypeError {
    fn from(value: InvalidSymbolKind) -> Self {
        Self::InternalError(InternalError::InvalidSymbolKind(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum InternalError {
    NoTypeFound(NoTypeFound),
    NoSymbolFound(NoSymbolFound),
    InvalidSymbolKind(InvalidSymbolKind),
}

impl Reportable for InternalError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::NoSymbolFound(error) => error.into_report(interner),
            Self::NoTypeFound(error) => error.into_report(interner),
            Self::InvalidSymbolKind(error) => error.into_report(interner),
        }
    }
}
