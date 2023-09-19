#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{
    BlockNode, CallNode, ComparisonNode, EqualityNode, ExpressionKind, ExpressionNode, FactorNode,
    FunDeclarationNode, GroupingNode, LetDeclarationNode, LiteralKind, LiteralNode, ParameterNode,
    ProgramNode, StatementKind, TermNode, TypeKind, TypeNode, UnaryNode, VariableNode,
};
use crate::cursor::Cursor;
use crate::error::{ErrorKind, ParserError, Result};
use diagnostics::file::{FileID, Files};
use diagnostics::positional::Spannable;
use diagnostics::report::Report;
use errors::parser::didnt_expect;
use lexer::token::TokenKind;
use lexer::Lexer;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
    files: &'a Files,
    file_id: FileID,
    pub errors: Vec<ParserError>,
    wrong_start: bool,
}

impl<'a> Parser<'a> {
    pub fn new(files: &'a Files, file_id: FileID, lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            cursor: Cursor::new(files, file_id, lexer),
            files,
            file_id,
            errors: Vec::new(),
            wrong_start: false,
        }
    }

    /// ```ebnf
    /// program = declarations* EOF ;
    /// ```
    pub fn parse_program(&mut self) -> ProgramNode {
        let mut statements = Vec::new();
        loop {
            match self.parse_declaration() {
                Ok(expression) => {
                    statements.push(expression);
                }
                Err(ParserError {
                    kind: ErrorKind::EndOfFile,
                    ..
                }) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize();
                }
            };
        }

        ProgramNode::new(statements)
    }

    /// ```ebnf
    /// declaration = fun_declaration
    ///             | let_declaration
    ///             | statement ;
    /// ```
    fn parse_declaration(&mut self) -> Result<StatementKind> {
        match self.parse_let_declaration() {
            Ok(result) => return Ok(result),
            Err(error) if error.wrong_start => {}
            Err(error) => return Err(error),
        }

        match self.parse_fun_declaration() {
            Ok(result) => return Ok(result),
            Err(error) if error.wrong_start => {}
            Err(error) => return Err(error),
        }

        if let Ok(result) = self.parse_statement() {
            return Ok(result);
        }

        let token = self.cursor.peek()?;
        Err(ParserError::new(ErrorKind::DidntExpect(
            Spannable::new(token.kind.as_ref().to_string(), token.span),
            "statement, fun or let declaration".to_string(),
        )))
    }

    /// ```ebnf
    /// statement = expression_statement
    ///           | block ;
    /// ```
    fn parse_statement(&mut self) -> Result<StatementKind> {
        match self.parse_expression_statement() {
            Ok(result) => return Ok(result),
            Err(error) if error.wrong_start => {}
            Err(error) => return Err(error),
        }

        match self.parse_block() {
            Ok(result) => return Ok(result),
            Err(error) if error.wrong_start => {}
            Err(error) => return Err(error),
        }

        let token = self.cursor.peek()?;
        Err(ParserError::new(ErrorKind::DidntExpect(
            Spannable::new(token.kind.as_ref().to_string(), token.span),
            "expression statement or block".to_string(),
        )))
    }

    /// ```ebnf
    /// expression_statement = expression ";" ;
    /// ```
    fn parse_expression_statement(&mut self) -> Result<StatementKind> {
        let expression = self.parse_expression(true)?;

        self.cursor.eat(TokenKind::Semicolon);

        Ok(ExpressionNode::statement(expression))
    }

    /// ```ebnf
    /// block = "{" declaration* "}" ;
    /// ```
    fn parse_block(&mut self) -> Result<StatementKind> {
        self.cursor
            .eat(TokenKind::OBracket)
            .map_err(|error| error.wrong_start(true))?;

        let mut statements = Vec::new();
        loop {
            match self.parse_declaration() {
                Ok(expression) => {
                    statements.push(expression);
                }
                Err(ParserError {
                    kind: ErrorKind::EndOfFile,
                    ..
                }) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize();
                }
            };
        }

        self.cursor.eat(TokenKind::CBracket)?;

        Ok(BlockNode::statement(statements))
    }

    /// ```ebnf
    /// fun_declaration = "fun" IDENTIFIER "(" parameters? ")" type block ;
    /// ```
    fn parse_fun_declaration(&mut self) -> Result<StatementKind> {
        self.cursor
            .eat(TokenKind::Fun)
            .map_err(|error| error.wrong_start(true))?;

        let identifier = self.cursor.eat(TokenKind::Identifier)?;

        self.cursor.eat(TokenKind::OParent)?;

        let parameters = if self.cursor.eat(TokenKind::CParent).is_err() {
            self.parse_parameters()?
        } else {
            Vec::new()
        };

        self.cursor.eat(TokenKind::CParent)?;

        let type_ = self.parse_type()?;

        let block = self.parse_block()?;

        Ok(FunDeclarationNode::statement(
            identifier, parameters, type_, block,
        ))
    }

    /// ```ebnf
    /// parameters = IDENTIFIER type ( "," IDENTIFIER type )* ;
    /// ```
    fn parse_parameters(&mut self) -> Result<Vec<ParameterNode>> {
        let mut parameters = Vec::new();

        loop {
            let identifier = self.cursor.eat(TokenKind::Identifier)?;
            let type_ = self.parse_type()?;

            parameters.push(ParameterNode::new(identifier, type_));

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        Ok(parameters)
    }

    /// ```ebnf
    /// type = "@" ( "u8" | "i8"
    ///      | "u16" | "i16"
    ///      | "u32" | "i32"
    ///      | "u64" | "i64"
    ///      | "f32" | "f64"
    ///      | "bool" ) ;
    /// ```
    fn parse_type(&mut self) -> Result<TypeNode> {
        self.cursor.eat(TokenKind::At)?;

        let token = self.cursor.eat_any(&[
            TokenKind::U8,
            TokenKind::I8,
            TokenKind::U16,
            TokenKind::I16,
            TokenKind::U32,
            TokenKind::I32,
            TokenKind::U64,
            TokenKind::I64,
            TokenKind::F32,
            TokenKind::F64,
            TokenKind::Bool,
        ])?;

        Ok(TypeNode::new(token.kind))
    }

    /// ```ebnf
    /// let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;
    /// ```
    fn parse_let_declaration(&mut self) -> Result<StatementKind> {
        self.cursor
            .eat(TokenKind::Let)
            .map_err(|error| error.wrong_start(true))?;

        let name = self.cursor.eat(TokenKind::Identifier)?;

        let type_ = self.parse_type()?;

        let expression = match self.cursor.eat(TokenKind::Assign) {
            Ok(_) => Some(self.parse_expression(false)?),
            Err(_) => None,
        };

        self.cursor.eat(TokenKind::Semicolon)?;

        Ok(LetDeclarationNode::statement(name, type_, expression))
    }

    /// ```ebnf
    /// expression = equality;
    /// ```
    fn parse_expression(&mut self, start: bool) -> Result<ExpressionKind> {
        self.parse_equality().map_err(|error| {
            if !start {
                error.wrong_start(false)
            } else {
                error
            }
        })
    }

    /// ```ebnf
    /// equality = comparison ( ( "==" | "!=" ) comparison )* ;
    /// ```
    fn parse_equality(&mut self) -> Result<ExpressionKind> {
        let mut expression = self.parse_comparison(true)?;

        while let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Equal, TokenKind::NotEqual])
        {
            let rhs = self.parse_comparison(false)?;
            expression = EqualityNode::expression(expression, token, rhs);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    /// ```
    fn parse_comparison(&mut self, start: bool) -> Result<ExpressionKind> {
        let mut expression = self.parse_term(start)?;

        while let Ok(token) = self.cursor.eat_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let rhs = self.parse_term(false)?;
            expression = ComparisonNode::expression(expression, token, rhs);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// term = factor ( ( "-" | "+" ) factor )* ;
    /// ```
    fn parse_term(&mut self, start: bool) -> Result<ExpressionKind> {
        let mut expression = self.parse_factor(start)?;

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let rhs = self.parse_factor(false)?;
            expression = TermNode::expression(expression, token, rhs);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// factor = unary ( ( "/" | "*" ) unary )* ;
    /// ```
    fn parse_factor(&mut self, start: bool) -> Result<ExpressionKind> {
        let mut expression = self.parse_unary(start)?;

        while let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Slash, TokenKind::Asterisk])
        {
            let rhs = self.parse_unary(false)?;
            expression = FactorNode::expression(expression, token, rhs);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// unary = ( ( "!" | "-" ) unary )
    ///       | call ;
    /// ```
    fn parse_unary(&mut self, start: bool) -> Result<ExpressionKind> {
        if let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let expression = self.parse_unary(false)?;
            return Ok(UnaryNode::expression(token, expression));
        }

        self.parse_call(start)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn parse_call(&mut self, start: bool) -> Result<ExpressionKind> {
        let mut primary = self.parse_primary(start)?;

        if let ExpressionKind::Variable(ref mut node) = primary {
            node.is_function = true;
        }

        while let Ok(token) = self.cursor.eat(TokenKind::OParent) {
            primary = self.finish_parse_call(primary)?;
        }

        Ok(primary)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn finish_parse_call(&mut self, callee: ExpressionKind) -> Result<ExpressionKind> {
        if self.cursor.eat(TokenKind::CParent).is_ok() {
            return Ok(CallNode::expression(callee, Vec::new()));
        }

        let mut arguments = Vec::new();
        loop {
            arguments.push(self.parse_expression(false)?);

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        Ok(CallNode::expression(callee, arguments))
    }

    /// ```ebnf
    /// primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expression ")" ;
    /// ```
    fn parse_primary(&mut self, start: bool) -> Result<ExpressionKind> {
        if let Ok(token) = self.cursor.eat(TokenKind::Integer) {
            Ok(LiteralNode::expression(token, LiteralKind::Integer))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Decimal) {
            Ok(LiteralNode::expression(token, LiteralKind::Decimal))
        } else if let Ok(token) = self.cursor.eat(TokenKind::String) {
            Ok(LiteralNode::expression(token, LiteralKind::String))
        } else if let Ok(token) = self.cursor.eat(TokenKind::True) {
            Ok(LiteralNode::expression(token, LiteralKind::Bool))
        } else if let Ok(token) = self.cursor.eat(TokenKind::False) {
            Ok(LiteralNode::expression(token, LiteralKind::Bool))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Identifier) {
            Ok(VariableNode::expression(token))
        } else if self.cursor.eat(TokenKind::OParent).is_ok() {
            let expression = self.parse_expression(false)?;
            self.cursor.eat(TokenKind::CParent)?;
            Ok(GroupingNode::expression(expression))
        } else {
            let token = self.cursor.peek()?;
            Err(ParserError::new(ErrorKind::DidntExpect(
                Spannable::new(token.kind.as_ref().to_string(), token.span),
                "int, decimal, string, true, false, identifier, oparent".to_string(),
            ))
            .wrong_start(start))
        }
    }
}
