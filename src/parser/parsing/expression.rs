use crate::parser::{
    MathicParser, ParserResult,
    error::ParseError,
    grammar::expression::{ExprStmt, PrimaryExpr},
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_expr(&self) -> ParserResult<ExprStmt> {
        self.parse_primary_expr()
    }

    fn parse_primary_expr(&self) -> ParserResult<ExprStmt> {
        let Some(lookahead) = self.peek()? else {
            return Err(ParseError::UnexpectedEnd);
        };

        let expr = match lookahead.token {
            Token::Num => {
                let num_token = self.consume_token(Token::Num)?;
                ExprStmt::Primary(PrimaryExpr::Num(num_token.lexeme.to_string()))
            }
            Token::True => {
                self.consume_token(Token::True)?;
                ExprStmt::Primary(PrimaryExpr::Bool(true))
            }
            Token::False => {
                self.consume_token(Token::False)?;
                ExprStmt::Primary(PrimaryExpr::Bool(false))
            }
            Token::Ident => {
                let ident = self.consume_token(Token::Ident)?;
                ExprStmt::Primary(PrimaryExpr::Ident(ident.token))
            }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    format!("Expected expression, found: {}", lookahead.lexeme).into_boxed_str(),
                ));
            }
        };

        Ok(expr)
    }
}
