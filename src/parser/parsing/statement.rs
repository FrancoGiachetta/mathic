use crate::parser::{
    MathicParser, ParserResult, Span,
    ast::{
        declaration::DeclStmt,
        expression::ExprStmt,
        statement::{BlockStmt, Stmt, StmtKind},
    },
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_stmt(&self) -> ParserResult<Stmt> {
        let lookahead = self.peek_not_none()?;

        let start_span = lookahead.span;

        let (kind, span) = match lookahead.token {
            Token::Df => {
                let func = self.parse_func()?;

                let span = Span::from_merged_spans(start_span, self.current_span());
                (StmtKind::Decl(DeclStmt::Func(func)), span)
            }
            Token::If => {
                let if_stmt = self.parse_if_stmt()?;
                let span = Span::from_merged_spans(start_span, self.current_span());
                (StmtKind::If(if_stmt), span)
            }
            Token::While => {
                let while_stmt = self.parse_while_stmt()?;
                let span = Span::from_merged_spans(start_span, self.current_span());
                (StmtKind::While(while_stmt), span)
            }
            Token::For => {
                let for_stmt = self.parse_for_stmt()?;
                let span = Span::from_merged_spans(start_span, self.current_span());
                (StmtKind::For(for_stmt), span)
            }
            Token::Let => {
                let var = self.parse_var_decl()?;
                let span = Span::from_merged_spans(start_span, var.expr.span);
                (StmtKind::Decl(DeclStmt::Var(var)), span)
            }
            Token::Struct => {
                let struct_decl = self.parse_struct()?;
                let span = Span::from_merged_spans(start_span, self.current_span());
                (StmtKind::Decl(DeclStmt::Struct(struct_decl)), span)
            }

            Token::Sym => {
                todo!()
            }
            Token::Return => {
                let expr = self.parse_return()?;
                let span = Span::from_merged_spans(start_span, expr.span);
                (StmtKind::Return(expr), span)
            }
            Token::LBrace => {
                let block = self.parse_block()?;
                let span = if let Some(last) = block.stmts.last() {
                    Span::from_merged_spans(start_span, last.span)
                } else {
                    start_span
                };
                (StmtKind::Block(block), span)
            }
            _ => {
                let expr_stmt = self.parse_expr_stmt()?;
                (expr_stmt.kind, expr_stmt.span)
            }
        };

        Ok(Stmt { kind, span })
    }

    pub fn parse_block(&self) -> ParserResult<BlockStmt> {
        let start_span = self.consume_token(Token::LBrace)?;

        let mut stmts = Vec::new();

        while !self.check_next(Token::RBrace)? {
            stmts.push(self.parse_stmt()?);
        }

        let end_span = self.consume_token(Token::RBrace)?;

        Ok(BlockStmt {
            stmts,
            span: Span::from_merged_spans(start_span.span, end_span.span),
        })
    }

    pub fn parse_return(&self) -> ParserResult<ExprStmt> {
        let return_token = self.next()?;

        let value = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        let span = Span::from_merged_spans(return_token.span, self.current_span());

        Ok(ExprStmt {
            kind: value.kind,
            span,
        })
    }

    fn parse_expr_stmt(&self) -> ParserResult<Stmt> {
        let expr = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        let span = Span::from_merged_spans(expr.span, self.current_span());

        Ok(Stmt {
            kind: StmtKind::Expr(expr),
            span,
        })
    }
}
