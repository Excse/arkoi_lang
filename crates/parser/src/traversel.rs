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
        walk_program(self, node)
    }

    fn visit_statement(&mut self, node: &'a mut StatementKind) -> Self::Result {
        walk_statement(self, node)
    }

    fn visit_expression_statement(&mut self, node: &'a mut ExpressionNode) -> Self::Result {
        walk_expression_statement(self, node)
    }

    fn visit_let_declaration(&mut self, node: &'a mut LetDeclarationNode) -> Self::Result {
        walk_let_declaration(self, node)
    }

    fn visit_fun_declaration(&mut self, node: &'a mut FunDeclarationNode) -> Self::Result {
        walk_fun_declaration(self, node)
    }

    fn visit_parameter(&mut self, node: &'a mut ParameterNode) -> Self::Result {
        walk_parameter(self, node)
    }

    fn visit_block(&mut self, node: &'a mut BlockNode) -> Self::Result {
        walk_block(self, node)
    }

    fn visit_expression(&mut self, node: &'a mut ExpressionKind) -> Self::Result {
        walk_expression(self, node)
    }

    fn visit_equality(&mut self, node: &'a mut EqualityNode) -> Self::Result {
        walk_equality(self, node)
    }

    fn visit_comparison(&mut self, node: &'a mut ComparisonNode) -> Self::Result {
        walk_comparison(self, node)
    }

    fn visit_term(&mut self, node: &'a mut TermNode) -> Self::Result {
        walk_term(self, node)
    }

    fn visit_factor(&mut self, node: &'a mut FactorNode) -> Self::Result {
        walk_factor(self, node)
    }

    fn visit_unary(&mut self, node: &'a mut UnaryNode) -> Self::Result {
        walk_unary(self, node)
    }

    fn visit_call(&mut self, node: &'a mut CallNode) -> Self::Result {
        walk_call(self, node)
    }

    fn visit_grouping(&mut self, node: &'a mut GroupingNode) -> Self::Result {
        walk_grouping(self, node)
    }

    fn visit_literal(&mut self, node: &'a mut LiteralNode) -> Self::Result {
        walk_literal(self, node)
    }

    fn visit_variable(&mut self, node: &'a mut VariableNode) -> Self::Result {
        walk_variable(self, node)
    }

    fn visit_type(&mut self, node: &'a mut TypeNode) -> Self::Result {
        walk_type(self, node)
    }
}

pub fn walk_program<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut ProgramNode) -> V::Result {
    node.statements
        .iter_mut()
        .map(|statement| visitor.visit_statement(statement));
    V::default_result()
}

pub fn walk_statement<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut StatementKind,
) -> V::Result {
    match node {
        StatementKind::Expression(node) => visitor.visit_expression_statement(node),
        StatementKind::LetDeclaration(node) => visitor.visit_let_declaration(node),
        StatementKind::FunDeclaration(node) => visitor.visit_fun_declaration(node),
        StatementKind::Block(node) => visitor.visit_block(node),
    }
}

pub fn walk_expression_statement<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut ExpressionNode,
) -> V::Result {
    visitor.visit_expression(&mut node.expression)
}

pub fn walk_let_declaration<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut LetDeclarationNode,
) -> V::Result {
    visitor.visit_type(&mut node.type_);

    if let Some(ref mut expression) = node.expression {
        visitor.visit_expression(expression);
    }

    V::default_result()
}

pub fn walk_fun_declaration<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut FunDeclarationNode,
) -> V::Result {
    node.parameters
        .iter_mut()
        .map(|parameter| visitor.visit_parameter(parameter));

    visitor.visit_type(&mut node.type_);

    visitor.visit_statement(&mut node.block);

    V::default_result()
}

pub fn walk_parameter<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut ParameterNode,
) -> V::Result {
    visitor.visit_type(&mut node.type_);

    V::default_result()
}

pub fn walk_block<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut BlockNode) -> V::Result {
    node.statements
        .iter_mut()
        .map(|statement| visitor.visit_statement(statement));

    V::default_result()
}

pub fn walk_expression<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut ExpressionKind,
) -> V::Result {
    match node {
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

pub fn walk_equality<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut EqualityNode) -> V::Result {
    visitor.visit_expression(&mut node.lhs);
    visitor.visit_expression(&mut node.rhs);

    V::default_result()
}

pub fn walk_comparison<'a, V: Visitor<'a>>(
    visitor: &mut V,
    node: &'a mut ComparisonNode,
) -> V::Result {
    visitor.visit_expression(&mut node.lhs);
    visitor.visit_expression(&mut node.rhs);

    V::default_result()
}

pub fn walk_term<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut TermNode) -> V::Result {
    visitor.visit_expression(&mut node.lhs);
    visitor.visit_expression(&mut node.rhs);

    V::default_result()
}

pub fn walk_factor<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut FactorNode) -> V::Result {
    visitor.visit_expression(&mut node.lhs);
    visitor.visit_expression(&mut node.rhs);

    V::default_result()
}

pub fn walk_unary<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut UnaryNode) -> V::Result {
    visitor.visit_expression(&mut node.expression);

    V::default_result()
}

pub fn walk_call<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut CallNode) -> V::Result {
    visitor.visit_expression(&mut node.callee);
    node.arguments
        .iter_mut()
        .map(|argument| visitor.visit_expression(argument));

    V::default_result()
}

pub fn walk_grouping<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut GroupingNode) -> V::Result {
    visitor.visit_expression(&mut node.expression)
}

pub fn walk_literal<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut LiteralNode) -> V::Result {
    V::default_result()
}

pub fn walk_variable<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut VariableNode) -> V::Result {
    V::default_result()
}

pub fn walk_type<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a mut TypeNode) -> V::Result {
    V::default_result()
}
