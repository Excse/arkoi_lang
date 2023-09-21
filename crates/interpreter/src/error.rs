#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::Output;

pub type Result = std::result::Result<Output, InterpreterError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum InterpreterError {
    Undefined,
}
