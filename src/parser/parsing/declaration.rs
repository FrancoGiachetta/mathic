use crate::parser::{
    MathicParser, ParserResult,
    ast::{
        declaration::{FuncDecl, Param, VarDecl},
        statement::BlockStmt,
    },
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_func(&self) -> ParserResult<FuncDecl> {
        let start_span = self.next()?.span; // Consume Df.

        let name = {
            let ident = self.consume_token(Token::Ident)?;
            ident.lexeme.to_string()
        };

        self.consume_token(Token::LParen)?;

        let params = if self.check_next(Token::RParen)? {
            Vec::new()
        } else {
            self.parse_params()?
        };

        self.consume_token(Token::RParen)?;

        // Return type parsing should be here.

        let BlockStmt { stmts } = self.parse_block()?;

        let span = self.merge_spans(&start_span, &self.current_span());

        Ok(FuncDecl {
            name,
            params,
            body: stmts,
            span,
        })
    }

    pub fn parse_var_decl(&self) -> ParserResult<VarDecl> {
        self.next()?; // Consume Let;

        let ident = self.consume_token(Token::Ident)?;
        let name = ident.lexeme.to_string();

        self.consume_token(Token::Eq)?;

        let expr = self.parse_expr()?;

        self.consume_token(Token::Semicolon)?;

        Ok(VarDecl { name, expr })
    }

    fn parse_params(&self) -> ParserResult<Vec<Param>> {
        let identifier = self.consume_token(Token::Ident)?;
        let mut params = vec![Param {
            name: identifier.lexeme.to_string(),
            span: identifier.span,
        }];

        while self.match_token(Token::Comma)?.is_some() {
            let identifier = self.consume_token(Token::Ident)?;

            // Param's type parsing should be here.

            params.push(Param {
                name: identifier.lexeme.to_string(),
                span: identifier.span,
            });
        }

        Ok(params)
    }
}
