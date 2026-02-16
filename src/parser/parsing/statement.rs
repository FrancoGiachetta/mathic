use crate::parser::{
    MathicParser, ParserResult,
    ast::{
        declaration::DeclStmt,
        expression::ExprStmt,
        statement::{BlockStmt, Stmt},
    },
    error::{ParseError, SyntaxError},
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_stmt(&self) -> ParserResult<Stmt> {
        let Ok(Some(lookahead)) = self.peek() else {
            return Err(ParseError::Syntax(SyntaxError::UnexpectedEnd {
                expected: "statement".to_string(),
                span: self.current_span(),
            }));
        };

        Ok(match lookahead.token {
            Token::Df => Stmt::Decl(DeclStmt::Func(self.parse_func()?)),
            Token::If => Stmt::If(self.parse_if_stmt()?),
            Token::While => Stmt::While(self.parse_while_stmt()?),
            Token::For => Stmt::For(self.parse_for_stmt()?),
            Token::Let => Stmt::Decl(DeclStmt::Var(self.parse_var_decl()?)),
            Token::Struct | Token::Sym => {
                todo!()
            }
            Token::Return => Stmt::Return(self.parse_return()?),
            Token::LBrace => Stmt::Block(self.parse_block()?),
            Token::Ident => self.parse_assignment_stmt()?,
            _ => {
                return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                    found: lookahead.into(),
                    expected: "statement".to_string(),
                }));
            }
        })
    }

    pub fn parse_block(&self) -> ParserResult<BlockStmt> {
        self.consume_token(Token::LBrace)?;

        let mut stmts = Vec::new();

        while !self.check_next(Token::RBrace)? {
            stmts.push(self.parse_stmt()?);
        }

        self.consume_token(Token::RBrace)?;

        Ok(BlockStmt { stmts })
    }

    pub fn parse_return(&self) -> ParserResult<ExprStmt> {
        self.next()?; // Consume Return;

        let value = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        Ok(value)
    }

    fn parse_assignment_stmt(&self) -> ParserResult<Stmt> {
        let ident = self.consume_token(Token::Ident)?;
        let name = ident.lexeme.to_string();

        self.consume_token(Token::Eq)?;

        let value = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        Ok(Stmt::Assign { name, value })
    }
}
