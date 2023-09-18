#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{ExpressionKind, Literal, Parameter, Program, StatementKind, Type, TypeKind};
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
    pub fn parse_program(&mut self) -> Program {
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

        Program(statements)
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
        let expression = self.parse_expression()?;

        self.cursor.eat(TokenKind::Semicolon);

        Ok(StatementKind::Expression(expression))
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

        Ok(StatementKind::Block(statements))
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

        Ok(StatementKind::FunDeclaration(identifier, parameters, type_))
    }

    /// ```ebnf
    /// parameters = IDENTIFIER type ( "," IDENTIFIER type )* ;
    /// ```
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();

        loop {
            let identifier = self
                .cursor
                .eat(TokenKind::Identifier)
                .map_err(|error| error.wrong_start(parameters.is_empty()))?;

            let type_ = self.parse_type()?;

            parameters.push(Parameter::new(identifier, type_));

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
        self.cursor
            .eat(TokenKind::At)
            .map_err(|error| error.wrong_start(true))?;

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

        Ok(Type::new(token.kind))
    }

    /// ```ebnf
    /// let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;
    /// ```
    fn parse_let_declaration(&mut self) -> Result<StatementKind> {
        self.cursor
            .eat(TokenKind::Let)
            .map_err(|error| error.wrong_start(true))?;

        let identifier = self.cursor.eat(TokenKind::Identifier)?;

        let expression = match self.cursor.eat(TokenKind::Assign) {
            Ok(_) => Some(self.parse_expression()?),
            Err(_) => None,
        };

        self.cursor.eat(TokenKind::Semicolon)?;

        Ok(StatementKind::LetDeclaration(identifier, expression))
    }

    /// ```ebnf
    /// expression = equality;
    /// ```
    fn parse_expression(&mut self) -> Result<ExpressionKind> {
        self.parse_equality()
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
            let right = self.parse_comparison(false)?;
            expression = ExpressionKind::Equality(Box::new(expression), token, Box::new(right));
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
            let right = self.parse_term(false)?;
            expression = ExpressionKind::Comparison(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    /// ```ebnf
    /// term = factor ( ( "-" | "+" ) factor )* ;
    /// ```
    fn parse_term(&mut self, start: bool) -> Result<ExpressionKind> {
        let mut expression = self.parse_factor(start)?;

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let right = self.parse_factor(false)?;
            expression = ExpressionKind::Term(Box::new(expression), token, Box::new(right));
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
            let right = self.parse_unary(false)?;
            expression = ExpressionKind::Factor(Box::new(expression), token, Box::new(right));
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
            let right = self.parse_unary(false)?;
            return Ok(ExpressionKind::Unary(token, Box::new(right)));
        }

        self.parse_call(start)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn parse_call(&mut self, start: bool) -> Result<ExpressionKind> {
        let mut primary = self.parse_primary(start)?;

        while let Ok(token) = self.cursor.eat(TokenKind::OParent) {
            primary = self.finish_parse_call(Box::new(primary))?;
        }

        Ok(primary)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn finish_parse_call(&mut self, callee: Box<ExpressionKind>) -> Result<ExpressionKind> {
        if self.cursor.eat(TokenKind::CParent).is_ok() {
            return Ok(ExpressionKind::Call(callee, Vec::new()));
        }

        let mut arguments = Vec::new();
        loop {
            arguments.push(self.parse_expression()?);

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        Ok(ExpressionKind::Call(callee, arguments))
    }

    /// ```ebnf
    /// primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expression ")" ;
    /// ```
    fn parse_primary(&mut self, start: bool) -> Result<ExpressionKind> {
        if let Ok(token) = self.cursor.eat(TokenKind::Integer) {
            Ok(ExpressionKind::Literal(Literal::Integer(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Decimal) {
            return Ok(ExpressionKind::Literal(Literal::Decimal(token)));
        } else if let Ok(token) = self.cursor.eat(TokenKind::String) {
            Ok(ExpressionKind::Literal(Literal::String(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::True) {
            Ok(ExpressionKind::Literal(Literal::Boolean(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::False) {
            Ok(ExpressionKind::Literal(Literal::Boolean(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Identifier) {
            Ok(ExpressionKind::Variable(token))
        } else if self.cursor.eat(TokenKind::OParent).is_ok() {
            let expression = self.parse_expression()?;
            self.cursor.eat(TokenKind::CParent)?;
            Ok(ExpressionKind::Grouping(Box::new(expression)))
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
