use crate::parser::{
    MathicParser, ParserResult,
    error::ParseError,
    grammar::expression::{ExprStmt, PrimaryExpr},
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_expr(&self) -> ParserResult<ExprStmt> {
        // For now, we skip assignment and go directly to logical_or
        self.parse_logic_or()
    }

    fn parse_logic_or(&self) -> ParserResult<ExprStmt> {
        let mut left = self.parse_logic_and()?;

        while let Some(span_or) = self.match_token(Token::Or)? {
            let right = self.parse_logic_and()?;

            left = ExprStmt::Logical {
                lhs: Box::new(left),
                op: span_or.token,
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logic_and(&self) -> ParserResult<ExprStmt> {
        let mut left = self.parse_equality()?;

        while let Some(span_and) = self.match_token(Token::And)? {
            let right = self.parse_logic_and()?;

            left = ExprStmt::Logical {
                lhs: Box::new(left),
                op: span_and.token,
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&self) -> ParserResult<ExprStmt> {
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
