use crate::diagnostics::parse::{ExpectedToken, ParseError, SyntaxError};
use crate::parser::{
    MathicParser, ParserResult,
    ast::expression::{
        ArithOp, BinaryOp, CmpOp, ExprStmt, ExprStmtKind, LogicalOp, PrimaryExpr, UnaryOp,
    },
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_expr(&self) -> ParserResult<ExprStmt> {
        self.parse_assignment()
    }

    fn parse_assignment(&self) -> ParserResult<ExprStmt> {
        let Some(lookahead) = self.peek()? else {
            return Err(ParseError::Syntax(SyntaxError::UnexpectedEnd {
                span: self.current_span(),
            }));
        };
        let lhs = self.parse_logic_or()?;

        if self.match_token(Token::Eq)?.is_some() {
            let ExprStmtKind::Primary(PrimaryExpr::Ident(name)) = lhs.kind else {
                return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                    found: lookahead.into(),
                    expected: ExpectedToken::Identifier,
                }));
            };

            let rhs = self.parse_logic_or()?;
            let span = self.merge_spans(&lhs.span, &rhs.span);

            return Ok(ExprStmt {
                kind: ExprStmtKind::Assign {
                    name,
                    expr: Box::new(rhs),
                },
                span,
            });
        }

        Ok(lhs)
    }

    fn parse_logic_or(&self) -> ParserResult<ExprStmt> {
        let mut left = self.parse_logic_and()?;

        while let Some(op) = self.match_token(Token::Or)? {
            let right = self.parse_logic_and()?;
            let span = self.merge_spans(&left.span, &right.span);

            left = ExprStmt {
                kind: ExprStmtKind::Logical {
                    lhs: Box::new(left),
                    op: match op.token {
                        Token::Or => LogicalOp::Or,
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Token(Token::Or),
                            }));
                        }
                    },
                    rhs: Box::new(right),
                },
                span,
            };
        }

        Ok(left)
    }

    fn parse_logic_and(&self) -> ParserResult<ExprStmt> {
        let mut left = self.parse_equality()?;

        while let Some(op) = self.match_token(Token::And)? {
            let right = self.parse_equality()?;
            let span = self.merge_spans(&left.span, &right.span);

            left = ExprStmt {
                kind: ExprStmtKind::Logical {
                    lhs: Box::new(left),
                    op: match op.token {
                        Token::And => LogicalOp::And,
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Token(Token::And),
                            }));
                        }
                    },
                    rhs: Box::new(right),
                },
                span,
            };
        }

        Ok(left)
    }

    fn parse_equality(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_inequality()?;

        while let Some(op) = self.match_any_token(&[Token::EqEq, Token::BangEq])? {
            let rhs = self.parse_inequality()?;
            let span = self.merge_spans(&expr.span, &rhs.span);

            expr = ExprStmt {
                kind: ExprStmtKind::Binary {
                    lhs: Box::new(expr),
                    op: match op.token {
                        Token::EqEq => BinaryOp::Compare(CmpOp::Eq),
                        Token::BangEq => BinaryOp::Compare(CmpOp::Ne),
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Custom("either == or !=".to_string()),
                            }));
                        }
                    },
                    rhs: Box::new(rhs),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_inequality(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_term()?;

        while let Some(op) =
            self.match_any_token(&[Token::Greater, Token::EqLess, Token::Less, Token::EqGreater])?
        {
            let rhs = self.parse_term()?;
            let span = self.merge_spans(&expr.span, &rhs.span);

            expr = ExprStmt {
                kind: ExprStmtKind::Binary {
                    lhs: Box::new(expr),
                    op: match op.token {
                        Token::Less => BinaryOp::Compare(CmpOp::Lt),
                        Token::EqLess => BinaryOp::Compare(CmpOp::Le),
                        Token::Greater => BinaryOp::Compare(CmpOp::Gt),
                        Token::EqGreater => BinaryOp::Compare(CmpOp::Ge),
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Custom(
                                    "either <, >, <= or >=".to_string(),
                                ),
                            }));
                        }
                    },
                    rhs: Box::new(rhs),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_term(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_any_token(&[Token::Plus, Token::Minus])? {
            let rhs = self.parse_factor()?;
            let span = self.merge_spans(&expr.span, &rhs.span);

            expr = ExprStmt {
                kind: ExprStmtKind::Binary {
                    lhs: Box::new(expr),
                    op: match op.token {
                        Token::Plus => BinaryOp::Arithmetic(ArithOp::Add),
                        Token::Minus => BinaryOp::Arithmetic(ArithOp::Sub),
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Custom("either + or -".to_string()),
                            }));
                        }
                    },
                    rhs: Box::new(rhs),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_factor(&self) -> ParserResult<ExprStmt> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_any_token(&[Token::Star, Token::Slash])? {
            let rhs = self.parse_unary()?;
            let span = self.merge_spans(&expr.span, &rhs.span);

            expr = ExprStmt {
                kind: ExprStmtKind::Binary {
                    lhs: Box::new(expr),
                    op: match &op.token {
                        Token::Star => BinaryOp::Arithmetic(ArithOp::Mul),
                        Token::Slash => BinaryOp::Arithmetic(ArithOp::Div),
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Custom("either * or /".to_string()),
                            }));
                        }
                    },
                    rhs: Box::new(rhs),
                },
                span,
            };
        }

        Ok(expr)
    }

    fn parse_unary(&self) -> ParserResult<ExprStmt> {
        if let Some(op) = self.match_any_token(&[Token::Bang, Token::Minus])? {
            let rhs = self.parse_unary()?;
            let span = self.merge_spans(&op.span, &rhs.span);

            return Ok(ExprStmt {
                kind: ExprStmtKind::Unary {
                    op: match op.token {
                        Token::Bang => UnaryOp::Not,
                        Token::Minus => UnaryOp::Neg,
                        _ => {
                            return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                                found: op.into(),
                                expected: ExpectedToken::Custom("either ! or -".to_string()),
                            }));
                        }
                    },
                    rhs: Box::new(rhs),
                },
                span,
            });
        }

        self.parse_call()
    }

    fn parse_call(&self) -> ParserResult<ExprStmt> {
        let Some(lookahead) = self.peek()? else {
            return Err(ParseError::Syntax(SyntaxError::UnexpectedEnd {
                span: self.current_span(),
            }));
        };
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

            let span = self.merge_spans(&expr.span, &self.current_span());

            if let ExprStmtKind::Primary(PrimaryExpr::Ident(callee)) = expr.kind {
                expr = ExprStmt {
                    kind: ExprStmtKind::Call { callee, args },
                    span,
                };
            } else {
                return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                    found: lookahead.into(),
                    expected: ExpectedToken::Identifier,
                }));
            }
        }

        Ok(expr)
    }

    fn parse_primary_expr(&self) -> ParserResult<ExprStmt> {
        let lookahead = self.next()?;
        let span = lookahead.span.clone();

        let kind = match lookahead.token {
            Token::Num => ExprStmtKind::Primary(PrimaryExpr::Num(lookahead.lexeme.to_string())),
            Token::True => ExprStmtKind::Primary(PrimaryExpr::Bool(true)),
            Token::False => ExprStmtKind::Primary(PrimaryExpr::Bool(false)),
            Token::Ident => ExprStmtKind::Primary(PrimaryExpr::Ident(lookahead.lexeme.to_string())),
            Token::LParen => {
                let expr = self.parse_expr()?;
                let close_paren = self.consume_token(Token::RParen)?;
                let span = self.merge_spans(&span, &close_paren.span);

                return Ok(ExprStmt {
                    kind: ExprStmtKind::Group(Box::new(expr)),
                    span,
                });
            }
            _ => {
                return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                    found: lookahead.into(),
                    expected: ExpectedToken::Identifier,
                }));
            }
        };

        Ok(ExprStmt { kind, span })
    }
}
