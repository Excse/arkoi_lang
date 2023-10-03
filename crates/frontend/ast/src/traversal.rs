#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    ast::{
        Block, Call, Comparison, Equality, ExprKind, ExprStmt, Factor, FunDecl, Grouping, Id,
        LetDecl, Literal, Parameter, Program, StmtKind, Term, Type, Unary,
    },
    Return,
};

pub trait Visitor<'a>: Sized {
    type Return;
    type Error;

    fn default_result() -> Result<Self::Return, Self::Error>;

    fn visit_program(&mut self, node: &'a Program) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_stmt(&mut self, node: &'a StmtKind) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expr_stmt(&mut self, node: &'a ExprStmt) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_let_decl(&mut self, node: &'a LetDecl) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_fun_decl(&mut self, node: &'a FunDecl) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_parameter(&mut self, node: &'a Parameter) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_block(&mut self, node: &'a Block) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_return(&mut self, node: &'a Return) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expr(&mut self, node: &'a ExprKind) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_equality(&mut self, node: &'a Equality) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_comparison(&mut self, node: &'a Comparison) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_term(&mut self, node: &'a Term) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_factor(&mut self, node: &'a Factor) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_unary(&mut self, node: &'a Unary) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_call(&mut self, node: &'a Call) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_grouping(&mut self, node: &'a Grouping) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_literal(&mut self, node: &'a Literal) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_id(&mut self, node: &'a Id) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_type(&mut self, node: &'a Type) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }
}

pub trait Walkable<'a, V: Visitor<'a>> {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        V::default_result()
    }
}

pub trait Visitable<'a, V: Visitor<'a>> {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error>;
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Program {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Program {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_program(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for StmtKind {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self {
            Self::ExprStmt(node) => node.accept(visitor),
            Self::LetDecl(node) => node.accept(visitor),
            Self::FunDecl(node) => node.accept(visitor),
            Self::Block(node) => node.accept(visitor),
            Self::Return(node) => node.accept(visitor),
        }
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for StmtKind {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_stmt(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ExprStmt {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ExprStmt {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expr_stmt(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for LetDecl {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        if let Some(ref expression) = self.expression {
            expression.accept(visitor)?;
        }

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for LetDecl {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_let_decl(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for FunDecl {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.parameters.iter().try_for_each(|parameter| {
            parameter.accept(visitor)?;
            Ok(())
        })?;

        self.type_.accept(visitor)?;

        self.block.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for FunDecl {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_fun_decl(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Parameter {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Parameter {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_parameter(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Block {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Block {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_block(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Return {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self.expression {
            Some(ref expression) => expression.accept(visitor),
            None => V::default_result(),
        }
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Return {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_return(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ExprKind {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self {
            ExprKind::Equality(node) => node.accept(visitor),
            ExprKind::Comparison(node) => node.accept(visitor),
            ExprKind::Term(node) => node.accept(visitor),
            ExprKind::Factor(node) => node.accept(visitor),
            ExprKind::Unary(node) => node.accept(visitor),
            ExprKind::Call(node) => node.accept(visitor),
            ExprKind::Grouping(node) => node.accept(visitor),
            ExprKind::Literal(node) => node.accept(visitor),
            ExprKind::Id(node) => node.accept(visitor),
        }
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ExprKind {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expr(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Equality {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Equality {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_equality(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Comparison {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Comparison {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_comparison(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Term {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Term {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_term(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Factor {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Factor {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_factor(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Unary {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Unary {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_unary(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Call {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.callee.accept(visitor)?;

        self.arguments.iter().try_for_each(|argument| {
            argument.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Call {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_call(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Grouping {
    fn walk(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Grouping {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_grouping(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Literal {}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Literal {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_literal(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Id {}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Id {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_id(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for Type {}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for Type {
    fn accept(&'a self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_type(self)
    }
}
