use crate::parser::{
    ast::{
        declaration::{FuncDecl, Param},
        statement::BlockStmt,
    },
    token::Token,
    MathicParser, ParserResult,
};

impl<'a> MathicParser<'a> {
    pub fn parse_func(&self) -> ParserResult<FuncDecl> {
        self.next()?; // Consume Df.

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

        Ok(FuncDecl {
            name,
            params,
            body: stmts,
        })
    }

    fn parse_params(&self) -> ParserResult<Vec<Param>> {
        let identifier = self.consume_token(Token::Ident)?;
        let mut params = vec![Param {
            name: identifier.lexeme.to_string(),
        }];

        while self.match_token(Token::Comma)?.is_some() {
            let identifier = self.consume_token(Token::Ident)?;

            // Param's type parsing should be here.

            params.push(Param {
                name: identifier.lexeme.to_string(),
            });
        }

        Ok(params)
    }
}
