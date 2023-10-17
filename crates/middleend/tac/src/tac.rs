#[cfg(feature = "serialize")]
use serde::Serialize;

use std::rc::Rc;

use ast::{
    traversal::{Visitable, Visitor},
    BinaryOperator, UnaryOperator,
};

type Result = std::result::Result<Option<Operand>, TACError>;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Unary {
    op: Operand,
    result: Operand,
    operator: UnaryOperator,
}

impl Unary {
    pub fn new(op: Operand, result: Operand, operator: UnaryOperator) -> Self {
        Self {
            op,
            result,
            operator,
        }
    }
}

impl From<Unary> for Quadruple {
    fn from(value: Unary) -> Self {
        Self::LogNeg(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Binary {
    lhs: Operand,
    rhs: Operand,
    result: Operand,
    operator: BinaryOperator,
}

impl Binary {
    pub fn new(lhs: Operand, rhs: Operand, result: Operand, operator: BinaryOperator) -> Self {
        Self {
            lhs,
            rhs,
            result,
            operator,
        }
    }
}

impl From<Binary> for Quadruple {
    fn from(value: Binary) -> Self {
        Self::Binary(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Return {
    label: Rc<Label>,
}

impl Return {
    pub fn new(label: Rc<Label>) -> Self {
        Self { label }
    }
}

impl From<Return> for Quadruple {
    fn from(value: Return) -> Self {
        Self::Return(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum Quadruple {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    LogNeg(Box<Unary>),
    Label(Rc<Label>),
    Return(Box<Return>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Label {
    pub index: usize,
}

impl Label {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl From<Rc<Label>> for Operand {
    fn from(value: Rc<Label>) -> Self {
        Self::Label(value)
    }
}

impl From<Rc<Label>> for Quadruple {
    fn from(value: Rc<Label>) -> Self {
        Self::Label(value)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum Operand {
    Label(Rc<Label>),
    Immediate,
    Temp,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum TACError {
    NoOperand,
}

#[derive(Debug, Default)]
pub struct TACTransformer {
    instructions: Vec<Quadruple>,
    label_index: usize,
}

impl TACTransformer {
    pub fn label(&mut self) -> Rc<Label> {
        let label = Rc::new(Label::new(self.label_index));

        self.label_index += 1;
        self.instructions.push(label.clone().into());

        label
    }

    pub fn temp(&mut self) -> Operand {
        todo!()
    }

    pub fn insert(&mut self, instruction: impl Into<Quadruple>) {
        let instruction = instruction.into();
        self.instructions.push(instruction);
    }
}

impl Visitor for TACTransformer {
    type Return = Option<Operand>;
    type Error = TACError;

    fn default_result() -> Result {
        Ok(None)
    }

    fn visit_binary(&mut self, node: &mut ast::Binary) -> Result {
        let lhs = node.lhs.accept(self)?.ok_or(TACError::NoOperand)?;
        let rhs = node.rhs.accept(self)?.ok_or(TACError::NoOperand)?;

        let temp = self.temp();
        let binary = Binary::new(lhs, rhs, temp.clone(), node.operator);

        self.instructions.push(binary.into());

        Ok(Some(temp))
    }

    fn visit_unary(&mut self, node: &mut ast::Unary) -> Result {
        let operand = node.expression.accept(self)?.ok_or(TACError::NoOperand)?;

        let temp = self.temp();
        let unary = Unary::new(operand, temp.clone(), node.operator);

        self.instructions.push(unary.into());

        Ok(Some(temp))
    }
}
