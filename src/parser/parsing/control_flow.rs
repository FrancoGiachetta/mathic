use crate::parser::{
    ast::control_flow::{ForStmt, IfStmt, WhileStmt},
    token::Token,
    MathicParser, ParserResult,
};

impl<'a> MathicParser<'a> {
    pub fn parse_if_stmt(&self) -> ParserResult<IfStmt> {
        self.next()?; // consume If.

        let condition = self.parse_expr()?;

        let then_block = self.parse_block()?;

        let else_block = if self.check_next(Token::Else)? {
            self.consume_token(Token::Else)?;
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(IfStmt {
            condition,
            then_block,
            else_block,
        })
    }

    pub fn parse_while_stmt(&self) -> ParserResult<WhileStmt> {
        self.next()?; // consume While.

        let condition = self.parse_expr()?;

        let body = self.parse_block()?;

        Ok(WhileStmt { condition, body })
    }

    pub fn parse_for_stmt(&self) -> ParserResult<ForStmt> {
        self.next()?; // consume For.

        let start = self.parse_expr()?;

        self.consume_token(Token::Dot)?;
        self.consume_token(Token::Dot)?;

        let end = self.parse_expr()?;

        let body = self.parse_block()?;

        Ok(ForStmt { start, end, body })
    }
}
