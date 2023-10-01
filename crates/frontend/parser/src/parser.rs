#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::cursor::Cursor;
use crate::error::{DidntExpect, ErrorKind, InternalError, ParserError, Result};
use ast::{
    Block, Call, Comparison, Equality, ExprKind, ExprStmt, Factor,
    FunDecl, Grouping, Id, LetDecl, LiteralKind, Literal,
    Parameter, Program, Return, StmtKind, Term, Type, Unary,
};
use diagnostics::positional::{Span, Spannable};
use lexer::iterator::TokenIterator;
use lexer::token::TokenKind;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
    pub errors: Vec<ParserError>,
    wrong_start: bool,
}

impl<'a> Parser<'a> {
    pub fn new(iterator: TokenIterator<'a>) -> Parser<'a> {
        Parser {
            cursor: Cursor::new(iterator),
            errors: Vec::new(),
            wrong_start: false,
        }
    }

    /// ```ebnf
    /// program = program_declaration* EOF ;
    /// ```
    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        loop {
            match self.parse_program_declaration() {
                Ok(expression) => {
                    statements.push(expression);
                }
                Err(ParserError {
                    kind: ErrorKind::InternalError(InternalError::EndOfFile(_)),
                    ..
                }) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize_program();
                }
            };
        }

        let span = if let Some(start) = statements.first() {
            let end = statements.last().unwrap();
            start.span().combine(end.span())
        } else {
            Span::single(0)
        };

        Program::new(statements, span)
    }

    /// ```ebnf
    /// program_statements = fun_declaration
    ///                    | let_declaration ;
    /// ```
    fn parse_program_declaration(&mut self) -> Result<StmtKind> {
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

        let token = self.cursor.peek()?;
        Err(DidntExpect::error(token, "fun or let declaration"))
    }

    /// ```ebnf
    /// statement = expression_statement
    ///           | block ;
    /// ```
    fn parse_statement(&mut self) -> Result<StmtKind> {
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
        Err(DidntExpect::error(token, "expression statement or block"))
    }

    /// ```ebnf
    /// expression_statement = expression ";" ;
    /// ```
    fn parse_expression_statement(&mut self) -> Result<StmtKind> {
        let expression = self.parse_expression(true)?;

        self.cursor.eat(TokenKind::Semicolon)?;

        Ok(ExprStmt::statement(expression))
    }

    /// ```ebnf
    /// block = "{" block_declaration* "}" ;
    /// ```
    fn parse_block(&mut self) -> Result<StmtKind> {
        let start = self
            .cursor
            .eat(TokenKind::Brace(true))
            .map_err(|error| error.wrong_start(true))?;

        if let Ok(end) = self.cursor.eat(TokenKind::Brace(false)) {
            let span = start.span().combine(end.span());
            return Ok(Block::statement(Vec::new(), span));
        }

        let mut statements = Vec::new();
        loop {
            if self.cursor.is_peek(TokenKind::Brace(false)).is_some() {
                break;
            }

            match self.parse_block_declaration() {
                Ok(expression) => {
                    statements.push(expression);
                }
                Err(ParserError {
                    kind: ErrorKind::InternalError(InternalError::EndOfFile(_)),
                    ..
                }) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize_block();
                }
            };
        }

        let end = self.cursor.eat(TokenKind::Brace(false))?;

        let span = start.span().combine(end.span());
        Ok(Block::statement(statements, span))
    }

    /// ```ebnf
    /// block_declaration = let_declaration
    ///                   | return_statement
    ///                   | statement ;
    /// ```
    fn parse_block_declaration(&mut self) -> Result<StmtKind> {
        match self.parse_let_declaration() {
            Ok(result) => return Ok(result),
            Err(error) if error.wrong_start => {}
            Err(error) => return Err(error),
        }

        match self.parse_return_statement() {
            Ok(result) => return Ok(result),
            Err(error) if error.wrong_start => {}
            Err(error) => return Err(error),
        }

        if let Ok(result) = self.parse_statement() {
            return Ok(result);
        }

        let token = self.cursor.peek()?;
        Err(DidntExpect::error(token, "statement, let declaration"))
    }

    /// ```ebnf
    /// return_statement = return expression? ";" ;
    /// ```
    fn parse_return_statement(&mut self) -> Result<StmtKind> {
        let start = self
            .cursor
            .eat(TokenKind::Return)
            .map_err(|error| error.wrong_start(true))?;

        let expression = self.parse_expression(false).ok();

        let end = self.cursor.eat(TokenKind::Semicolon)?;

        let span = start.span().combine(end.span());
        Ok(Return::statement(expression, span))
    }

    /// ```ebnf
    /// fun_declaration = "fun" IDENTIFIER "(" parameters? ")" type block ;
    /// ```
    fn parse_fun_declaration(&mut self) -> Result<StmtKind> {
        let start = self
            .cursor
            .eat(TokenKind::Fun)
            .map_err(|error| error.wrong_start(true))?;

        let identifier = self.cursor.eat(TokenKind::Id)?;

        self.cursor.eat(TokenKind::Parent(true))?;

        let parameters = if self.cursor.eat(TokenKind::Parent(false)).is_err() {
            let parameters = self.parse_parameters()?;

            self.cursor.eat(TokenKind::Parent(false))?;

            parameters
        } else {
            Vec::new()
        };

        let type_ = self.parse_type()?;

        let block = match self.parse_block()? {
            StmtKind::Block(node) => node,
            _ => panic!("Couldn't unbox the block. This shouldn't have happened."),
        };

        let span = start.span().combine(block.span());
        Ok(FunDecl::statement(
            identifier, parameters, type_, block, span,
        ))
    }

    /// ```ebnf
    /// parameters = IDENTIFIER type ( "," IDENTIFIER type )* ;
    /// ```
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();

        loop {
            let id = self.cursor.eat(TokenKind::Id)?;
            let type_ = self.parse_type()?;

            let span = id.span().combine(type_.span());
            parameters.push(Parameter::new(id, type_, span));

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
    fn parse_type(&mut self) -> Result<Type> {
        let start = self.cursor.eat(TokenKind::At)?;

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

        let span = start.span().combine(token.span());
        Ok(Type::new(token.kind, span))
    }

    /// ```ebnf
    /// let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;
    /// ```
    fn parse_let_declaration(&mut self) -> Result<StmtKind> {
        let start = self
            .cursor
            .eat(TokenKind::Let)
            .map_err(|error| error.wrong_start(true))?;

        let name = self.cursor.eat(TokenKind::Id)?;

        let type_ = self.parse_type()?;

        let expression = match self.cursor.eat(TokenKind::Eq) {
            Ok(_) => Some(self.parse_expression(false)?),
            Err(_) => None,
        };

        let end = self.cursor.eat(TokenKind::Semicolon)?;

        let span = start.span().combine(end.span());
        Ok(LetDecl::statement(name, type_, expression, span))
    }

    /// ```ebnf
    /// expression = equality;
    /// ```
    fn parse_expression(&mut self, start: bool) -> Result<ExprKind> {
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
    fn parse_equality(&mut self) -> Result<ExprKind> {
        let mut expression = self.parse_comparison(true)?;

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::EqEq, TokenKind::NotEq]) {
            let rhs = self.parse_comparison(false)?;

            let span = expression.span().combine(rhs.span());
            expression = Equality::expression(expression, token, rhs, span);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    /// ```
    fn parse_comparison(&mut self, start: bool) -> Result<ExprKind> {
        let mut expression = self.parse_term(start)?;

        while let Ok(token) = self.cursor.eat_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEq,
            TokenKind::Less,
            TokenKind::LessEq,
        ]) {
            let rhs = self.parse_term(false)?;

            let span = expression.span().combine(rhs.span());
            expression = Comparison::expression(expression, token, rhs, span);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// term = factor ( ( "-" | "+" ) factor )* ;
    /// ```
    fn parse_term(&mut self, start: bool) -> Result<ExprKind> {
        let mut expression = self.parse_factor(start)?;

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let rhs = self.parse_factor(false)?;

            let span = expression.span().combine(rhs.span());
            expression = Term::expression(expression, token, rhs, span);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// factor = unary ( ( "/" | "*" ) unary )* ;
    /// ```
    fn parse_factor(&mut self, start: bool) -> Result<ExprKind> {
        let mut expression = self.parse_unary(start)?;

        while let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Slash, TokenKind::Asterisk])
        {
            let rhs = self.parse_unary(false)?;

            let span = expression.span().combine(rhs.span());
            expression = Factor::expression(expression, token, rhs, span);
        }

        Ok(expression)
    }

    /// ```ebnf
    /// unary = ( ( "!" | "-" ) unary )
    ///       | call ;
    /// ```
    fn parse_unary(&mut self, start: bool) -> Result<ExprKind> {
        if let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let expression = self.parse_unary(false)?;

            let span = token.span().combine(expression.span());
            return Ok(Unary::expression(token, expression, span));
        }

        self.parse_call(start)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn parse_call(&mut self, start: bool) -> Result<ExprKind> {
        let mut primary = self.parse_primary(start)?;

        while self.cursor.eat(TokenKind::Parent(true)).is_ok() {
            primary = self.finish_parse_call(primary)?;
        }

        Ok(primary)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn finish_parse_call(&mut self, callee: ExprKind) -> Result<ExprKind> {
        if let Ok(end) = self.cursor.eat(TokenKind::Parent(true)) {
            let span = callee.span().combine(end.span());
            return Ok(Call::expression(callee, Vec::new(), span));
        }

        let mut arguments = Vec::new();
        loop {
            arguments.push(self.parse_expression(false)?);

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        let end = self.cursor.eat(TokenKind::Parent(false))?;

        let span = callee.span().combine(end.span());
        Ok(Call::expression(callee, arguments, span))
    }

    /// ```ebnf
    /// primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expression ")" ;
    /// ```
    fn parse_primary(&mut self, start: bool) -> Result<ExprKind> {
        if let Ok(token) = self.cursor.eat(TokenKind::Int) {
            Ok(Literal::expression(token, LiteralKind::Int))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Decimal) {
            Ok(Literal::expression(token, LiteralKind::Decimal))
        } else if let Ok(token) = self.cursor.eat(TokenKind::String) {
            Ok(Literal::expression(token, LiteralKind::String))
        } else if let Ok(token) = self.cursor.eat(TokenKind::True) {
            Ok(Literal::expression(token, LiteralKind::Bool))
        } else if let Ok(token) = self.cursor.eat(TokenKind::False) {
            Ok(Literal::expression(token, LiteralKind::Bool))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Id) {
            Ok(Id::expression(token))
        } else if let Ok(start) = self.cursor.eat(TokenKind::Parent(true)) {
            let expression = self.parse_expression(false)?;
            let end = self.cursor.eat(TokenKind::Parent(false))?;

            let span = start.span().combine(end.span());
            Ok(Grouping::expression(expression, span))
        } else {
            let token = self.cursor.peek()?;
            Err(DidntExpect::error(
                token,
                "int, decimal, string, true, false, identifier, oparent".to_string(),
            )
            .wrong_start(start))
        }
    }
}
