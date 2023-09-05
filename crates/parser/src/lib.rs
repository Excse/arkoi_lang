pub mod ast;
mod cursor;
mod reports;
pub mod traversel;

use ast::{ExpressionKind, LiteralKind, StatementKind};
use cursor::Cursor;
use diagnostics::Report;
use lexer::token::TokenKind;
use lexer::Lexer;
use serdebug::SerDebug;
use serde::Serialize;

pub struct Parser<'a> {
    cursor: Cursor<'a>,
    pub errors: Vec<ParserError<'a>>,
}

#[derive(SerDebug, Serialize)]
pub enum ParserError<'a> {
    Diagnostic(Report<'a>),
    EndOfFile,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            cursor: Cursor::new(lexer),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Vec<StatementKind<'a>> {
        let mut expressions = Vec::new();

        loop {
            match self.parse_statement() {
                Ok(expression) => {
                    expressions.push(expression);
                }
                Err(ParserError::EndOfFile) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize();
                }
            };
        }

        expressions
    }

    fn parse_statement(&mut self) -> Result<StatementKind<'a>, ParserError<'a>> {
        if self.cursor.consume_if(&[TokenKind::Let]).is_ok() {
            return self.parse_let_declaration();
        }

        if let Ok(expression) = self.parse_expression() {
            self.cursor.consume_if(&[TokenKind::Semicolon])?;
            return Ok(StatementKind::Expression(expression));
        }

        let current = self.cursor.peek()?;
        Err(reports::didnt_expect(current, &[]))
    }

    pub fn parse_let_declaration(&mut self) -> Result<StatementKind<'a>, ParserError<'a>> {
        let identifier = self.cursor.consume_if(&[TokenKind::Identifier("")])?;

        let expression = match self.cursor.consume_if(&[TokenKind::Assign]) {
            Ok(_) => Some(self.parse_expression()?),
            Err(_) => None,
        };

        self.cursor.consume_if(&[TokenKind::Semicolon])?;

        Ok(StatementKind::LetDeclaration(identifier, expression))
    }

    pub fn parse_expression(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        return self.parse_equality();
    }

    fn parse_equality(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_comparison()?;

        while let Ok(token) = self.cursor.peek() {
            let token = match token.kind {
                TokenKind::Equal | TokenKind::NotEqual => self.cursor.consume().unwrap(),
                _ => break,
            };

            let right = self.parse_comparison()?;
            expression = ExpressionKind::Equality(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_term()?;

        while let Ok(token) = self.cursor.peek() {
            let token = match token.kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => self.cursor.consume().unwrap(),
                _ => break,
            };

            let right = self.parse_term()?;
            expression = ExpressionKind::Comparison(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_factor()?;

        while let Ok(token) = self.cursor.peek() {
            let token = match token.kind {
                TokenKind::Minus | TokenKind::Plus => self.cursor.consume().unwrap(),
                _ => break,
            };

            let right = self.parse_factor()?;
            expression = ExpressionKind::Term(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_unary()?;

        while let Ok(token) = self.cursor.peek() {
            let token = match token.kind {
                TokenKind::Slash | TokenKind::Asterisk => self.cursor.consume().unwrap(),
                _ => break,
            };

            let right = self.parse_unary()?;
            expression = ExpressionKind::Factor(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        if let Ok(token) = self
            .cursor
            .consume_if(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let right = self.parse_unary()?;
            return Ok(ExpressionKind::Unary(token, Box::new(right)));
        }

        return self.parse_primary();
    }

    fn parse_primary(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let current = self.cursor.peek()?;
        Ok(match current.kind {
            TokenKind::Integer(_) => {
                let current = self.cursor.consume().unwrap();
                ExpressionKind::Literal(LiteralKind::Integer(current))
            }
            TokenKind::Decimal(_) => {
                let current = self.cursor.consume().unwrap();
                ExpressionKind::Literal(LiteralKind::Decimal(current))
            }
            TokenKind::String(_) => {
                let current = self.cursor.consume().unwrap();
                ExpressionKind::Literal(LiteralKind::String(current))
            }
            TokenKind::Boolean(_) => {
                let current = self.cursor.consume().unwrap();
                ExpressionKind::Literal(LiteralKind::Boolean(current))
            }
            TokenKind::Identifier(_) => {
                let current = self.cursor.consume().unwrap();
                ExpressionKind::Variable(current)
            }
            TokenKind::OParent => {
                self.cursor.consume().unwrap();
                let expression = self.parse_expression()?;
                self.cursor.consume_if(&[TokenKind::CParent])?;
                ExpressionKind::Grouping(Box::new(expression))
            }
            _ => {
                return Err(reports::didnt_expect(
                    current,
                    &[
                        TokenKind::Integer(0),
                        TokenKind::Decimal(0.0),
                        TokenKind::String(""),
                        TokenKind::Boolean(false),
                        TokenKind::Identifier(""),
                        TokenKind::OParent,
                    ],
                ))
            }
        })
    }
}
