use std::iter::Peekable;

use super::grammar::{DeclStmt, Program, Statement, StructDeclStmt};

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
        loop {
            let tk = self.next();
            let mut vec_decls = vec![];

            let decl = match tk.r#type {
                TokenType::Def => self.parse_function()?,
                TokenType::Struct => self.parse_struct()?,
                TokenType::Eof => {
                    return Ok(Program {
                        functions: vec_decls,
                    })
                }
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        tk.line,
                        tk.start,
                        Box::from("Expected struct, function declaration"),
                    ))
                }
            };
            vec_decls.push(decl);
        }
    }

    fn parse_struct(&mut self) -> ParseResult<Statement> {
        let tk =
            self.consume_token(TokenType::Identifier, "Expected identifier after 'struct'.")?;
        let attrs = self.parse_parameters()?;

        Ok(Statement::Decl(DeclStmt::Struct(StructDeclStmt {
            name: tk.lexeme.unwrap(),
            attrs,
        })))
    }

    fn parse_parameters(&mut self) -> ParseResult<Vec<(String, String)>> {
        let mut attrs = vec![];

        while let TokenType::Identifier = self.peek().r#type {
            let name = self.next().lexeme.unwrap();

            self.consume_token(TokenType::Colon, "expected ':' after attribute.")?;

            let ty = self
                .consume_token(TokenType::Type, "Expected type after ':'.")?
                .lexeme
                .unwrap();

            attrs.push((name, ty));
        }

        Ok(attrs)
    }

    fn consume_token(&mut self, ty: TokenType, msg: &str) -> ParseResult<Token> {
        let tk = self.peek();

        if ty == tk.r#type {
            return Ok(self.next());
        }

        Err(ParseError::UnexpectedToken(
            tk.line,
            tk.start,
            Box::from(msg),
        ))
    }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn next(&mut self) -> Token {
        self.tokens.next().unwrap()
    }
}
