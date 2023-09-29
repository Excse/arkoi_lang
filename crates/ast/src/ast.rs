use std::rc::Rc;

#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{symbol::Symbol, traversal::Visitor};
use lexer::token::{Token, TokenKind};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProgramNode {
    pub statements: Vec<StatementKind>,
}

impl ProgramNode {
    pub fn new(statements: Vec<StatementKind>) -> Self {
        ProgramNode { statements }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Expression(Box<ExpressionNode>),
    LetDeclaration(Box<LetDeclarationNode>),
    FunDeclaration(Box<FunDeclarationNode>),
    Block(Box<BlockNode>),
    Return(Box<ReturnNode>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionNode {
    pub expression: ExpressionKind,
}

impl ExpressionNode {
    pub fn statement(expression: ExpressionKind) -> StatementKind {
        StatementKind::Expression(Box::new(ExpressionNode { expression }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct LetDeclarationNode {
    pub name: Token,
    pub type_: TypeNode,
    pub expression: Option<ExpressionKind>,
    pub symbol: Option<Rc<Symbol>>,
}

impl LetDeclarationNode {
    pub fn statement(
        name: Token,
        type_: TypeNode,
        expression: Option<ExpressionKind>,
    ) -> StatementKind {
        StatementKind::LetDeclaration(Box::new(LetDeclarationNode {
            name,
            type_,
            expression,
            symbol: None,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunDeclarationNode {
    pub name: Token,
    pub parameters: Vec<ParameterNode>,
    pub type_: TypeNode,
    pub block: StatementKind,
    pub symbol: Option<Rc<Symbol>>,
}

impl FunDeclarationNode {
    pub fn statement(
        name: Token,
        parameters: Vec<ParameterNode>,
        type_: TypeNode,
        block: StatementKind,
    ) -> StatementKind {
        StatementKind::FunDeclaration(Box::new(FunDeclarationNode {
            name,
            parameters,
            type_,
            block,
            symbol: None,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub statements: Vec<StatementKind>,
}

impl BlockNode {
    pub fn statement(statements: Vec<StatementKind>) -> StatementKind {
        StatementKind::Block(Box::new(BlockNode { statements }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
    pub expression: Option<ExpressionKind>,
}

impl ReturnNode {
    pub fn statement(expression: Option<ExpressionKind>) -> StatementKind {
        StatementKind::Return(Box::new(ReturnNode { expression }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ParameterNode {
    pub name: Token,
    pub type_: TypeNode,
    pub symbol: Option<Rc<Symbol>>,
}

impl ParameterNode {
    pub fn new(name: Token, type_: TypeNode) -> Self {
        ParameterNode {
            name,
            type_,
            symbol: None,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeKind {
    Int(bool, usize),
    Decimal(usize),
    Bool,
}

impl From<TokenKind> for TypeKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::U8 => TypeKind::Int(false, 8),
            TokenKind::I8 => TypeKind::Int(true, 8),
            TokenKind::U16 => TypeKind::Int(false, 16),
            TokenKind::I16 => TypeKind::Int(true, 16),
            TokenKind::U32 => TypeKind::Int(false, 32),
            TokenKind::I32 => TypeKind::Int(true, 32),
            TokenKind::U64 => TypeKind::Int(false, 64),
            TokenKind::I64 => TypeKind::Int(true, 64),
            TokenKind::F32 => TypeKind::Decimal(32),
            TokenKind::F64 => TypeKind::Decimal(64),
            TokenKind::Bool => TypeKind::Bool,
            _ => panic!("This tokenkind can't be converted to a typekind."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct TypeNode {
    pub kind: TypeKind,
}

impl TypeNode {
    pub fn new(kind: impl Into<TypeKind>) -> Self {
        TypeNode { kind: kind.into() }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionKind {
    Equality(Box<EqualityNode>),
    Comparison(Box<ComparisonNode>),
    Term(Box<TermNode>),
    Factor(Box<FactorNode>),
    Unary(Box<UnaryNode>),
    Call(Box<CallNode>),
    Grouping(Box<GroupingNode>),
    Literal(Box<LiteralNode>),
    Variable(Box<VariableNode>),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EqualityOperator {
    Eq,
    NotEq,
}

impl From<Token> for EqualityOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::EqEq => Self::Eq,
            TokenKind::NotEq => Self::NotEq,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct EqualityNode {
    pub lhs: ExpressionKind,
    pub operator: EqualityOperator,
    pub rhs: ExpressionKind,
}

impl EqualityNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<EqualityOperator>,
        rhs: ExpressionKind,
    ) -> ExpressionKind {
        ExpressionKind::Equality(Box::new(EqualityNode {
            lhs,
            operator: operator.into(),
            rhs,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEq,
    Less,
    LessEq,
}

impl From<Token> for ComparisonOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Greater => Self::Greater,
            TokenKind::GreaterEq => Self::GreaterEq,
            TokenKind::Less => Self::Less,
            TokenKind::LessEq => Self::LessEq,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonNode {
    pub lhs: ExpressionKind,
    pub operator: ComparisonOperator,
    pub rhs: ExpressionKind,
}

impl ComparisonNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<ComparisonOperator>,
        rhs: ExpressionKind,
    ) -> ExpressionKind {
        ExpressionKind::Comparison(Box::new(ComparisonNode {
            lhs,
            operator: operator.into(),
            rhs,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TermOperator {
    Add,
    Sub,
}

impl From<Token> for TermOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct TermNode {
    pub lhs: ExpressionKind,
    pub operator: TermOperator,
    pub rhs: ExpressionKind,
}

impl TermNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<TermOperator>,
        rhs: ExpressionKind,
    ) -> ExpressionKind {
        ExpressionKind::Term(Box::new(TermNode {
            lhs,
            operator: operator.into(),
            rhs,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FactorOperator {
    Mul,
    Div,
}

impl From<Token> for FactorOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Asterisk => Self::Mul,
            TokenKind::Slash => Self::Div,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FactorNode {
    pub lhs: ExpressionKind,
    pub operator: FactorOperator,
    pub rhs: ExpressionKind,
}

impl FactorNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<FactorOperator>,
        rhs: ExpressionKind,
    ) -> ExpressionKind {
        ExpressionKind::Factor(Box::new(FactorNode {
            lhs,
            operator: operator.into(),
            rhs,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Neg,
    LogNeg,
}

impl From<Token> for UnaryOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Minus => Self::Neg,
            TokenKind::Apostrophe => Self::LogNeg,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryNode {
    pub operator: UnaryOperator,
    pub expression: ExpressionKind,
}

impl UnaryNode {
    pub fn expression(
        operator: impl Into<UnaryOperator>,
        expression: ExpressionKind,
    ) -> ExpressionKind {
        ExpressionKind::Unary(Box::new(UnaryNode {
            operator: operator.into(),
            expression,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CallNode {
    pub callee: ExpressionKind,
    pub arguments: Vec<ExpressionKind>,
}

impl CallNode {
    pub fn expression(callee: ExpressionKind, arguments: Vec<ExpressionKind>) -> ExpressionKind {
        ExpressionKind::Call(Box::new(CallNode { callee, arguments }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct GroupingNode {
    pub expression: ExpressionKind,
}

impl GroupingNode {
    pub fn expression(expression: ExpressionKind) -> ExpressionKind {
        ExpressionKind::Grouping(Box::new(GroupingNode { expression }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct VariableNode {
    pub identifier: Token,
    pub target: Option<Rc<Symbol>>,
}

impl VariableNode {
    pub fn expression(identifier: Token) -> ExpressionKind {
        ExpressionKind::Variable(Box::new(VariableNode {
            identifier,
            target: None,
        }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralKind {
    String,
    Int,
    Decimal,
    Bool,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct LiteralNode {
    pub token: Token,
    pub kind: LiteralKind,
}

impl LiteralNode {
    pub fn expression(token: Token, kind: LiteralKind) -> ExpressionKind {
        ExpressionKind::Literal(Box::new(LiteralNode { token, kind }))
    }
}
