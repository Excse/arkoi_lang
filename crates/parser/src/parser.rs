#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::ast::{ExpressionKind, LiteralKind, Parameter, Program, StatementKind, Type};
use crate::cursor::Cursor;
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
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub enum ParserError {
    Report(Report),
    WrongStart,
    EndOfFile,
}

type ParserResult<T> = Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new(files: &'a Files, file_id: FileID, lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            cursor: Cursor::new(files, file_id, lexer),
            files,
            file_id,
            errors: Vec::new(),
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
                Err(ParserError::EndOfFile) => break,
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
    fn parse_declaration(&mut self) -> ParserResult<StatementKind> {
        if let Parsable::Yes(result) = self.parse_fun_declaration() {
            return result;
        }

        if let Parsable::Yes(result) = self.parse_let_declaration() {
            return result;
        }

        self.parse_statement()
    }

    /// ```ebnf
    /// statement = expression_statement
    ///           | block ;
    /// ```
    fn parse_statement(&mut self) -> ParserResult<StatementKind> {
        if let Parsable::Yes(result) = self.parse_block() {
            return result;
        }

        if let Ok(expression) = self.parse_expression() {
            self.cursor.eat(TokenKind::Semicolon)?;
            return Ok(StatementKind::Expression(expression));
        }

        let token = self.cursor.peek()?;
        Err(ParserError::Report(didnt_expect(
            self.files,
            self.file_id,
            Spannable::new(token.kind.as_ref(), token.span),
            "",
        )))
    }

    fn parse_expression_statement(&mut self) -> ParserResult<StatementKind> {
        let expression = self.parse_expression()?;

        self.cursor.eat(TokenKind::Semicolon);

        Ok(StatementKind::Expression(expression))
    }

    fn parse_block(&mut self) -> ParserResult<StatementKind> {
        self.cursor.eat(TokenKind::OBracket)?;

        let mut statements = Vec::new();
        loop {
            match self.parse_declaration() {
                Ok(expression) => {
                    statements.push(expression);
                }
                Err(ParserError::EndOfFile) => break,
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
    fn parse_fun_declaration(&mut self) -> ParserResult<StatementKind> {
        self.cursor.eat(TokenKind::Fun)?;

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
    fn parse_parameters(&mut self) -> ParserResult<Vec<Parameter>> {
        let mut parameters = Vec::new();

        loop {
            let identifier = self.cursor.eat(TokenKind::Identifier)?;
            let type_ = self.parse_type()?;

            parameters.push(Parameter::new(identifier, type_));

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_type(&mut self) -> ParserResult<Type> {
        todo!()
    }

    /// ```ebnf
    /// let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;
    /// ```
    fn parse_let_declaration(&mut self) -> ParserResult<StatementKind> {
        self.cursor.eat(TokenKind::Let)?;

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
    fn parse_expression(&mut self) -> ParserResult<ExpressionKind> {
        self.parse_equality()
    }

    /// ```ebnf
    /// equality = comparison ( ( "==" | "!=" ) comparison )* ;
    /// ```
    fn parse_equality(&mut self) -> ParserResult<ExpressionKind> {
        let mut expression = self.parse_comparison()?;

        while let Ok(token) = self
            .cursor
            .eat_all(&[TokenKind::Equal, TokenKind::NotEqual])
        {
            let right = self.parse_comparison()?;
            expression = ExpressionKind::Equality(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    /// ```ebnf
    /// comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    /// ```
    fn parse_comparison(&mut self) -> ParserResult<ExpressionKind> {
        let mut expression = self.parse_term()?;

        while let Ok(token) = self.cursor.eat_all(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let right = self.parse_term()?;
            expression = ExpressionKind::Comparison(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    /// ```ebnf
    /// term = factor ( ( "-" | "+" ) factor )* ;
    /// ```
    fn parse_term(&mut self) -> ParserResult<ExpressionKind> {
        let mut expression = self.parse_factor()?;

        while let Ok(token) = self.cursor.eat_all(&[TokenKind::Plus, TokenKind::Minus]) {
            let right = self.parse_factor()?;
            expression = ExpressionKind::Term(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    /// ```ebnf
    /// factor = unary ( ( "/" | "*" ) unary )* ;
    /// ```
    fn parse_factor(&mut self) -> ParserResult<ExpressionKind> {
        let mut expression = self.parse_unary()?;

        while let Ok(token) = self
            .cursor
            .eat_all(&[TokenKind::Slash, TokenKind::Asterisk])
        {
            let right = self.parse_unary()?;
            expression = ExpressionKind::Factor(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    /// ```ebnf
    /// unary = ( ( "!" | "-" ) unary )
    ///       | call ;
    /// ```
    fn parse_unary(&mut self) -> ParserResult<ExpressionKind> {
        if let Ok(token) = self
            .cursor
            .eat_all(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let right = self.parse_unary()?;
            return Ok(ExpressionKind::Unary(token, Box::new(right)));
        }

        self.parse_call()
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn parse_call(&mut self) -> ParserResult<ExpressionKind> {
        let mut primary = self.parse_primary()?;

        while let Ok(token) = self.cursor.eat(TokenKind::OParent) {
            primary = self.finish_parse_call(Box::new(primary))?;
        }

        Ok(primary)
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn finish_parse_call(&mut self, callee: Box<ExpressionKind>) -> ParserResult<ExpressionKind> {
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
    fn parse_primary(&mut self) -> ParserResult<ExpressionKind> {
        if let Ok(token) = self.cursor.eat(TokenKind::Integer) {
            Ok(ExpressionKind::Literal(LiteralKind::Integer(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Decimal) {
            return Ok(ExpressionKind::Literal(LiteralKind::Decimal(token)));
        } else if let Ok(token) = self.cursor.eat(TokenKind::String) {
            Ok(ExpressionKind::Literal(LiteralKind::String(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::True) {
            Ok(ExpressionKind::Literal(LiteralKind::Boolean(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::False) {
            Ok(ExpressionKind::Literal(LiteralKind::Boolean(token)))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Identifier) {
            Ok(ExpressionKind::Variable(token))
        } else if self.cursor.eat(TokenKind::OParent).is_ok() {
            let expression = self.parse_expression()?;
            self.cursor.eat(TokenKind::CParent)?;
            Ok(ExpressionKind::Grouping(Box::new(expression)))
        } else {
            let token = self.cursor.peek()?;
            Err(ParserError::Report(didnt_expect(
                self.files,
                self.file_id,
                Spannable::new(token.kind.as_ref(), token.span),
                "int, decimal, string, true, false, identifier, oparent",
            )))
        }
    }
}
