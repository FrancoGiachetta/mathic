use crate::parser::{MathicParser, ParserResult, Span, ast::declaration::Path, token::Token};

mod control_flow;
mod declaration;
mod expression;
mod statement;

impl MathicParser<'_> {
    pub fn parse_path(&self) -> ParserResult<Path> {
        let ident = self.consume_token(Token::Ident)?;
        let curr_span = self.current_span();
        let mut idents = vec![ident.lexeme.to_string()];
        let mut last_ident_span = ident.span;

        while self.match_token(Token::ColonColon)?.is_some() {
            let next_ident = self.consume_token(Token::Ident)?;
            idents.push(next_ident.lexeme.to_string());
            last_ident_span = next_ident.span;
        }

        let span = Span::from_merged_spans(curr_span, last_ident_span);

        Ok(Path {
            idents,
            span,
            import_all: false,
        })
    }

    pub fn parse_path_with_all(&self) -> ParserResult<Path> {
        let ident = self.consume_token(Token::Ident)?;
        let curr_span = self.current_span();
        let mut idents = vec![ident.lexeme.to_string()];
        let mut last_ident_span = ident.span;
        let mut import_all = false;

        loop {
            if self.match_token(Token::ColonColon)?.is_some() {
                let next_ident = self.consume_token(Token::Ident)?;
                idents.push(next_ident.lexeme.to_string());
                last_ident_span = next_ident.span
            } else if self.match_token(Token::Star)?.is_some() {
                break import_all = true;
            }
        }

        let span = Span::from_merged_spans(curr_span, last_ident_span);

        Ok(Path {
            idents,
            span,
            import_all,
        })
    }
}
