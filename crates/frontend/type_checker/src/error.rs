#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::Type;
use ast::TypeKind;
use diagnostics::{
    positional::{Span, Spanned},
    report::{Report, Reportable},
};

pub type Result = std::result::Result<Option<Type>, TypeError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct NoTypeFound {
    span: Span,
}

impl NoTypeFound {
    pub fn error(span: Span) -> TypeError {
        TypeError::NoTypeFound(NoTypeFound { span })
    }
}

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
pub enum TypeError {
    NoTypeFound(NoTypeFound),
    InvalidBinaryType(InvalidBinaryType),
    InvalidUnaryType(InvalidUnaryType),
}

impl Reportable for TypeError {
    fn into_report(self) -> Report {
        match self {
            Self::NoTypeFound(_error) => todo!(),
            Self::InvalidBinaryType(_error) => todo!(),
            Self::InvalidUnaryType(_error) => todo!(),
        }
    }
}
