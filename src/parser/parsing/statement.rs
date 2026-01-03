use crate::parser::{
    MathicParser, ParserResult,
    grammar::statement::{BlockStmt, Stmt},
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_stmt(&self) -> ParserResult<Stmt> {
        todo!()
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
