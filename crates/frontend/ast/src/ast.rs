#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{
    fmt::{Display, Formatter, Result},
    rc::Rc,
};

use crate::traversal::Visitor;
use diagnostics::positional::{Span, Spannable};
use lexer::token::{Token, TokenKind};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProgramNode {
    pub statements: Vec<StatementKind>,
    span: Span,
}

impl ProgramNode {
    pub fn new(statements: Vec<StatementKind>, span: Span) -> Self {
        ProgramNode { statements, span }
    }
}

impl<'a> Spannable<'a> for ProgramNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum StatementKind {
    Expression(Box<ExpressionNode>),
    LetDeclaration(Box<LetDeclarationNode>),
    FunDeclaration(Box<FunDeclarationNode>),
    Block(Rc<BlockNode>),
    Return(Box<ReturnNode>),
}

impl<'a> Spannable<'a> for StatementKind {
    fn span(&'a self) -> &'a Span {
        match self {
            Self::Expression(node) => node.span(),
            Self::LetDeclaration(node) => node.span(),
            Self::FunDeclaration(node) => node.span(),
            Self::Block(node) => node.span(),
            Self::Return(node) => node.span(),
        }
    }
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

impl<'a> Spannable<'a> for ExpressionNode {
    fn span(&'a self) -> &'a Span {
        self.expression.span()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct LetDeclarationNode {
    pub name: Token,
    pub type_: TypeNode,
    pub expression: Option<ExpressionKind>,
    span: Span,
}

impl LetDeclarationNode {
    pub fn statement(
        name: Token,
        type_: TypeNode,
        expression: Option<ExpressionKind>,
        span: Span,
    ) -> StatementKind {
        StatementKind::LetDeclaration(Box::new(LetDeclarationNode {
            name,
            type_,
            expression,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for LetDeclarationNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunDeclarationNode {
    pub name: Token,
    pub parameters: Vec<ParameterNode>,
    pub type_: TypeNode,
    pub block: Rc<BlockNode>,
    span: Span,
}

impl FunDeclarationNode {
    pub fn statement(
        name: Token,
        parameters: Vec<ParameterNode>,
        type_: TypeNode,
        block: Rc<BlockNode>,
        span: Span,
    ) -> StatementKind {
        StatementKind::FunDeclaration(Box::new(FunDeclarationNode {
            name,
            parameters,
            type_,
            block,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for FunDeclarationNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub statements: Vec<StatementKind>,
    span: Span,
}

impl BlockNode {
    pub fn statement(statements: Vec<StatementKind>, span: Span) -> StatementKind {
        StatementKind::Block(Rc::new(BlockNode { statements, span }))
    }
}

impl<'a> Spannable<'a> for BlockNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnNode {
    pub expression: Option<ExpressionKind>,
    span: Span,
}

impl ReturnNode {
    pub fn statement(expression: Option<ExpressionKind>, span: Span) -> StatementKind {
        StatementKind::Return(Box::new(ReturnNode { expression, span }))
    }
}

impl<'a> Spannable<'a> for ReturnNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ParameterNode {
    pub name: Token,
    pub type_: TypeNode,
    span: Span,
}

impl ParameterNode {
    pub fn new(name: Token, type_: TypeNode, span: Span) -> Self {
        ParameterNode { name, type_, span }
    }
}

impl<'a> Spannable<'a> for ParameterNode {
    fn span(&'a self) -> &'a Span {
        &self.span
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
    span: Span,
}

impl TypeNode {
    pub fn new(kind: impl Into<TypeKind>, span: Span) -> Self {
        TypeNode {
            kind: kind.into(),
            span,
        }
    }
}

impl<'a> Spannable<'a> for TypeNode {
    fn span(&'a self) -> &'a Span {
        &self.span
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
    Variable(Box<IdNode>),
}

impl<'a> Spannable<'a> for ExpressionKind {
    fn span(&'a self) -> &'a Span {
        match self {
            Self::Equality(node) => node.span(),
            Self::Comparison(node) => node.span(),
            Self::Term(node) => node.span(),
            Self::Factor(node) => node.span(),
            Self::Unary(node) => node.span(),
            Self::Call(node) => node.span(),
            Self::Grouping(node) => node.span(),
            Self::Literal(node) => node.span(),
            Self::Variable(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EqualityOperator {
    Eq,
    NotEq,
}

impl Display for EqualityOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),
        }
    }
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
    span: Span,
}

impl EqualityNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<EqualityOperator>,
        rhs: ExpressionKind,
        span: Span,
    ) -> ExpressionKind {
        ExpressionKind::Equality(Box::new(EqualityNode {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for EqualityNode {
    fn span(&'a self) -> &'a Span {
        &self.span
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

impl Display for ComparisonOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Greater => write!(f, ">"),
            Self::GreaterEq => write!(f, ">="),
            Self::Less => write!(f, "<="),
            Self::LessEq => write!(f, "<="),
        }
    }
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
    span: Span,
}

impl ComparisonNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<ComparisonOperator>,
        rhs: ExpressionKind,
        span: Span,
    ) -> ExpressionKind {
        ExpressionKind::Comparison(Box::new(ComparisonNode {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for ComparisonNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TermOperator {
    Add,
    Sub,
}

impl Display for TermOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
        }
    }
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
    span: Span,
}

impl TermNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<TermOperator>,
        rhs: ExpressionKind,
        span: Span,
    ) -> ExpressionKind {
        ExpressionKind::Term(Box::new(TermNode {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for TermNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FactorOperator {
    Mul,
    Div,
}

impl Display for FactorOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
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
    span: Span,
}

impl FactorNode {
    pub fn expression(
        lhs: ExpressionKind,
        operator: impl Into<FactorOperator>,
        rhs: ExpressionKind,
        span: Span,
    ) -> ExpressionKind {
        ExpressionKind::Factor(Box::new(FactorNode {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for FactorNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Neg,
    LogNeg,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::LogNeg => write!(f, "!"),
        }
    }
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
    span: Span,
}

impl UnaryNode {
    pub fn expression(
        operator: impl Into<UnaryOperator>,
        expression: ExpressionKind,
        span: Span,
    ) -> ExpressionKind {
        ExpressionKind::Unary(Box::new(UnaryNode {
            operator: operator.into(),
            expression,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for UnaryNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CallNode {
    pub callee: ExpressionKind,
    pub arguments: Vec<ExpressionKind>,
    span: Span,
}

impl CallNode {
    pub fn expression(
        callee: ExpressionKind,
        arguments: Vec<ExpressionKind>,
        span: Span,
    ) -> ExpressionKind {
        ExpressionKind::Call(Box::new(CallNode {
            callee,
            arguments,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for CallNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct GroupingNode {
    pub expression: ExpressionKind,
    span: Span,
}

impl GroupingNode {
    pub fn expression(expression: ExpressionKind, span: Span) -> ExpressionKind {
        ExpressionKind::Grouping(Box::new(GroupingNode { expression, span }))
    }
}

impl<'a> Spannable<'a> for GroupingNode {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct IdNode {
    pub id: Token,
}

impl IdNode {
    pub fn expression(identifier: Token) -> ExpressionKind {
        ExpressionKind::Variable(Box::new(IdNode { id: identifier }))
    }
}

impl<'a> Spannable<'a> for IdNode {
    fn span(&'a self) -> &'a Span {
        self.id.span()
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

impl<'a> Spannable<'a> for LiteralNode {
    fn span(&'a self) -> &'a Span {
        self.token.span()
    }
}
