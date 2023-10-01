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
pub struct Program {
    pub statements: Vec<StmtKind>,
    span: Span,
}

impl Program {
    pub fn new(statements: Vec<StmtKind>, span: Span) -> Self {
        Program { statements, span }
    }
}

impl<'a> Spannable<'a> for Program {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
    ExprStmt(Box<ExprStmt>),
    LetDecl(Box<LetDecl>),
    FunDecl(Box<FunDecl>),
    Block(Rc<Block>),
    Return(Box<Return>),
}

impl<'a> Spannable<'a> for StmtKind {
    fn span(&'a self) -> &'a Span {
        match self {
            Self::ExprStmt(node) => node.span(),
            Self::LetDecl(node) => node.span(),
            Self::FunDecl(node) => node.span(),
            Self::Block(node) => node.span(),
            Self::Return(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ExprStmt {
    pub expression: ExprKind,
}

impl ExprStmt {
    pub fn statement(expression: ExprKind) -> StmtKind {
        StmtKind::ExprStmt(Box::new(ExprStmt { expression }))
    }
}

impl<'a> Spannable<'a> for ExprStmt {
    fn span(&'a self) -> &'a Span {
        self.expression.span()
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct LetDecl {
    pub name: Token,
    pub type_: Type,
    pub expression: Option<ExprKind>,
    span: Span,
}

impl LetDecl {
    pub fn statement(
        name: Token,
        type_: Type,
        expression: Option<ExprKind>,
        span: Span,
    ) -> StmtKind {
        StmtKind::LetDecl(Box::new(LetDecl {
            name,
            type_,
            expression,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for LetDecl {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunDecl {
    pub name: Token,
    pub parameters: Vec<Parameter>,
    pub type_: Type,
    pub block: Rc<Block>,
    span: Span,
}

impl FunDecl {
    pub fn statement(
        name: Token,
        parameters: Vec<Parameter>,
        type_: Type,
        block: Rc<Block>,
        span: Span,
    ) -> StmtKind {
        StmtKind::FunDecl(Box::new(FunDecl {
            name,
            parameters,
            type_,
            block,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for FunDecl {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<StmtKind>,
    span: Span,
}

impl Block {
    pub fn statement(statements: Vec<StmtKind>, span: Span) -> StmtKind {
        StmtKind::Block(Rc::new(Block { statements, span }))
    }
}

impl<'a> Spannable<'a> for Block {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub expression: Option<ExprKind>,
    span: Span,
}

impl Return {
    pub fn statement(expression: Option<ExprKind>, span: Span) -> StmtKind {
        StmtKind::Return(Box::new(Return { expression, span }))
    }
}

impl<'a> Spannable<'a> for Return {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: Token,
    pub type_: Type,
    span: Span,
}

impl Parameter {
    pub fn new(name: Token, type_: Type, span: Span) -> Self {
        Parameter { name, type_, span }
    }
}

impl<'a> Spannable<'a> for Parameter {
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
pub struct Type {
    pub kind: TypeKind,
    span: Span,
}

impl Type {
    pub fn new(kind: impl Into<TypeKind>, span: Span) -> Self {
        Type {
            kind: kind.into(),
            span,
        }
    }
}

impl<'a> Spannable<'a> for Type {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Equality(Box<Equality>),
    Comparison(Box<Comparison>),
    Term(Box<Term>),
    Factor(Box<Factor>),
    Unary(Box<Unary>),
    Call(Box<Call>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Variable(Box<Id>),
}

impl<'a> Spannable<'a> for ExprKind {
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
pub struct Equality {
    pub lhs: ExprKind,
    pub operator: EqualityOperator,
    pub rhs: ExprKind,
    span: Span,
}

impl Equality {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<EqualityOperator>,
        rhs: ExprKind,
        span: Span,
    ) -> ExprKind {
        ExprKind::Equality(Box::new(Equality {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for Equality {
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
pub struct Comparison {
    pub lhs: ExprKind,
    pub operator: ComparisonOperator,
    pub rhs: ExprKind,
    span: Span,
}

impl Comparison {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<ComparisonOperator>,
        rhs: ExprKind,
        span: Span,
    ) -> ExprKind {
        ExprKind::Comparison(Box::new(Comparison {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for Comparison {
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
pub struct Term {
    pub lhs: ExprKind,
    pub operator: TermOperator,
    pub rhs: ExprKind,
    span: Span,
}

impl Term {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<TermOperator>,
        rhs: ExprKind,
        span: Span,
    ) -> ExprKind {
        ExprKind::Term(Box::new(Term {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for Term {
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
pub struct Factor {
    pub lhs: ExprKind,
    pub operator: FactorOperator,
    pub rhs: ExprKind,
    span: Span,
}

impl Factor {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<FactorOperator>,
        rhs: ExprKind,
        span: Span,
    ) -> ExprKind {
        ExprKind::Factor(Box::new(Factor {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for Factor {
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
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: ExprKind,
    span: Span,
}

impl Unary {
    pub fn expression(
        operator: impl Into<UnaryOperator>,
        expression: ExprKind,
        span: Span,
    ) -> ExprKind {
        ExprKind::Unary(Box::new(Unary {
            operator: operator.into(),
            expression,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for Unary {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub callee: ExprKind,
    pub arguments: Vec<ExprKind>,
    span: Span,
}

impl Call {
    pub fn expression(callee: ExprKind, arguments: Vec<ExprKind>, span: Span) -> ExprKind {
        ExprKind::Call(Box::new(Call {
            callee,
            arguments,
            span,
        }))
    }
}

impl<'a> Spannable<'a> for Call {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub expression: ExprKind,
    span: Span,
}

impl Grouping {
    pub fn expression(expression: ExprKind, span: Span) -> ExprKind {
        ExprKind::Grouping(Box::new(Grouping { expression, span }))
    }
}

impl<'a> Spannable<'a> for Grouping {
    fn span(&'a self) -> &'a Span {
        &self.span
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Id {
    pub id: Token,
}

impl Id {
    pub fn expression(identifier: Token) -> ExprKind {
        ExprKind::Variable(Box::new(Id { id: identifier }))
    }
}

impl<'a> Spannable<'a> for Id {
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
pub struct Literal {
    pub token: Token,
    pub kind: LiteralKind,
}

impl Literal {
    pub fn expression(token: Token, kind: LiteralKind) -> ExprKind {
        ExprKind::Literal(Box::new(Literal { token, kind }))
    }
}

impl<'a> Spannable<'a> for Literal {
    fn span(&'a self) -> &'a Span {
        self.token.span()
    }
}
