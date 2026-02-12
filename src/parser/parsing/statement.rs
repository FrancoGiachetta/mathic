use crate::parser::{
    MathicParser, ParserResult,
    error::ParseError,
    grammar::{
        declaration::DeclStmt,
        statement::{BlockStmt, ReturnStmt, Stmt},
    },
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_stmt(&self) -> ParserResult<Stmt> {
        let Some(lookahead) = self.peek()? else {
            return Err(ParseError::UnexpectedEnd);
        };

        Ok(match lookahead.token {
            Token::Df => Stmt::Decl(DeclStmt::FuncDeclStmt(self.parse_func()?)),
            Token::If => Stmt::If(self.parse_if_stmt()?),
            Token::While => Stmt::While(self.parse_while_stmt()?),
            Token::For => Stmt::For(self.parse_for_stmt()?),
            Token::Struct | Token::Let | Token::Sym => {
                todo!()
            }
            Token::Return => Stmt::Return(self.parse_return()?),
            Token::LBrace => Stmt::Block(self.parse_block()?),
            _ => {
                return Err(ParseError::UnexpectedToken(
                    format!("Unexpected token: {}", lookahead.lexeme).into_boxed_str(),
                ));
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

    pub fn parse_return(&self) -> ParserResult<ReturnStmt> {
        self.next()?; // Consume Return;

        let value = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        Ok(ReturnStmt { value })
    }
}
