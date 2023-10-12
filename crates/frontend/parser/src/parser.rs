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
        let mut stmts = Vec::new();
        loop {
            match self.parse_program_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(ParserError::InternalError(InternalError::EndOfFile(_))) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize_program();
                }
            };
        }

        let span = match (stmts.first(), stmts.last()) {
            (Some(first), Some(last)) => first.span().combine(&last.span()),
            (_, _) => LabelSpan::default(),
        };

        Program::new(stmts, span)
    }

    /// ```ebnf
    /// program_stmts = fun_declaration
    ///                    | let_declaration ;
    /// ```
    fn parse_program_stmt(&mut self) -> Result<StmtKind> {
        if let Some(result) = self.try_parse_fun_decl()? {
            return Ok(result);
        }

        if let Some(result) = self.try_parse_let_decl()? {
            return Ok(result);
        }

        let token = self.cursor.peek()?;
        Err(Unexpected::new(token.kind.to_string(), token.span, "fun or let declaration").into())
    }

    /// ```ebnf
    /// stmt = expr_statement
    ///           | block ;
    /// ```
    fn try_parse_stmt(&mut self) -> Result<StmtKind> {
        if let Some(result) = self.try_parse_expr_stmt()? {
            return Ok(result);
        }

        if let Some(result) = self.try_parse_block()? {
            return Ok(result);
        }

        let token = self.cursor.peek()?;
        Err(Unexpected::new(token.kind.to_string(), token.span, "expr stmt or block").into())
    }

    /// ```ebnf
    /// expr_stmt = expression ";" ;
    /// ```
    fn try_parse_expr_stmt(&mut self) -> Result<Option<StmtKind>> {
        let expr = match self.try_parse_expr()? {
            Some(expr) => expr,
            None => return Ok(None),
        };

        self.cursor.eat(TokenKind::Semicolon)?;

        Ok(Some(ExprStmt::new(expr).into()))
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

        let mut stmts = Vec::new();
        loop {
            if self.cursor.is_peek(TokenKind::Brace(false)).is_some() {
                break;
            }

            match self.parse_block_decl() {
                Ok(stmt) => stmts.push(stmt),
                Err(ParserError::InternalError(InternalError::EndOfFile(_))) => break,
                Err(error) => {
                    self.errors.push(error);
                    self.cursor.synchronize_block();
                }
            };
        }

        let end = self.cursor.eat(TokenKind::Brace(false))?;

        let span = start.span.combine(&end.span);
        Ok(Some(Block::new(stmts, span).into()))
    }

    fn parse_block(&mut self) -> Result<StmtKind> {
        match self.try_parse_block()? {
            Some(stmt) => Ok(stmt),
            None => Err(UnoptionalParsing.into()),
        }
    }

    /// ```ebnf
    /// block_declaration = let_declaration
    ///                   | return_stmt
    ///                   | stmt ;
    /// ```
    fn parse_block_decl(&mut self) -> Result<StmtKind> {
        if let Some(result) = self.try_parse_let_decl()? {
            return Ok(result);
        }

        if let Some(result) = self.try_parse_return_stmt()? {
            return Ok(result);
        }

        if let Ok(result) = self.try_parse_stmt() {
            return Ok(result);
        }

        let token = self.cursor.peek()?;
        Err(Unexpected::new(token.kind.to_string(), token.span, "stmt, let declaration").into())
    }

    /// ```ebnf
    /// return_stmt = return expr? ";" ;
    /// ```
    fn try_parse_return_stmt(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Return) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let expr = self.try_parse_expr()?;

        let end = self.cursor.eat(TokenKind::Semicolon)?;

        let span = start.span.combine(&end.span);
        Ok(Some(Return::new(expr, span).into()))
    }

    /// ```ebnf
    /// fun_declaration = "fun" IDENTIFIER "(" parameters? ")" type block ;
    /// ```
    fn try_parse_fun_decl(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Fun) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let id = self.cursor.eat(TokenKind::Id)?;

        self.cursor.eat(TokenKind::Parent(true))?;

        let params = match self.cursor.eat(TokenKind::Parent(false)) {
            Err(_) => {
                let params = self.parse_params()?;

                self.cursor.eat(TokenKind::Parent(false))?;

                params
            }
            _ => Vec::new(),
        };

        let type_ = self.parse_type()?;

        let block = match self.parse_block()? {
            StmtKind::Block(node) => node,
            _ => panic!("Couldn't unbox the block. This shouldn't have happened."),
        };

        let span = start.span.combine(&block.span);
        Ok(Some(FunDecl::new(id, params, type_, block, span).into()))
    }

    /// ```ebnf
    /// parameters = IDENTIFIER type ( "," IDENTIFIER type )* ;
    /// ```
    fn parse_params(&mut self) -> Result<Vec<Parameter>> {
        let mut params = Vec::new();

        loop {
            let id = self.cursor.eat(TokenKind::Id)?;
            let type_ = self.parse_type()?;

            let span = id.span.combine(&type_.span);
            params.push(Parameter::new(id, type_, span));

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        Ok(params)
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
    /// let_declaration = "let" IDENTIFIER ( "=" expr )? ";" ;
    /// ```
    fn try_parse_let_decl(&mut self) -> Result<Option<StmtKind>> {
        let start = match self.cursor.eat(TokenKind::Let) {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let id = self.cursor.eat(TokenKind::Id)?;

        let type_ = self.parse_type()?;

        let expr = match self.cursor.eat(TokenKind::Eq) {
            Ok(_) => Some(self.parse_expr()?),
            Err(_) => None,
        };

        let end = self.cursor.eat(TokenKind::Semicolon)?;

        let span = start.span.combine(&end.span);
        Ok(Some(LetDecl::new(id, type_, expr, span).into()))
    }

    /// ```ebnf
    /// expr = equality;
    /// ```
    fn try_parse_expr(&mut self) -> Result<Option<ExprKind>> {
        self.try_parse_equality(true)
    }

    /// ```ebnf
    /// expr = equality;
    /// ```
    fn parse_expr(&mut self) -> Result<ExprKind> {
        self.parse_equality()
    }

    /// ```ebnf
    /// equality = comparison ( ( "==" | "!=" ) comparison )* ;
    /// ```
    fn try_parse_equality(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expr = match self.try_parse_comparison(start)? {
            Some(expr) => expr,
            None => return Ok(None),
        };

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::EqEq, TokenKind::NotEq]) {
            let rhs = self.parse_comparison()?;

            let span = expr.span().combine(&rhs.span());
            expr = Equality::new(expr, token, rhs, span).into();
        }

        Ok(Some(expr))
    }

    fn parse_equality(&mut self) -> Result<ExprKind> {
        match self.try_parse_equality(false)? {
            Some(expr) => Ok(expr),
            None => Err(UnoptionalParsing.into()),
        }
    }

    /// ```ebnf
    /// comparison = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    /// ```
    fn try_parse_comparison(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expr = match self.try_parse_term(start)? {
            Some(expr) => expr,
            None => return Ok(None),
        };

        while let Ok(token) = self.cursor.eat_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEq,
            TokenKind::Less,
            TokenKind::LessEq,
        ]) {
            let rhs = self.parse_term()?;

            let span = expr.span().combine(&rhs.span());
            expr = Comparison::new(expr, token, rhs, span).into();
        }

        Ok(Some(expr))
    }

    fn parse_comparison(&mut self) -> Result<ExprKind> {
        match self.try_parse_comparison(false)? {
            Some(expr) => Ok(expr),
            None => Err(UnoptionalParsing.into()),
        }
    }

    /// ```ebnf
    /// term = factor ( ( "-" | "+" ) factor )* ;
    /// ```
    fn try_parse_term(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expr = match self.try_parse_factor(start)? {
            Some(expr) => expr,
            None => return Ok(None),
        };

        while let Ok(token) = self.cursor.eat_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let rhs = self.parse_factor()?;

            let span = expr.span().combine(&rhs.span());
            expr = Term::new(expr, token, rhs, span).into();
        }

        Ok(Some(expr))
    }

    fn parse_term(&mut self) -> Result<ExprKind> {
        match self.try_parse_term(false)? {
            Some(expr) => Ok(expr),
            None => Err(UnoptionalParsing.into()),
        }
    }

    /// ```ebnf
    /// factor = unary ( ( "/" | "*" ) unary )* ;
    /// ```
    fn try_parse_factor(&mut self, start: bool) -> Result<Option<ExprKind>> {
        let mut expr = match self.try_parse_unary(start)? {
            Some(expr) => expr,
            None => return Ok(None),
        };

        while let Ok(token) = self
            .cursor
            .eat_any(&[TokenKind::Slash, TokenKind::Asterisk])
        {
            let rhs = self.parse_unary()?;

            let span = expr.span().combine(&rhs.span());
            expr = Factor::new(expr, token, rhs, span).into();
        }

        Ok(Some(expr))
    }

    fn parse_factor(&mut self) -> Result<ExprKind> {
        match self.try_parse_factor(false)? {
            Some(expr) => Ok(expr),
            None => Err(UnoptionalParsing.into()),
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
            let expr = self.parse_unary()?;

            let span = token.span.combine(&expr.span());
            return Ok(Some(Unary::new(token, expr, span).into()));
        }

        self.try_parse_call(start)
    }

    fn parse_unary(&mut self) -> Result<ExprKind> {
        match self.try_parse_unary(false)? {
            Some(expr) => Ok(expr),
            None => Err(UnoptionalParsing.into()),
        }
    }

    ///```ebnf
    /// call = primary ( "(" args? ")" )* ;
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
    /// call = primary ( "(" args? ")" )* ;
    ///```
    fn finish_parse_call(&mut self, callee: ExprKind) -> Result<ExprKind> {
        if let Ok(end) = self.cursor.eat(TokenKind::Parent(true)) {
            let span = callee.span().combine(&end.span);
            return Ok(Call::new(callee, Vec::new(), span).into());
        }

        let mut args = Vec::new();
        loop {
            args.push(self.parse_expr()?);

            if self.cursor.eat(TokenKind::Comma).is_err() {
                break;
            }
        }

        let end = self.cursor.eat(TokenKind::Parent(false))?;

        let span = callee.span().combine(&end.span);
        Ok(Call::new(callee, args, span).into())
    }

    /// ```ebnf
    /// primary = NUMBER | STRING | IDENTIFIER | "true" | "false" | "(" expr ")" ;
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
            let expr = self.parse_expr()?;
            let end = self.cursor.eat(TokenKind::Parent(false))?;

            let span = start.span.combine(&end.span);
            Ok(Some(Grouping::new(expr, span).into()))
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
