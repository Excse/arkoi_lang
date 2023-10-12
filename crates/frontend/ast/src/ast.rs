#[cfg(feature = "serialize")]
use serde::Serialize;

use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Result},
    rc::Rc,
};

use crate::{symbol::Symbol, traversal::Visitor};
use diagnostics::positional::LabelSpan;
use lexer::token::{Token, TokenKind};

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<StmtKind>,
    pub span: LabelSpan,
}

impl Program {
    pub fn new(statements: Vec<StmtKind>, span: LabelSpan) -> Self {
        Self { statements, span }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum StmtKind {
    ExprStmt(Box<ExprStmt>),
    LetDecl(Box<LetDecl>),
    FunDecl(Rc<RefCell<FunDecl>>),
    Block(Box<Block>),
    Return(Box<Return>),
}

impl StmtKind {
    pub fn span(&self) -> LabelSpan {
        match self {
            Self::ExprStmt(node) => node.expression.span(),
            Self::LetDecl(node) => node.span,
            Self::FunDecl(node) => node.borrow().span,
            Self::Block(node) => node.span,
            Self::Return(node) => node.span,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expression: ExprKind,
}

impl ExprStmt {
    pub fn new(expression: ExprKind) -> Self {
        Self { expression }
    }
}

impl From<ExprStmt> for StmtKind {
    fn from(value: ExprStmt) -> Self {
        Self::ExprStmt(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: Token,
    pub type_: Type,
    pub expression: Option<ExprKind>,
    pub span: LabelSpan,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
}

impl LetDecl {
    pub fn new(name: Token, type_: Type, expression: Option<ExprKind>, span: LabelSpan) -> Self {
        Self {
            name,
            type_,
            expression,
            span,
            symbol: None,
        }
    }
}

impl From<LetDecl> for StmtKind {
    fn from(value: LetDecl) -> Self {
        Self::LetDecl(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: Token,
    pub parameters: Vec<Parameter>,
    pub type_: Type,
    pub block: Box<Block>,
    pub span: LabelSpan,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
}

impl FunDecl {
    pub fn new(
        name: Token,
        parameters: Vec<Parameter>,
        type_: Type,
        block: Box<Block>,
        span: LabelSpan,
    ) -> Self {
        Self {
            name,
            parameters,
            type_,
            block,
            span,
            symbol: None,
        }
    }
}

impl From<FunDecl> for StmtKind {
    fn from(value: FunDecl) -> Self {
        Self::FunDecl(Rc::new(RefCell::new(value)))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<StmtKind>,
    pub span: LabelSpan,
}

impl Block {
    pub fn new(statements: Vec<StmtKind>, span: LabelSpan) -> Self {
        Self { statements, span }
    }
}

impl From<Block> for StmtKind {
    fn from(value: Block) -> Self {
        Self::Block(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Return {
    pub expression: Option<ExprKind>,
    pub span: LabelSpan,
}

impl Return {
    pub fn new(expression: Option<ExprKind>, span: LabelSpan) -> Self {
        Self { expression, span }
    }
}

impl From<Return> for StmtKind {
    fn from(value: Return) -> Self {
        Self::Return(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Token,
    pub type_: Type,
    pub span: LabelSpan,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
}

impl Parameter {
    pub fn new(name: Token, type_: Type, span: LabelSpan) -> Self {
        Self {
            name,
            type_,
            span,
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

impl Type {
    pub fn new(kind: impl Into<TypeKind>, span: LabelSpan) -> Self {
        Self {
            kind: kind.into(),
            span,
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Equality {
    pub lhs: ExprKind,
    pub operator: EqualityOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Equality {
    pub fn new(
        lhs: ExprKind,
        operator: impl Into<EqualityOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> Self {
        Self {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }
    }
}

impl From<Equality> for ExprKind {
    fn from(value: Equality) -> Self {
        Self::Equality(Box::new(value))
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
#[derive(Debug, Clone)]
pub struct Comparison {
    pub lhs: ExprKind,
    pub operator: ComparisonOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Comparison {
    pub fn new(
        lhs: ExprKind,
        operator: impl Into<ComparisonOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> Self {
        Self {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }
    }
}

impl From<Comparison> for ExprKind {
    fn from(value: Comparison) -> Self {
        Self::Comparison(Box::new(value))
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
#[derive(Debug, Clone)]
pub struct Term {
    pub lhs: ExprKind,
    pub operator: TermOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Term {
    pub fn new(
        lhs: ExprKind,
        operator: impl Into<TermOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> Self {
        Self {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }
    }
}

impl From<Term> for ExprKind {
    fn from(value: Term) -> Self {
        Self::Term(Box::new(value))
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
#[derive(Debug, Clone)]
pub struct Factor {
    pub lhs: ExprKind,
    pub operator: FactorOperator,
    pub rhs: ExprKind,
    pub span: LabelSpan,
}

impl Factor {
    pub fn new(
        lhs: ExprKind,
        operator: impl Into<FactorOperator>,
        rhs: ExprKind,
        span: LabelSpan,
    ) -> Self {
        Self {
            lhs,
            operator: operator.into(),
            rhs,
            span,
        }
    }
}

impl From<Factor> for ExprKind {
    fn from(value: Factor) -> Self {
        Self::Factor(Box::new(value))
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
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: ExprKind,
    pub span: LabelSpan,
}

impl Unary {
    pub fn new(operator: impl Into<UnaryOperator>, expression: ExprKind, span: LabelSpan) -> Self {
        Self {
            operator: operator.into(),
            expression,
            span,
        }
    }
}

impl From<Unary> for ExprKind {
    fn from(value: Unary) -> Self {
        Self::Unary(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Call {
    pub callee: ExprKind,
    pub arguments: Vec<ExprKind>,
    pub span: LabelSpan,
}

impl Call {
    pub fn new(callee: ExprKind, arguments: Vec<ExprKind>, span: LabelSpan) -> Self {
        Self {
            callee,
            arguments,
            span,
        }
    }
}

impl From<Call> for ExprKind {
    fn from(value: Call) -> Self {
        Self::Call(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: ExprKind,
    span: LabelSpan,
}

impl Grouping {
    pub fn new(expression: ExprKind, span: LabelSpan) -> Self {
        Self { expression, span }
    }
}

impl From<Grouping> for ExprKind {
    fn from(value: Grouping) -> Self {
        Self::Grouping(Box::new(value))
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct Id {
    pub id: Token,
    pub symbol: Option<Rc<RefCell<Symbol>>>,
}

impl Id {
    pub fn new(id: Token) -> Self {
        Self { id, symbol: None }
    }
}

impl From<Id> for ExprKind {
    fn from(value: Id) -> Self {
        Self::Id(Box::new(value))
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
#[derive(Debug, Clone)]
pub struct Literal {
    pub token: Token,
    pub kind: LiteralKind,
}

impl Literal {
    pub fn new(token: Token, kind: LiteralKind) -> Self {
        Self { token, kind }
    }
}

impl From<Literal> for ExprKind {
    fn from(value: Literal) -> Self {
        Self::Literal(Box::new(value))
    }
}
