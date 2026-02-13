use std::ops::Range;

use logos::{Lexer, Logos};

use crate::parser::{error::LexError, token::Token};

pub type LexerOutput<'a> = Option<SpannedToken<'a>>;
pub type LexerResult<'a> = Result<Option<SpannedToken<'a>>, (LexError, Span)>;
pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        let span = self.inner.span();

        match res {
            Some(res) => match res {
                Ok(token) => Ok(Some(SpannedToken {
                    token,
                    lexeme: &self.source[span.clone()],
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

        Ok(self.lookahead.clone())
    }

    /// Returns the current position in the source.
    pub fn span(&self) -> Span {
        self.inner.span()
    }
}
