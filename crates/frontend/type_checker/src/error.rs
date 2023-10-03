#[cfg(feature = "serialize")]
use serde::Serialize;

use ast::{Type, TypeKind};
use diagnostics::{
    positional::{Span, Spanned},
    report::{Report, Reportable},
};

pub type Result = std::result::Result<Option<Type>, TypeError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct InvalidBinaryType {
    lhs: Spanned<TypeKind>,
    operator: String,
    rhs: Spanned<TypeKind>,
}

impl InvalidBinaryType {
    pub fn error(
        lhs: Spanned<TypeKind>,
        operator: impl Into<String>,
        rhs: Spanned<TypeKind>,
    ) -> TypeError {
        TypeError::InvalidBinaryType(InvalidBinaryType {
            rhs,
            operator: operator.into(),
            lhs,
        })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct InvalidUnaryType {
    operator: String,
    expression: Spanned<TypeKind>,
}

impl InvalidUnaryType {
    pub fn error(operator: impl Into<String>, expression: Spanned<TypeKind>) -> TypeError {
        TypeError::InvalidUnaryType(InvalidUnaryType {
            operator: operator.into(),
            expression,
        })
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidBinaryType(InvalidBinaryType),
    InvalidUnaryType(InvalidUnaryType),
    NotMatching(NotMatching),
    InternalError(InternalError),
}

impl Reportable for TypeError {
    fn into_report(self) -> Report {
        match self {
            Self::InvalidBinaryType(_error) => todo!("{:#?}", _error),
            Self::InvalidUnaryType(_error) => todo!("{:#?}", _error),
            Self::NotMatching(_error) => todo!("{:#?}", _error),
            Self::InternalError(error) => error.into_report(),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NoTypeFound {
    span: Span,
}

impl NoTypeFound {
    pub fn error(span: Span) -> TypeError {
        TypeError::InternalError(InternalError::NoTypeFound(NoTypeFound { span }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NoSymbolFound {
    span: Span,
}

impl NoSymbolFound {
    pub fn error(span: Span) -> TypeError {
        TypeError::InternalError(InternalError::NoSymbolFound(NoSymbolFound { span }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum InternalError {
    NoTypeFound(NoTypeFound),
    NoSymbolFound(NoSymbolFound),
}

impl Reportable for InternalError {
    fn into_report(self) -> Report {
        match self {
            Self::NoTypeFound(_error) => todo!(),
            Self::NoSymbolFound(_error) => todo!(),
        }
    }
}
