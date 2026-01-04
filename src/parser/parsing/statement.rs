use crate::parser::{
    MathicParser, ParserResult,
    error::ParseError,
    grammar::statement::{BlockStmt, Stmt},
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_stmt(&self) -> ParserResult<Stmt> {
        let Some(lookahead) = self.peek()? else {
            return Err(ParseError::UnexpectedEnd);
        };

        Ok(match lookahead.token {
            Token::Df | Token::Struct | Token::Let | Token::Sym => todo!(),
            Token::If => todo!(),
            Token::For => todo!(),
            Token::While => todo!(),
            Token::Return => todo!(),
            Token::LBrace => Stmt::Block(self.parse_block()?),
            _ => todo!(),
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
}
