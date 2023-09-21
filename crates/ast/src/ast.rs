use std::rc::Rc;

#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{symbol::Symbol, traversal::Visitor};
use lexer::token::{Token, TokenKind};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default)]
pub struct ProgramNode {
    pub statements: Vec<StatementKind>,
}

impl ProgramNode {
    pub fn new(statements: Vec<StatementKind>) -> Self {
        ProgramNode { statements }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum StatementKind {
    Expression(ExpressionNode),
    LetDeclaration(LetDeclarationNode),
    FunDeclaration(Box<FunDeclarationNode>),
    Block(BlockNode),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct ExpressionNode {
    pub expression: ExpressionKind,
}

impl ExpressionNode {
    pub fn statement(expression: ExpressionKind) -> StatementKind {
        StatementKind::Expression(ExpressionNode { expression })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
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
        StatementKind::LetDeclaration(LetDeclarationNode {
            name,
            type_,
            expression,
            symbol: None,
        })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct BlockNode {
    pub statements: Vec<StatementKind>,
}

impl BlockNode {
    pub fn statement(statements: Vec<StatementKind>) -> StatementKind {
        StatementKind::Block(BlockNode { statements })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
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
#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
}

impl From<TokenKind> for TypeKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::U8 => TypeKind::U8,
            TokenKind::I8 => TypeKind::I8,
            TokenKind::U16 => TypeKind::U16,
            TokenKind::I16 => TypeKind::I16,
            TokenKind::U32 => TypeKind::U32,
            TokenKind::I32 => TypeKind::I32,
            TokenKind::U64 => TypeKind::U64,
            TokenKind::I64 => TypeKind::I64,
            TokenKind::F32 => TypeKind::F32,
            TokenKind::F64 => TypeKind::F64,
            TokenKind::Bool => TypeKind::Bool,
            _ => panic!("This tokenkind can't be converted to a typekind."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct TypeNode {
    pub kind: TypeKind,
}

impl TypeNode {
    pub fn new(kind: impl Into<TypeKind>) -> Self {
        TypeNode { kind: kind.into() }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ExpressionKind {
    Equality(Box<EqualityNode>),
    Comparison(Box<ComparisonNode>),
    Term(Box<TermNode>),
    Factor(Box<FactorNode>),
    Unary(Box<UnaryNode>),
    Call(Box<CallNode>),
    Grouping(Box<GroupingNode>),
    Literal(LiteralNode),
    Variable(VariableNode),
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
}

impl From<Token> for EqualityOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Equal => Self::Equal,
            TokenKind::NotEqual => Self::NotEqual,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
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
#[derive(Debug, Clone, Copy)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl From<Token> for ComparisonOperator {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Greater => Self::Greater,
            TokenKind::GreaterEqual => Self::GreaterEqual,
            TokenKind::Less => Self::Less,
            TokenKind::LessEqual => Self::LessEqual,
            _ => todo!("This convertion is not implemented."),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct GroupingNode {
    pub expression: ExpressionKind,
}

impl GroupingNode {
    pub fn expression(expression: ExpressionKind) -> ExpressionKind {
        ExpressionKind::Grouping(Box::new(GroupingNode { expression }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct VariableNode {
    pub identifier: Token,
    pub target: Option<Rc<Symbol>>,
}

impl VariableNode {
    pub fn expression(identifier: Token) -> ExpressionKind {
        ExpressionKind::Variable(VariableNode {
            identifier,
            target: None,
        })
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy)]
pub enum LiteralKind {
    String,
    Integer,
    Decimal,
    Bool,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct LiteralNode {
    pub token: Token,
    pub kind: LiteralKind,
}

impl LiteralNode {
    pub fn expression(token: Token, kind: LiteralKind) -> ExpressionKind {
        ExpressionKind::Literal(LiteralNode { token, kind })
    }
}
