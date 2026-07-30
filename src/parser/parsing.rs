use crate::parser::{MathicParser, ParserResult, Span, ast::declaration::IdentItem, token::Token};

mod control_flow;
mod declaration;
mod expression;
mod statement;

impl MathicParser<'_> {
    pub fn parse_ident_chain(&self) -> ParserResult<IdentItem> {
        let ident = self.consume_token(Token::Ident)?;

        if self.match_token(Token::ColonColon)?.is_some() {
            let next_ident = self.consume_token(Token::Ident)?;
            let curr_span = self.current_span();
            let mut idents = vec![ident.lexeme.to_string(), next_ident.lexeme.to_string()];
            let mut last_ident_span = ident.span;

            while self.match_token(Token::ColonColon)?.is_some() {
                let next_ident = self.consume_token(Token::Ident)?;
                idents.push(next_ident.lexeme.to_string());
                last_ident_span = next_ident.span;
            }

            let span = Span::from_merged_spans(curr_span, last_ident_span);

            Ok(IdentItem::Chain {
                ident: idents,
                span,
            })
        } else {
            Ok(IdentItem::One {
                ident: ident.lexeme.to_string(),
                span: ident.span,
            })
        }
    }
}
