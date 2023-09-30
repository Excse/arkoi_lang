#[cfg(feature = "serialize")]
use serde::Serialize;

use ast::traversal::Visitor;

use crate::error::TypeError;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct TypeChecker {
    pub errors: Vec<TypeError>,
}

impl<'a> Visitor<'a> for TypeChecker {
    type Return = ();
    type Error = ();

    fn default_result() -> Result<Self::Return, Self::Error> {
        Ok(())
    }
}
