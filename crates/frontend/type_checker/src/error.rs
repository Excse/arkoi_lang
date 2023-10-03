#[cfg(feature = "serialize")]
use serde::Serialize;

use ast::{Type, TypeKind};
use diagnostics::{
    positional::LabelSpan,
    report::{Report, Reportable},
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

// impl Reportable for InvalidBinaryType {
//     fn into_report(self) -> Report {
//         let report_message = format!(
//             "There is no binary operator that supports: {} {} {}",
//             self.lhs, self.operator, self.rhs
//         );

//         ReportBuilder::default()
//             .message(report_message)
//             .code(1)
//             .serverity(Serverity::Error)
//             .label(
//                 LabelBuilder::default()
//                     .span(*self.span.span())
//                     .file(*self.span)
//                     .build()
//                     .unwrap(),
//             )
//             .build()
//             .unwrap()
//     }
// }

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
            Self::InternalError(error) => panic!("Internal error: {:#?}", error),
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

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum InternalError {
    NoTypeFound(NoTypeFound),
    NoSymbolFound(NoSymbolFound),
}
