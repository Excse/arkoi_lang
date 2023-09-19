#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{
    BlockNode, CallNode, ComparisonNode, EqualityNode, ExpressionKind, ExpressionNode, FactorNode,
    FunDeclarationNode, GroupingNode, LetDeclarationNode, LiteralNode, ParameterNode, ProgramNode,
    StatementKind, TermNode, TypeNode, UnaryNode, VariableNode,
};

pub trait Visitor<'a>: Sized {
    type Result;

    fn default_result() -> Self::Result;

    fn visit_program(&mut self, node: &'a mut ProgramNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_statement(&mut self, node: &'a mut StatementKind) -> Self::Result {
        node.walk(self)
    }

    fn visit_expression_statement(&mut self, node: &'a mut ExpressionNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_let_declaration(&mut self, node: &'a mut LetDeclarationNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_fun_declaration(&mut self, node: &'a mut FunDeclarationNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_parameter(&mut self, node: &'a mut ParameterNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_block(&mut self, node: &'a mut BlockNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_expression(&mut self, node: &'a mut ExpressionKind) -> Self::Result {
        node.walk(self)
    }

    fn visit_equality(&mut self, node: &'a mut EqualityNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_comparison(&mut self, node: &'a mut ComparisonNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_term(&mut self, node: &'a mut TermNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_factor(&mut self, node: &'a mut FactorNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_unary(&mut self, node: &'a mut UnaryNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_call(&mut self, node: &'a mut CallNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_grouping(&mut self, node: &'a mut GroupingNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_literal(&mut self, node: &'a mut LiteralNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_variable(&mut self, node: &'a mut VariableNode) -> Self::Result {
        node.walk(self)
    }

    fn visit_type(&mut self, node: &'a mut TypeNode) -> Self::Result {
        node.walk(self)
    }
}

pub trait Walkable<'a, V: Visitor<'a>> {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ProgramNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        self.statements
            .iter_mut()
            .map(|statement| visitor.visit_statement(statement));

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for StatementKind {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        match self {
            StatementKind::Expression(node) => visitor.visit_expression_statement(node),
            StatementKind::LetDeclaration(node) => visitor.visit_let_declaration(node),
            StatementKind::FunDeclaration(node) => visitor.visit_fun_declaration(node),
            StatementKind::Block(node) => visitor.visit_block(node),
        }
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ExpressionNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.expression)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for LetDeclarationNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_type(&mut self.type_);

        if let Some(ref mut expression) = self.expression {
            visitor.visit_expression(expression);
        }

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for FunDeclarationNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        self.parameters
            .iter_mut()
            .map(|parameter| visitor.visit_parameter(parameter));

        visitor.visit_type(&mut self.type_);

        visitor.visit_statement(&mut self.block);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ParameterNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_type(&mut self.type_);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for BlockNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        self.statements
            .iter_mut()
            .map(|statement| visitor.visit_statement(statement));

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ExpressionKind {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        match self {
            ExpressionKind::Equality(node) => visitor.visit_equality(node.as_mut()),
            ExpressionKind::Comparison(node) => visitor.visit_comparison(node.as_mut()),
            ExpressionKind::Term(node) => visitor.visit_term(node.as_mut()),
            ExpressionKind::Factor(node) => visitor.visit_factor(node.as_mut()),
            ExpressionKind::Unary(node) => visitor.visit_unary(node.as_mut()),
            ExpressionKind::Call(node) => visitor.visit_call(node.as_mut()),
            ExpressionKind::Grouping(node) => visitor.visit_grouping(node.as_mut()),
            ExpressionKind::Literal(node) => visitor.visit_literal(node),
            ExpressionKind::Variable(node) => visitor.visit_variable(node),
        }
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for EqualityNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.lhs);
        visitor.visit_expression(&mut self.rhs);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for ComparisonNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.lhs);
        visitor.visit_expression(&mut self.rhs);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for TermNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.lhs);
        visitor.visit_expression(&mut self.rhs);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for FactorNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.lhs);
        visitor.visit_expression(&mut self.rhs);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for UnaryNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.expression);

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for CallNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.callee);
        self.arguments
            .iter_mut()
            .map(|argument| visitor.visit_expression(argument));

        V::default_result()
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for GroupingNode {
    fn walk(&'a mut self, visitor: &mut V) -> V::Result {
        visitor.visit_expression(&mut self.expression)
    }
}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for LiteralNode {}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for VariableNode {}

impl<'a, V: Visitor<'a>> Walkable<'a, V> for TypeNode {}
