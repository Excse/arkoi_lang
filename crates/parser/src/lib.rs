use lexer::token::{Token, TokenKind};
use lexer::{Lexer, TokenIter};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<TokenIter<'a>>,
}

#[derive(Debug)]
pub enum LiteralKind<'a> {
    String(&'a str),
    Integer(usize),
    Decimal(f64),
    Boolean(bool),
}

#[derive(Debug)]
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
    DidntExpect(TokenKind<'a>, &'static [TokenKind<'static>]),
    EndOfFile,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            tokens: lexer.iter().peekable(),
        }
    }

    pub fn parse_expression(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        return self.parse_equality();
    }

    fn parse_equality(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_comparison()?;

        while let Some(token) = self.tokens.peek() {
            let token = match token.kind {
                TokenKind::Equal | TokenKind::NotEqual => self.tokens.next().unwrap(),
                _ => break,
            };

            let right = self.parse_comparison()?;
            expression = ExpressionKind::Equality(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_term()?;

        while let Some(token) = self.tokens.peek() {
            let token = match token.kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => self.tokens.next().unwrap(),
                _ => break,
            };

            let right = self.parse_term()?;
            expression = ExpressionKind::Comparison(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_factor()?;

        while let Some(token) = self.tokens.peek() {
            let token = match token.kind {
                TokenKind::Minus | TokenKind::Plus => self.tokens.next().unwrap(),
                _ => break,
            };

            let right = self.parse_factor()?;
            expression = ExpressionKind::Term(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let mut expression = self.parse_unary()?;

        while let Some(token) = self.tokens.peek() {
            let token = match token.kind {
                TokenKind::Slash | TokenKind::Asterisk => self.tokens.next().unwrap(),
                _ => break,
            };

            let right = self.parse_unary()?;
            expression = ExpressionKind::Factor(Box::new(expression), token, Box::new(right));
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        if let Ok(token) = self.matches(&[TokenKind::Apostrophe, TokenKind::Minus]) {
            let right = self.parse_unary()?;
            return Ok(ExpressionKind::Unary(token, Box::new(right)));
        }

        return self.parse_primary();
    }

    fn parse_primary(&mut self) -> Result<ExpressionKind<'a>, ParserError<'a>> {
        let current = self.current()?;
        let expression = Ok(match current.kind {
            TokenKind::Integer(value) => ExpressionKind::Literal(LiteralKind::Integer(value)),
            TokenKind::Decimal(value) => ExpressionKind::Literal(LiteralKind::Decimal(value)),
            TokenKind::String(value) => ExpressionKind::Literal(LiteralKind::String(value)),
            TokenKind::Boolean(value) => ExpressionKind::Literal(LiteralKind::Boolean(value)),
            TokenKind::OParent => {
                let expression = self.parse_expression()?;
                self.matches(&[TokenKind::CParent])?;
                ExpressionKind::Grouping(Box::new(expression))
            }
            token_kind => {
                return Err(ParserError::DidntExpect(
                    token_kind,
                    &[
                        TokenKind::Integer(0),
                        TokenKind::Decimal(0.0),
                        TokenKind::String(""),
                        TokenKind::Boolean(false),
                        TokenKind::OParent,
                    ],
                ))
            }
        });

        self.tokens.next();

        expression
    }

    fn current(&mut self) -> Result<&Token<'a>, ParserError<'a>> {
        self.tokens.peek().ok_or_else(|| ParserError::EndOfFile)
    }

    fn matches(&mut self, expected: &'static [TokenKind]) -> Result<Token<'a>, ParserError<'a>> {
        let token = match self.tokens.peek() {
            Some(token) => token,
            None => return Err(ParserError::EndOfFile),
        };

        if expected.iter().any(|kind| kind == &token.kind) {
            return Ok(self.tokens.next().unwrap());
        }

        Err(ParserError::DidntExpect(token.kind, expected))
    }
}
