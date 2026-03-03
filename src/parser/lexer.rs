use logos::{Lexer, Logos};

use crate::{
    diagnostics::parse::LexError,
    parser::{Span, token::Token},
};

pub type LexerOutput<'a> = SpannedToken<'a>;
pub type LexerResult<'a> = Result<Option<LexerOutput<'a>>, (LexError, Span)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpannedToken<'a> {
    pub token: Token,
    pub lexeme: &'a str,
    pub span: Span,
}

pub struct MathicLexer<'src> {
    source: &'src str,
    inner: Lexer<'src, Token>,
    lookahead: Option<SpannedToken<'src>>,
}

impl<'src> MathicLexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            inner: Token::lexer(source),
            lookahead: None,
        }
    }

    fn read_next(&mut self) -> LexerResult<'src> {
        let res = self.inner.next();
        let span = Span::from(self.inner.span());

        match res {
            Some(res) => match res {
                Ok(token) => Ok(Some(SpannedToken {
                    token,
                    lexeme: &self.source[span.into_range()],
                    span,
                })),
                Err(e) => Err((e, span)),
            },
            None => Ok(None),
        }
    }

    /// Returns the next token, advancing the lexer.
    pub fn next(&mut self) -> LexerResult<'src> {
        if let Some(lookahead) = self.lookahead.take() {
            return Ok(Some(lookahead));
        }

        self.read_next()
    }

    /// Returns the next token without advancing the lexer.
    pub fn peek(&mut self) -> LexerResult<'src> {
        if self.lookahead.is_none() {
            self.lookahead = self.read_next()?;
        }

        Ok(self.lookahead)
    }
}
