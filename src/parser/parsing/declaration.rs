use crate::parser::{
    MathicParser, ParserResult,
    grammar::declaration::{FuncDecl, Param},
    token::Token,
};

impl<'a> MathicParser<'a> {
    pub fn parse_func(&self) -> ParserResult<FuncDecl> {
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

        let body = self.parse_block()?;

        Ok(FuncDecl { name, params, body })
    }

    fn parse_params(&self) -> ParserResult<Vec<Param>> {
        let identifier = self.consume_token(Token::Ident)?;
        let mut params = vec![Param {
            name: identifier.lexeme.to_string(),
        }];

        while self.match_token(Token::Comma)? {
            let identifier = self.consume_token(Token::Ident)?;

            // Param's type parsing should be here.

            params.push(Param {
                name: identifier.lexeme.to_string(),
            });
        }

        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        MathicParser,
        grammar::{Program, declaration::FuncDecl, statement::BlockStmt},
    };

    fn check_ast(source: &str, expected_ast: Program) {
        let parser = MathicParser::new(source);
        let ast = parser.parse().unwrap();

        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn empty_func() {
        let source = "
            df main() {}
        ";

        check_ast(
            source,
            Program {
                structs: vec![],
                funcs: vec![FuncDecl {
                    name: "main".to_string(),
                    params: vec![],
                    body: BlockStmt { stmts: vec![] },
                }],
            },
        );
    }
}
