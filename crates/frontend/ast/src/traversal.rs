use std::{cell::RefCell, rc::Rc};

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

    fn visit_program(&mut self, node: &mut Program) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_stmt(&mut self, node: &mut StmtKind) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expr_stmt(&mut self, node: &mut ExprStmt) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_let_decl(&mut self, node: &mut LetDecl) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_fun_decl(
        &mut self,
        node: &mut Rc<RefCell<FunDecl>>,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_parameter(&mut self, node: &mut Parameter) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_block(&mut self, node: &mut Block) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_return(&mut self, node: &mut Return) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expr(&mut self, node: &mut ExprKind) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_equality(&mut self, node: &mut Equality) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_comparison(&mut self, node: &mut Comparison) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_term(&mut self, node: &mut Term) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_factor(&mut self, node: &mut Factor) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_unary(&mut self, node: &mut Unary) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_call(&mut self, node: &mut Call) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_grouping(&mut self, node: &mut Grouping) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_literal(&mut self, node: &mut Literal) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_id(&mut self, node: &mut Id) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_type(&mut self, node: &mut Type) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }
}

pub trait Walkable<V: Visitor> {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        V::default_result()
    }
}

pub trait Visitable<V: Visitor> {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error>;
}

impl<V: Visitor> Walkable<V> for Program {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter_mut().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Program {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_program(self)
    }
}

impl<V: Visitor> Walkable<V> for StmtKind {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
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
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_stmt(self)
    }
}

impl<V: Visitor> Walkable<V> for ExprStmt {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<V: Visitor> Visitable<V> for ExprStmt {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expr_stmt(self)
    }
}

impl<V: Visitor> Walkable<V> for LetDecl {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        if let Some(ref mut expression) = self.expression {
            expression.accept(visitor)?;
        }

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for LetDecl {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_let_decl(self)
    }
}

impl<V: Visitor> Walkable<V> for Rc<RefCell<FunDecl>> {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.borrow_mut()
            .parameters
            .iter_mut()
            .try_for_each(|parameter| {
                parameter.accept(visitor)?;
                Ok(())
            })?;

        self.borrow_mut().type_.accept(visitor)?;

        self.borrow_mut().block.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Rc<RefCell<FunDecl>> {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_fun_decl(self)
    }
}

impl<V: Visitor> Walkable<V> for Parameter {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Parameter {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_parameter(self)
    }
}

impl<V: Visitor> Walkable<V> for Block {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter_mut().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Block {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_block(self)
    }
}

impl<V: Visitor> Walkable<V> for Return {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self.expression {
            Some(ref mut expression) => expression.accept(visitor),
            None => V::default_result(),
        }
    }
}

impl<V: Visitor> Visitable<V> for Return {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_return(self)
    }
}

impl<V: Visitor> Walkable<V> for ExprKind {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
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
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expr(self)
    }
}

impl<V: Visitor> Walkable<V> for Equality {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Equality {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_equality(self)
    }
}

impl<V: Visitor> Walkable<V> for Comparison {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Comparison {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_comparison(self)
    }
}

impl<V: Visitor> Walkable<V> for Term {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Term {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_term(self)
    }
}

impl<V: Visitor> Walkable<V> for Factor {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Factor {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_factor(self)
    }
}

impl<V: Visitor> Walkable<V> for Unary {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Unary {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_unary(self)
    }
}

impl<V: Visitor> Walkable<V> for Call {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.callee.accept(visitor)?;

        self.arguments.iter_mut().try_for_each(|argument| {
            argument.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<V: Visitor> Visitable<V> for Call {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_call(self)
    }
}

impl<V: Visitor> Walkable<V> for Grouping {
    fn walk(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<V: Visitor> Visitable<V> for Grouping {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_grouping(self)
    }
}

impl<V: Visitor> Walkable<V> for Literal {}

impl<V: Visitor> Visitable<V> for Literal {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_literal(self)
    }
}

impl<V: Visitor> Walkable<V> for Id {}

impl<V: Visitor> Visitable<V> for Id {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_id(self)
    }
}

impl<V: Visitor> Walkable<V> for Type {}

impl<V: Visitor> Visitable<V> for Type {
    fn accept(&mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_type(self)
    }
}
