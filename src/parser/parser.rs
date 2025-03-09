use std::{any::Any, iter::Peekable};

use super::grammar::{
    Block, DeclStmt, FuncDeclStmt, IdentifierExpr, Program, Statement, StructDeclStmt,
};

use crate::lexer::token::{Token, TokenType};

use super::error::ParseError;

type ParseResult<T> = Result<T, ParseError>;

pub struct MathParser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current: u16,
}

impl MathParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens.into_iter().peekable();

        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Program> {
        let mut functions = vec![];

        while let Some(tk) = self.next() {
            match tk.r#type {
                TokenType::Def => functions.push(self.function_declaration()?),
                TokenType::Struct => self.struct_declaration()?,
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        tk.line,
                        tk.start,
                        Box::from(
                            format!(
                                "Unexpected token {}: expected function or struct declarations",
                                tk,
                            )
                            .as_str(),
                        ),
                    ))
                }
            }
        }

        let program = Program { functions };

        Ok(program)
    }

    #[allow(unused)]
    fn struct_declaration(&mut self) -> ParseResult<()> {
        Ok(())
    }

    fn function_declaration(&mut self) -> ParseResult<FuncDeclStmt> {
        let function_name = self.consume_token(
            TokenType::Identifier,
            "Expected function name after 'def' keyword.",
        )?;

        self.consume_token(TokenType::LeftParen, "Expected '(' after function name")?;

        let params = self.function_params()?;

        self.consume_token(TokenType::RightParen, "Expected ')' after function name")?;

        let body = self.block()?;

        let return_type = if self.next_if(TokenType::Arrow).is_some() {
            let ty = self.consume_token(TokenType::Type, "Expected type after '->'")?;

            Some(ty.lexeme.unwrap().into())
        } else {
            None
        };

        Ok(FuncDeclStmt {
            name: function_name.lexeme.unwrap(),
            params,
            body,
            return_type,
        })
    }

    fn function_params(&mut self) -> ParseResult<Option<Vec<IdentifierExpr>>> {
        if let TokenType::Identifier = self.next_tk_type()? {
            let mut parameters = Vec::new();

            parameters.push(self.identifer_type()?);

            while self.next_if(TokenType::Comma).is_some() {
                parameters.push(self.identifer_type()?);
            }

            return Ok(Some(parameters));
        }

        Ok(None)
    }

    fn identifer_type(&mut self) -> ParseResult<IdentifierExpr> {
        let param_name = self.consume_token(TokenType::Identifier, "Expected identifier name")?;

        self.consume_token(TokenType::Colon, "Expected ':' after identifier name")?;

        let param_type =
            self.consume_token(TokenType::Type, "Expected parameter type after ':'")?;

        Ok(IdentifierExpr {
            ident_name: param_name.lexeme.unwrap(),
            ty: param_type.lexeme.unwrap().into(),
        })
    }

    fn block(&mut self) -> ParseResult<Block> {
        let stmts = Vec::new();

        Ok(Block { stmts })
    }

    fn consume_token(&mut self, ty: TokenType, err: &str) -> ParseResult<Token> {
        let next_token = self.next();

        if let Some(t) = next_token {
            if t.r#type == ty {
                return Ok(t);
            }

            return Err(ParseError::UnexpectedToken(t.line, t.start, Box::from("")));
        }

        Err(ParseError::UnexpectedEnd)
    }

    fn next_tk_type(&mut self) -> ParseResult<TokenType> {
        match self.peek() {
            Some(tk) => Ok(tk.r#type.clone()),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn next_if(&mut self, ty: TokenType) -> Option<Token> {
        self.tokens.next_if(|tk| tk.r#type == ty)
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
