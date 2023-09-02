mod cursor;
mod reports;

use serde::Serialize;

use lexer::token::{Token, TokenKind};
use diagnostics::Report;
use cursor::Cursor;
use lexer::Lexer;

pub struct Parser<'a> {
    cursor: Cursor<'a>,
    pub errors: Vec<ParserError<'a>>,
}

#[derive(Debug, Serialize)]
pub enum LiteralKind<'a> {
    String(&'a str),
    Integer(usize),
    Decimal(f64),
    Boolean(bool),
    Identifier(&'a str),
}

#[derive(Debug, Serialize)]
pub enum StatementKind<'a> {
    ExpressionStatement(ExpressionKind<'a>),
}

#[derive(Debug, Serialize)]
pub enum ExpressionKind<'a> {
    Equality(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Comparison(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Term(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Factor(Box<ExpressionKind<'a>>, Token<'a>, Box<ExpressionKind<'a>>),
    Unary(Token<'a>, Box<ExpressionKind<'a>>),
    Grouping(Box<ExpressionKind<'a>>),
    Literal(LiteralKind<'a>),
}

#[derive(Debug)]
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
        if let Ok(expression) = self.parse_expression() {
            self.cursor.matches(&[TokenKind::Semicolon])?;
            return Ok(StatementKind::ExpressionStatement(expression));
        }

        let current = self.cursor.peek()?;
        Err(reports::didnt_expect(current, &[]))
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
            .matches(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let right = self.parse_unary()?;
            return Ok(ExpressionKind::Unary(token, Box::new(right)));
        }

        return self.parse_primary();
    }

    fn parse_primary(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let current = self.cursor.peek()?;
        Ok(match current.kind {
            TokenKind::Integer(value) => {
                self.cursor.consume();
                ExpressionKind::Literal(LiteralKind::Integer(value))
            }
            TokenKind::Decimal(value) => {
                self.cursor.consume();
                ExpressionKind::Literal(LiteralKind::Decimal(value))
            }
            TokenKind::String(value) => {
                self.cursor.consume();
                ExpressionKind::Literal(LiteralKind::String(value))
            }
            TokenKind::Boolean(value) => {
                self.cursor.consume();
                ExpressionKind::Literal(LiteralKind::Boolean(value))
            }
            TokenKind::Identifier(value) => {
                self.cursor.consume();
                ExpressionKind::Literal(LiteralKind::Identifier(value))
            }
            TokenKind::OParent => {
                self.cursor.consume();
                let expression = self.parse_expression()?;
                self.cursor.matches(&[TokenKind::CParent])?;
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
