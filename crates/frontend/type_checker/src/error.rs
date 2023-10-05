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
    pub fn error(
        lhs: TypeKind,
        operator: impl Into<String>,
        rhs: TypeKind,
        span: LabelSpan,
    ) -> TypeError {
        TypeError::InvalidBinaryType(InvalidBinaryType {
            rhs,
            operator: operator.into(),
            lhs,
            span,
        })
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
    pub fn error(operator: impl Into<String>, expression: TypeKind, span: LabelSpan) -> TypeError {
        TypeError::InvalidUnaryType(InvalidUnaryType {
            operator: operator.into(),
            expression,
            span,
        })
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
    pub fn error(got: Type, expected: Type) -> TypeError {
        TypeError::NotMatching(NotMatching { got, expected })
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
pub enum TypeError {
    InvalidBinaryType(InvalidBinaryType),
    InvalidUnaryType(InvalidUnaryType),
    NotMatching(NotMatching),
    InternalError(InternalError),
}

impl Reportable for TypeError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::InvalidBinaryType(error) => error.into_report(interner),
            Self::InvalidUnaryType(error) => error.into_report(interner),
            Self::NotMatching(error) => error.into_report(interner),
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
    pub fn error(span: LabelSpan) -> TypeError {
        TypeError::InternalError(InternalError::NoTypeFound(NoTypeFound { span }))
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
    pub fn error(span: LabelSpan) -> TypeError {
        TypeError::InternalError(InternalError::NoSymbolFound(NoSymbolFound { span }))
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum InternalError {
    NoTypeFound(NoTypeFound),
    NoSymbolFound(NoSymbolFound),
}

impl Reportable for InternalError {
    fn into_report(self, interner: &Rodeo) -> Report {
        match self {
            Self::NoSymbolFound(error) => error.into_report(interner),
            Self::NoTypeFound(error) => error.into_report(interner),
        }
    }
}
