#[cfg(feature = "serialize")]
use serde::Serialize;

use diagnostics::report::{Report, Reportable};

pub type Result = std::result::Result<(), TypeError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum TypeError {}

impl Reportable for TypeError {
    fn into_report(self) -> Report {
        todo!()
    }
}
