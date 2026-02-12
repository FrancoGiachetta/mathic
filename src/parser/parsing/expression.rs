use crate::parser::{
    ast::expression::{ExprStmt, PrimaryExpr},
    error::ParseError,
    token::Token,
    MathicParser, ParserResult,
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
            let right = self.parse_equality()?;

            left = ExprStmt::Logical {
                lhs: Box::new(left),
                op: span_and.token,
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_inequality()?;

        while let Some(op) = self.match_any_token(&[Token::EqEq, Token::BangEq])? {
            let rhs = self.parse_inequality()?;

            expr = ExprStmt::BinOp {
                lhs: Box::new(expr),
                op: op.token,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_inequality(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_term()?;

        while let Some(op) =
            self.match_any_token(&[Token::Greater, Token::EqLess, Token::Less, Token::EqGrater])?
        {
            let rhs = self.parse_term()?;
            expr = ExprStmt::BinOp {
                lhs: Box::new(expr),
                op: op.token,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_term(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_any_token(&[Token::Plus, Token::Minus])? {
            let rhs = self.parse_factor()?;

            expr = ExprStmt::BinOp {
                lhs: Box::new(expr),
                op: op.token,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_any_token(&[Token::Star, Token::Slash])? {
            let rhs = self.parse_unary()?;

            expr = ExprStmt::BinOp {
                lhs: Box::new(expr),
                op: op.token,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&self) -> ParserResult<ExprStmt> {
        if let Some(op) = self.match_any_token(&[Token::Bang, Token::Minus])? {
            let rhs = self.parse_unary()?;

            return Ok(ExprStmt::Unary {
                op: op.token,
                rhs: Box::new(rhs),
            });
        }

        self.parse_call()
    }

    fn parse_call(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_primary_expr()?;

        while self.match_token(Token::LParen)?.is_some() {
            let args = if self.check_next(Token::RParen)? {
                Vec::new()
            } else {
                let mut args = Vec::new();
                args.push(self.parse_expr()?);

                while self.match_token(Token::Comma)?.is_some() {
                    args.push(self.parse_expr()?);
                }

                args
            };

            self.consume_token(Token::RParen)?;

            if let ExprStmt::Primary(PrimaryExpr::Ident(calle)) = expr {
                expr = ExprStmt::Call { calle, args };
            } else {
                return Err(ParseError::UnexpectedToken(
                    "Expected identifier for function call".into(),
                ));
            }
        }

        Ok(expr)
    }

    fn parse_primary_expr(&self) -> ParserResult<ExprStmt> {
        let Some(lookahead) = self.next()? else {
            return Err(ParseError::UnexpectedEnd);
        };

        let expr = match lookahead.token {
            Token::Num => ExprStmt::Primary(PrimaryExpr::Num(lookahead.lexeme.to_string())),
            Token::True => ExprStmt::Primary(PrimaryExpr::Bool(true)),
            Token::False => ExprStmt::Primary(PrimaryExpr::Bool(false)),
            Token::Ident => ExprStmt::Primary(PrimaryExpr::Ident(lookahead.lexeme.to_string())),
            Token::LParen => {
                let expr = self.parse_expr()?;

                self.consume_token(Token::RParen)?;
                ExprStmt::Group(Box::new(expr))
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
