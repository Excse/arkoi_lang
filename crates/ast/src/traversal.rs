#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{
    BlockNode, CallNode, ComparisonNode, EqualityNode, ExpressionKind, ExpressionNode, FactorNode,
    FunDeclarationNode, GroupingNode, LetDeclarationNode, LiteralNode, ParameterNode, ProgramNode,
    StatementKind, TermNode, TypeNode, UnaryNode, VariableNode,
};

pub trait Visitor<'a>: Sized {
    type Return;
    type Error;

    fn default_result() -> Result<Self::Return, Self::Error>;

    fn visit_program(&mut self, node: &'a mut ProgramNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_statement(
        &mut self,
        node: &'a mut StatementKind,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expression_statement(
        &mut self,
        node: &'a mut ExpressionNode,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_let_declaration(
        &mut self,
        node: &'a mut LetDeclarationNode,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_fun_declaration(
        &mut self,
        node: &'a mut FunDeclarationNode,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_parameter(
        &mut self,
        node: &'a mut ParameterNode,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_block(&mut self, node: &'a mut BlockNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_expression(
        &mut self,
        node: &'a mut ExpressionKind,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_equality(&mut self, node: &'a mut EqualityNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_comparison(
        &mut self,
        node: &'a mut ComparisonNode,
    ) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_term(&mut self, node: &'a mut TermNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_factor(&mut self, node: &'a mut FactorNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_unary(&mut self, node: &'a mut UnaryNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_call(&mut self, node: &'a mut CallNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_grouping(&mut self, node: &'a mut GroupingNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_literal(&mut self, node: &'a mut LiteralNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_variable(&mut self, node: &'a mut VariableNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }

    fn visit_type(&mut self, node: &'a mut TypeNode) -> Result<Self::Return, Self::Error> {
        node.walk(self)
    }
}

pub trait Walkable<'a, V: Visitor<'a>> {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        V::default_result()
    }
}

pub trait Visitable<'a, V: Visitor<'a>> {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error>;
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ProgramNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter_mut().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ProgramNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_program(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for StatementKind {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self {
            StatementKind::Expression(node) => node.accept(visitor),
            StatementKind::LetDeclaration(node) => node.accept(visitor),
            StatementKind::FunDeclaration(node) => node.accept(visitor),
            StatementKind::Block(node) => node.accept(visitor),
        }
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for StatementKind {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_statement(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ExpressionNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ExpressionNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expression_statement(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for LetDeclarationNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        if let Some(ref mut expression) = self.expression {
            expression.accept(visitor)?;
        }

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for LetDeclarationNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_let_declaration(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for FunDeclarationNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.parameters.iter_mut().try_for_each(|parameter| {
            parameter.accept(visitor)?;
            Ok(())
        })?;

        self.type_.accept(visitor)?;

        self.block.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for FunDeclarationNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_fun_declaration(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ParameterNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.type_.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ParameterNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_parameter(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for BlockNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.statements.iter_mut().try_for_each(|statement| {
            statement.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for BlockNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_block(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ExpressionKind {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        match self {
            ExpressionKind::Equality(node) => node.accept(visitor),
            ExpressionKind::Comparison(node) => node.accept(visitor),
            ExpressionKind::Term(node) => node.accept(visitor),
            ExpressionKind::Factor(node) => node.accept(visitor),
            ExpressionKind::Unary(node) => node.accept(visitor),
            ExpressionKind::Call(node) => node.accept(visitor),
            ExpressionKind::Grouping(node) => node.accept(visitor),
            ExpressionKind::Literal(node) => node.accept(visitor),
            ExpressionKind::Variable(node) => node.accept(visitor),
        }
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ExpressionKind {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_expression(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for EqualityNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for EqualityNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_equality(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ComparisonNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for ComparisonNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_comparison(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for TermNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for TermNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_term(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for FactorNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.lhs.accept(visitor)?;
        self.rhs.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for FactorNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_factor(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for UnaryNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for UnaryNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_unary(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for CallNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.callee.accept(visitor)?;

        self.arguments.iter_mut().try_for_each(|argument| {
            argument.accept(visitor)?;
            Ok(())
        })?;

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for CallNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_call(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for GroupingNode {
    fn walk(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        self.expression.accept(visitor)
    }
}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for GroupingNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_grouping(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for LiteralNode {}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for LiteralNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_literal(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for VariableNode {}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for VariableNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_variable(self)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for TypeNode {}

impl<'a, V: Visitor<'a>> Visitable<'a, V> for TypeNode {
    fn accept(&'a mut self, visitor: &mut V) -> Result<V::Return, V::Error> {
        visitor.visit_type(self)
    }
}
