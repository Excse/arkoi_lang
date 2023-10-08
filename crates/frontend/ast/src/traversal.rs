#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::{
    ast::{
        Block, Call, Comparison, Equality, ExprKind, ExprStmt, Factor, FunDecl, Grouping, Id,
        LetDecl, Literal, Parameter, Program, StmtKind, Term, Type, Unary,
    },
    Return,
};

pub trait Visitor: Sized {
    type Return;
    type Error;

    fn default_result() -> Result<Self::Return, Self::Error>;

    fn visit_program(&mut self, node: &Program) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_stmt(&mut self, node: &StmtKind) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expr_stmt(&mut self, node: &ExprStmt) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_let_decl(&mut self, node: &LetDecl) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_fun_decl(&mut self, node: &FunDecl) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_parameter(&mut self, node: &Parameter) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_block(&mut self, node: &Block) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_return(&mut self, node: &Return) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expr(&mut self, node: &ExprKind) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_equality(&mut self, node: &Equality) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_comparison(&mut self, node: &Comparison) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_term(&mut self, node: &Term) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_factor(&mut self, node: &Factor) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_unary(&mut self, node: &Unary) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_call(&mut self, node: &Call) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_grouping(&mut self, node: &Grouping) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_id(&mut self, node: &Id) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_type(&mut self, node: &Type) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }
}

pub trait Walkable<V: Visitor> {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        V::default_result()
    }
}

pub trait Visitable<V: Visitor> {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error>;
}

impl<V: Visitor> Walkable<V> for Program {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Program {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_program(self)
    }
}

impl<V: Visitor> Walkable<V> for StmtKind {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self {
            Self::ExprStmt(node) => node.accept(visitor),
            Self::LetDecl(node) => node.accept(visitor),
            Self::FunDecl(node) => node.accept(visitor),
            Self::Block(node) => node.accept(visitor),
            Self::Return(node) => node.accept(visitor),
        }
    }
}

impl<V: Visitor> Visitable<V> for StmtKind {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_stmt(self)
    }
}

impl<V: Visitor> Walkable<V> for ExprStmt {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<V: Visitor> Visitable<V> for ExprStmt {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expr_stmt(self)
    }
}

impl<V: Visitor> Walkable<V> for LetDecl {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        if let Some(ref expression) = self.expression {
            expression.accept(visitor)?;
        }

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for LetDecl {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_let_decl(self)
    }
}

impl<V: Visitor> Walkable<V> for FunDecl {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.parameters.iter().try_for_each(|parameter| {
            parameter.accept(visitor)?;
            Ok(())
        })?;

        self.type_.accept(visitor)?;

        self.block.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for FunDecl {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_fun_decl(self)
    }
}

impl<V: Visitor> Walkable<V> for Parameter {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Parameter {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_parameter(self)
    }
}

impl<V: Visitor> Walkable<V> for Block {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Block {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_block(self)
    }
}

impl<V: Visitor> Walkable<V> for Return {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self.expression {
            Some(ref expression) => expression.accept(visitor),
            None => V::default_result(),
        }
    }
}

impl<V: Visitor> Visitable<V> for Return {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_return(self)
    }
}

impl<V: Visitor> Walkable<V> for ExprKind {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
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

impl<V: Visitor> Visitable<V> for ExprKind {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expr(self)
    }
}

impl<V: Visitor> Walkable<V> for Equality {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Equality {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_equality(self)
    }
}

impl<V: Visitor> Walkable<V> for Comparison {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Comparison {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_comparison(self)
    }
}

impl<V: Visitor> Walkable<V> for Term {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Term {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_term(self)
    }
}

impl<V: Visitor> Walkable<V> for Factor {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Factor {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_factor(self)
    }
}

impl<V: Visitor> Walkable<V> for Unary {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Unary {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_unary(self)
    }
}

impl<V: Visitor> Walkable<V> for Call {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.callee.accept(visitor)?;

        self.arguments.iter().try_for_each(|argument| {
            argument.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Call {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_call(self)
    }
}

impl<V: Visitor> Walkable<V> for Grouping {
    fn walk(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<V: Visitor> Visitable<V> for Grouping {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_grouping(self)
    }
}

impl<V: Visitor> Walkable<V> for Literal {}

impl<V: Visitor> Visitable<V> for Literal {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_literal(self)
    }
}

impl<V: Visitor> Walkable<V> for Id {}

impl<V: Visitor> Visitable<V> for Id {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_id(self)
    }
}

impl<V: Visitor> Walkable<V> for Type {}

impl<V: Visitor> Visitable<V> for Type {
    fn accept(&self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_type(self)
    }
}
