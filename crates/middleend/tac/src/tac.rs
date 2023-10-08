#[cfg(feature = "serialize")]
use serde::Serialize;

use std::rc::Rc;

use ast::traversal::Visitor;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Label {
    index: usize,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct GlobalRelative {
    offset: usize,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct FunctionRelative {
    offset: isize,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Immediate;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Operand {
    GlobalRelative(GlobalRelative),
    FunctionRelative(GlobalRelative),
    Immediate(Immediate),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Goto {
    label: Rc<Label>,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Return {
    value: Operand,
    result: Operand,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Binary {
    lhs: Operand,
    op: BinaryOperator,
    rhs: Operand,
}

impl Binary {
    pub fn instruction(lhs: Operand, op: BinaryOperator, rhs: Operand) -> Quadruple {
        Quadruple::Binary(Box::new(Binary { lhs, op, rhs }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Quadruple {
    Label(Rc<Label>),
    Goto(Box<Goto>),
    Return(Box<Return>),
    Binary(Box<Binary>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum TACError {
    NoQuadruple,
}

#[derive(Debug, Default)]
pub struct TACTransformer {
    instructions: Vec<Quadruple>,
}

impl Visitor for TACTransformer {
    type Return = Option<Operand>;
    type Error = TACError;

    fn default_result() -> Result<Self::Return, Self::Error> {
        Ok(None)
    }
}
