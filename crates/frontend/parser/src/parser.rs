#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::cursor::Cursor;
use crate::error::{InternalError, ParserError, Result, Unexpected, UnoptionalParsing};
use ast::{
    Block, Call, Comparison, Equality, ExprKind, ExprStmt, Factor, FunDecl, Grouping, Id, LetDecl,
    Literal, LiteralKind, Parameter, Program, Return, StmtKind, Term, Type, Unary,
};
use diagnostics::positional::LabelSpan;
use lexer::iterator::TokenIterator;
use lexer::token::TokenKind;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(iterator: TokenIterator<'a>) -> Parser<'a> {
        Self {
            cursor: Cursor::new(iterator),
            errors: Vec::new(),
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
                Err(ParserError::InternalError(InternalError::EndOfFile(_))) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize_program();
                }
            };
        }

        let span = if let Some(start) = statements.first() {
            let end = statements.last().unwrap();
            start.span().combine(&end.span())
        } else {
            LabelSpan::default()
        };

        Program::new(statements, span)
    }

    /// ```ebnf
    /// program_statements = fun_declaration
    ///                    | let_declaration ;
    /// ```
    fn parse_program_declaration(&mut self) -> Result<StmtKind> {
        match self.try_parse_let_declaration() {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}
            Err(error) => return Err(error),
        }

        match self.try_parse_fun_declaration() {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}
            Err(error) => return Err(error),
        }

        let token = self.cursor.peek()?;
        Err(Unexpected::new(token.kind.to_string(), token.span, "fun or let declaration").into())
    }

    /// ```ebnf
    /// statement = expression_statement
    ///           | block ;
    /// ```
    fn parse_statement(&mut self) -> Result<StmtKind> {
        match self.try_parse_expression_statement() {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}
            Err(error) => return Err(error),
        }

        match self.try_parse_block() {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}
            Err(error) => return Err(error),
        }

        let token = self.cursor.peek()?;
        Err(Unexpected::new(
            token.kind.to_string(),
            token.span,
            "expression statement or block",
        )
        .into())
    }

    /// ```ebnf
    /// expression_statement = expression ";" ;
    /// ```
    fn try_parse_expression_statement(&mut self) -> Result<Option<StmtKind>> {
        let expression = match self.try_parse_expression() {
            Ok(Some(expression)) => expression,
            Ok(None) => return Ok(None),
            Err(error) => return Err(error),
        };

        self.cursor.eat(TokenKind::Semicolon)?;

        Ok(Some(ExprStmt::new(expression).into()))
    }

    /// ```ebnf
    /// block = "{" block_declaration* "}" ;
    /// ```
    fn try_parse_block(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Brace(true)) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        if let Ok(end) = self.cursor.eat(TokenKind::Brace(false)) {
            let span = start.span.combine(&end.span);
            return Ok(Some(Block::new(Vec::new(), span).into()));
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
                Err(ParserError::InternalError(InternalError::EndOfFile(_))) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize_block();
                }
            };
        }

        let end = self.cursor.eat(TokenKind::Brace(false))?;

        let span = start.span.combine(&end.span);
        Ok(Some(Block::new(statements, span).into()))
    }

    fn parse_block(&mut self) -> Result<StmtKind> {
        match self.try_parse_block() {
            Ok(Some(statement)) => Ok(statement),
            Ok(None) => Err(UnoptionalParsing.into()),
            Err(error) => Err(error),
        }
    }

    /// ```ebnf
    /// block_declaration = let_declaration
    ///                   | return_statement
    ///                   | statement ;
    /// ```
    fn parse_block_declaration(&mut self) -> Result<StmtKind> {
        match self.try_parse_let_declaration() {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}
            Err(error) => return Err(error),
        }

        match self.try_parse_return_statement() {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}
            Err(error) => return Err(error),
        }

        if let Ok(result) = self.parse_statement() {
            return Ok(result);
        }

        let token = self.cursor.peek()?;
        Err(Unexpected::new(
            token.kind.to_string(),
            token.span,
            "statement, let declaration",
        )
        .into())
    }

    /// ```ebnf
    /// return_statement = return expression? ";" ;
    /// ```
    fn try_parse_return_statement(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Return) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let expression = match self.try_parse_expression() {
            Ok(Some(expression)) => Some(expression),
            Ok(None) => None,
            Err(error) => return Err(error),
        };

        let end = self.cursor.eat(TokenKind::Semicolon)?;

        let span = start.span.combine(&end.span);
        Ok(Some(Return::new(expression, span).into()))
    }

    /// ```ebnf
    /// fun_declaration = "fun" IDENTIFIER "(" parameters? ")" type block ;
    /// ```
    fn try_parse_fun_declaration(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Fun) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

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

        let span = start.span.combine(&block.span);
        Ok(Some(
            FunDecl::new(identifier, parameters, type_, block, span).into(),
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

            let span = id.span.combine(&type_.span);
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

        let span = start.span.combine(&token.span);
        Ok(Type::new(token.kind, span))
    }

    /// ```ebnf
    /// let_declaration = "let" IDENTIFIER ( "=" expression )? ";" ;
    /// ```
    fn try_parse_let_declaration(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Let) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let name = self.cursor.eat(TokenKind::Id)?;

        let type_ = self.parse_type()?;

        let expression = match self.cursor.eat(TokenKind::Eq) {
            Ok(_) => Some(self.parse_expression()?),
            Err(_) => None,
        };

        let end = self.cursor.eat(TokenKind::Semicolon)?;

        let span = start.span.combine(&end.span);
        Ok(Some(LetDecl::new(name, type_, expression, span).into()))
    }

    /// ```ebnf
    /// expression = equality;
    /// ```
    fn try_parse_expression(&mut self) -> Result<Option<ExprKind>> {
        self.try_parse_equality(true)
    }

    /// ```ebnf
    /// expression = equality;
    /// ```
    fn parse_expression(&mut self) -> Result<ExprKind> {
        self.parse_equality()
    }

    /// ```ebnf
    /// equality = comparison ( ( "==" | "!=" ) comparison )* ;
    /// ```
    fn try_parse_equality(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expression = match self.try_parse_comparison(start)? {
            Some(expression) => expression,
            None => return Ok(None),
        };

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::EqEq, TokenKind::NotEq]) {
            let rhs = self.parse_comparison()?;

            let span = expression.span().combine(&rhs.span());
            expression = Equality::new(expression, token, rhs, span).into();
        }

        Ok(Some(expression))
    }

    fn parse_equality(&mut self) -> Result<ExprKind> {
        match self.try_parse_equality(false) {
            Ok(Some(expression)) => Ok(expression),
            Ok(None) => Err(UnoptionalParsing.into()),
            Err(error) => Err(error),
        }
    }

    /// ```ebnf
    /// comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    /// ```
    fn try_parse_comparison(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expression = match self.try_parse_term(start)? {
            Some(expression) => expression,
            None => return Ok(None),
        };

        while let Ok(token) = self.cursor.eat_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEq,
            TokenKind::Less,
            TokenKind::LessEq,
        ]) {
            let rhs = self.parse_term()?;

            let span = expression.span().combine(&rhs.span());
            expression = Comparison::new(expression, token, rhs, span).into();
        }

        Ok(Some(expression))
    }

    fn parse_comparison(&mut self) -> Result<ExprKind> {
        match self.try_parse_comparison(false) {
            Ok(Some(expression)) => Ok(expression),
            Ok(None) => Err(UnoptionalParsing.into()),
            Err(error) => Err(error),
        }
    }

    /// ```ebnf
    /// term = factor ( ( "-" | "+" ) factor )* ;
    /// ```
    fn try_parse_term(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expression = match self.try_parse_factor(start)? {
            Some(expression) => expression,
            None => return Ok(None),
        };

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let rhs = self.parse_factor()?;

            let span = expression.span().combine(&rhs.span());
            expression = Term::new(expression, token, rhs, span).into();
        }

        Ok(Some(expression))
    }

    fn parse_term(&mut self) -> Result<ExprKind> {
        match self.try_parse_term(false) {
            Ok(Some(expression)) => Ok(expression),
            Ok(None) => Err(UnoptionalParsing.into()),
            Err(error) => Err(error),
        }
    }

    /// ```ebnf
    /// factor = unary ( ( "/" | "*" ) unary )* ;
    /// ```
    fn try_parse_factor(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expression = match self.try_parse_unary(start)? {
            Some(expression) => expression,
            None => return Ok(None),
        };

        while let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Slash, TokenKind::Asterisk])
        {
            let rhs = self.parse_unary()?;

            let span = expression.span().combine(&rhs.span());
            expression = Factor::new(expression, token, rhs, span).into();
        }

        Ok(Some(expression))
    }

    fn parse_factor(&mut self) -> Result<ExprKind> {
        match self.try_parse_factor(false) {
            Ok(Some(expression)) => Ok(expression),
            Ok(None) => Err(UnoptionalParsing.into()),
            Err(error) => Err(error),
        }
    }

    /// ```ebnf
    /// unary = ( ( "!" | "-" ) unary )
    ///       | call ;
    /// ```
    fn try_parse_unary(&mut self, start: bool) -> Result<Option<ExprKind>> {
        if let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Apostrophe, TokenKind::Minus])
        {
            let expression = self.parse_unary()?;

            let span = token.span.combine(&expression.span());
            return Ok(Some(Unary::new(token, expression, span).into()));
        }

        self.try_parse_call(start)
    }

    fn parse_unary(&mut self) -> Result<ExprKind> {
        match self.try_parse_unary(false) {
            Ok(Some(expression)) => Ok(expression),
            Ok(None) => Err(UnoptionalParsing.into()),
            Err(error) => Err(error),
        }
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn try_parse_call(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut primary = match self.try_parse_primary(start)? {
            Some(primary) => primary,
            None => return Ok(None),
        };

        while self.cursor.eat(TokenKind::Parent(true)).is_ok() {
            primary = self.finish_parse_call(primary)?;
        }

        Ok(Some(primary))
    }

    ///```ebnf
    /// call = primary ( "(" arguments? ")" )* ;
    ///```
    fn finish_parse_call(&mut self, callee: ExprKind) -> Result<ExprKind> {
        if let Ok(end) = self.cursor.eat(TokenKind::Parent(true)) {
            let span = callee.span().combine(&end.span);
            return Ok(Call::new(callee, Vec::new(), span).into());
        }

        let mut arguments = Vec::new();
        loop {
            arguments.push(self.parse_expression()?);

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        let end = self.cursor.eat(TokenKind::Parent(false))?;

        let span = callee.span().combine(&end.span);
        Ok(Call::new(callee, arguments, span).into())
    }

    /// ```ebnf
    /// primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expression ")" ;
    /// ```
    fn try_parse_primary(&mut self, start: bool) -> Result<Option<ExprKind>> {
        if let Ok(token) = self.cursor.eat(TokenKind::Int) {
            Ok(Some(Literal::new(token, LiteralKind::Int).into()))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Decimal) {
            Ok(Some(Literal::new(token, LiteralKind::Decimal).into()))
        } else if let Ok(token) = self.cursor.eat(TokenKind::String) {
            Ok(Some(Literal::new(token, LiteralKind::String).into()))
        } else if let Ok(token) = self.cursor.eat(TokenKind::True) {
            Ok(Some(Literal::new(token, LiteralKind::Bool).into()))
        } else if let Ok(token) = self.cursor.eat(TokenKind::False) {
            Ok(Some(Literal::new(token, LiteralKind::Bool).into()))
        } else if let Ok(token) = self.cursor.eat(TokenKind::Id) {
            Ok(Some(Id::new(token).into()))
        } else if let Ok(start) = self.cursor.eat(TokenKind::Parent(true)) {
            let expression = self.parse_expression()?;
            let end = self.cursor.eat(TokenKind::Parent(false))?;

            let span = start.span.combine(&end.span);
            Ok(Some(Grouping::new(expression, span).into()))
        } else if start {
            Ok(None)
        } else {
            let token = self.cursor.peek()?;
            Err(ParserError::from(Unexpected::new(
                token.kind.to_string(),
                token.span,
                "int, decimal, string, true, false, identifier, oparent".to_string(),
            )))
        }
    }
}
