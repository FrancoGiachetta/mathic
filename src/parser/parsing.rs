use crate::diagnostics::parse::{ExpectedToken, ParseError, SyntaxError};
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
            group_paths: Vec::with_capacity(0),
            span,
            import_all: false,
        })
    }

    pub fn parse_import_path(&self) -> ParserResult<Path> {
        let ident = self.consume_token(Token::Ident)?;
        let start_span = ident.span;
        let mut path = Path {
            idents: vec![ident.lexeme.to_string()],
            group_paths: Vec::new(),
            span: start_span,
            import_all: false,
        };

        if self.match_token(Token::ColonColon)?.is_some() {
            match self.peek_not_none()?.token {
                Token::Ident => {
                    let mut new_path = self.parse_import_path()?;

                    new_path.idents = [path.idents, new_path.idents].concat();

                    path = new_path;
                }
                Token::Star => {
                    self.next()?;
                    path.import_all = true;
                }
                Token::LBrace => {
                    self.next()?;

                    path.group_paths.push(self.parse_import_path()?);

                    while self.match_token(Token::Comma)?.is_some() {
                        path.group_paths.push(self.parse_import_path()?);
                    }

                    self.consume_token(Token::RBrace)?;
                }
                _ => {
                    return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                        found: self.peek_not_none()?.into(),
                        expected: ExpectedToken::Custom("path segment, '*', or '{'".to_string()),
                    }));
                }
            }
        }

        path.span = Span::from_merged_spans(start_span, self.current_span());

        Ok(path)
    }
}
