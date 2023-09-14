pub mod ast;
pub mod traversel;

mod cursor;

use diagnostics::file::{FileID, Files};
use diagnostics::positional::Spannable;
use diagnostics::report::Report;
use errors::parser::didnt_expect;
use serde::Serialize;
use serdebug::SerDebug;

use ast::{ExpressionKind, LiteralKind, StatementKind};
use cursor::Cursor;
use lexer::token::TokenKind;
use lexer::Lexer;

pub struct Parser<'a> {
    cursor: Cursor<'a>,
    files: &'a Files,
    file_id: FileID,
    pub errors: Vec<ParserError>,
}

#[derive(SerDebug, Serialize)]
pub enum ParserError {
    Report(Report),
    EndOfFile,
}

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
    /// program = statement* EOF ;
    /// ```
    pub fn parse_program(&mut self) -> Vec<StatementKind<'a>> {
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

        statements
    }

    /// ```ebnf
    /// declaration = let_declaration
    ///             | statement ;
    /// ```
    fn parse_declaration(&mut self) -> Result<StatementKind<'a>, ParserError> {
        if self.cursor.is_peek(TokenKind::Let).is_some() {
            return self.parse_let_declaration();
        }

        self.parse_statement()
    }

    /// ```ebnf
    /// statement = expression_statement ;
    /// ```
    fn parse_statement(&mut self) -> Result<StatementKind<'a>, ParserError> {
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

    /// ```ebnf
    /// let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;
    /// ```
    fn parse_let_declaration(&mut self) -> Result<StatementKind<'a>, ParserError> {
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
    fn parse_expression(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
        self.parse_equality()
    }

    /// ```ebnf
    /// equality = comparison ( ( "==" | "!=" ) comparison )* ;
    /// ```
    fn parse_equality(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
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
    fn parse_comparison(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
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
    fn parse_term(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
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
    fn parse_factor(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
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
    /// unary = ( "!" | "-" ) unary
    ///       | primary ;
    /// ```
    fn parse_unary(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
        if let Ok(token) = self
            .cursor
            .eat_all(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let right = self.parse_unary()?;
            return Ok(ExpressionKind::Unary(token, Box::new(right)));
        }

        self.parse_primary()
    }

    /// ```ebnf
    /// primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expression ")" ;
    /// ```
    fn parse_primary(&mut self) -> Result<ExpressionKind<'a>, ParserError> {
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
