#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{
    fmt::{Display, Formatter, Result},
    hash::Hash,
    rc::Rc,
};

use crate::traversal::Visitor;
use diagnostics::positional::LabelSpan;
use lexer::token::{Token, TokenKind};

pub trait Node<'a>: Hash {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Program {
    pub statements: Vec<StmtKind>,
    pub span: LabelSpan,
}

impl Program {
    pub fn new(statements: Vec<StmtKind>, span: LabelSpan) -> Self {
        Program { statements, span }
    }
}

impl<'a> Node<'a> for Program {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub enum StmtKind {
    ExprStmt(Box<ExprStmt>),
    LetDecl(Box<LetDecl>),
    FunDecl(Box<FunDecl>),
    Block(Rc<Block>),
    Return(Box<Return>),
}

impl<'a> Node<'a> for StmtKind {}

impl StmtKind {
    pub fn span(&self) -> LabelSpan {
        match self {
            Self::ExprStmt(node) => node.expression.span(),
            Self::LetDecl(node) => node.span,
            Self::FunDecl(node) => node.span,
            Self::Block(node) => node.span,
            Self::Return(node) => node.span,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct ExprStmt {
    pub expression: ExprKind,
}

impl<'a> Node<'a> for ExprStmt {}

impl ExprStmt {
    pub fn statement(expression: ExprKind) -> StmtKind {
        StmtKind::ExprStmt(Box::new(ExprStmt { expression }))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct LetDecl {
    pub name: Token,
    pub type_: Type,
    pub expression: Option<ExprKind>,
    pub span: LabelSpan,
}

impl LetDecl {
    pub fn statement(
        name: Token,
        type_: Type,
        expression: Option<ExprKind>,
        span: LabelSpan,
    ) -> StmtKind {
        StmtKind::LetDecl(Box::new(LetDecl {
            name,
            type_,
            expression,
            span,
        }))
    }
}

impl<'a> Node<'a> for LetDecl {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct FunDecl {
    pub name: Token,
    pub parameters: Vec<Parameter>,
    pub type_: Type,
    pub block: Rc<Block>,
    pub span: LabelSpan,
}

impl FunDecl {
    pub fn statement(
        name: Token,
        parameters: Vec<Parameter>,
        type_: Type,
        block: Rc<Block>,
        span: LabelSpan,
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

impl<'a> Node<'a> for FunDecl {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Block {
    pub statements: Vec<StmtKind>,
    pub span: LabelSpan,
}

impl Block {
    pub fn statement(statements: Vec<StmtKind>, span: LabelSpan) -> StmtKind {
        StmtKind::Block(Rc::new(Block { statements, span }))
    }
}

impl<'a> Node<'a> for Block {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Return {
    pub expression: Option<ExprKind>,
    pub span: LabelSpan,
}

impl Return {
    pub fn statement(expression: Option<ExprKind>, span: LabelSpan) -> StmtKind {
        StmtKind::Return(Box::new(Return { expression, span }))
    }
}

impl<'a> Node<'a> for Return {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Parameter {
    pub name: Token,
    pub type_: Type,
    pub span: LabelSpan,
}

impl Parameter {
    pub fn new(name: Token, type_: Type, span: LabelSpan) -> Self {
        Parameter { name, type_, span }
    }
}

impl<'a> Node<'a> for Parameter {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TypeKind {
    Int(bool, usize),
    Decimal(usize),
    Bool,
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Int(signed, size) => {
                let prefix = if *signed { "i" } else { "u" };
                write!(f, "{}{}", prefix, size)
            }
            Self::Decimal(size) => write!(f, "f{}", size),
            Self::Bool => write!(f, "bool"),
        }
    }
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
#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: LabelSpan,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state)
    }
}

impl Type {
    pub fn new(kind: impl Into<TypeKind>, span: LabelSpan) -> Self {
        Type {
            kind: kind.into(),
            span,
        }
    }
}

impl<'a> Node<'a> for Type {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub enum ExprKind {
    Equality(Box<Equality>),
    Comparison(Box<Comparison>),
    Term(Box<Term>),
    Factor(Box<Factor>),
    Unary(Box<Unary>),
    Call(Box<Call>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Id(Box<Id>),
}

impl<'a> Node<'a> for ExprKind {}

impl ExprKind {
    pub fn span(&self) -> LabelSpan {
        match self {
            Self::Equality(node) => node.span,
            Self::Comparison(node) => node.span,
            Self::Term(node) => node.span,
            Self::Factor(node) => node.span,
            Self::Unary(node) => node.span,
            Self::Call(node) => node.span,
            Self::Grouping(node) => node.span,
            Self::Literal(node) => node.token.span,
            Self::Id(node) => node.id.span,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
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
#[derive(Debug, Clone, Hash)]
pub struct Equality {
    pub lhs: ExprKind,
    pub operator: EqualityOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Equality {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<EqualityOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> ExprKind {
        ExprKind::Equality(Box::new(Equality {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Node<'a> for Equality {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
#[derive(Debug, Clone, Hash)]
pub struct Comparison {
    pub lhs: ExprKind,
    pub operator: ComparisonOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Comparison {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<ComparisonOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> ExprKind {
        ExprKind::Comparison(Box::new(Comparison {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Node<'a> for Comparison {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
#[derive(Debug, Clone, Hash)]
pub struct Term {
    pub lhs: ExprKind,
    pub operator: TermOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Term {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<TermOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> ExprKind {
        ExprKind::Term(Box::new(Term {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Node<'a> for Term {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
#[derive(Debug, Clone, Hash)]
pub struct Factor {
    pub lhs: ExprKind,
    pub operator: FactorOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Factor {
    pub fn expression(
        lhs: ExprKind,
        operator: impl Into<FactorOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> ExprKind {
        ExprKind::Factor(Box::new(Factor {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }))
    }
}

impl<'a> Node<'a> for Factor {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
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
#[derive(Debug, Clone, Hash)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: ExprKind,
    pub span: LabelSpan,
}

impl Unary {
    pub fn expression(
        operator: impl Into<UnaryOperator>,
        expression: ExprKind,
        span: LabelSpan,
    ) -> ExprKind {
        ExprKind::Unary(Box::new(Unary {
            operator: operator.into(),
            expression,
            span,
        }))
    }
}

impl<'a> Node<'a> for Unary {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Call {
    pub callee: ExprKind,
    pub arguments: Vec<ExprKind>,
    pub span: LabelSpan,
}

impl Call {
    pub fn expression(callee: ExprKind, arguments: Vec<ExprKind>, span: LabelSpan) -> ExprKind {
        ExprKind::Call(Box::new(Call {
            callee,
            arguments,
            span,
        }))
    }
}

impl<'a> Node<'a> for Call {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Grouping {
    pub expression: ExprKind,
    span: LabelSpan,
}

impl Grouping {
    pub fn expression(expression: ExprKind, span: LabelSpan) -> ExprKind {
        ExprKind::Grouping(Box::new(Grouping { expression, span }))
    }
}

impl<'a> Node<'a> for Grouping {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Id {
    pub id: Token,
}

impl Id {
    pub fn expression(identifier: Token) -> ExprKind {
        ExprKind::Id(Box::new(Id { id: identifier }))
    }
}

impl<'a> Node<'a> for Id {}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum LiteralKind {
    String,
    Int,
    Decimal,
    Bool,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Hash)]
pub struct Literal {
    pub token: Token,
    pub kind: LiteralKind,
}

impl Literal {
    pub fn expression(token: Token, kind: LiteralKind) -> ExprKind {
        ExprKind::Literal(Box::new(Literal { token, kind }))
    }
}

impl<'a> Node<'a> for Literal {}
