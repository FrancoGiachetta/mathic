use std::ops::Range;

use logos::{Lexer, Logos};

use crate::parser::token::Token;

pub type LexerOutput<'a> = Option<SpannedToken<'a>>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LexError {
    #[default]
    TokenError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken<'a> {
    pub token: Token,
    pub lexeme: &'a str,
}

pub struct MathicLexer<'src> {
    source: &'src str,
    inner: Lexer<'src, Token>,
    lookahead: LexerOutput<'src>,
}

impl<'src> MathicLexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            inner: Token::lexer(source),
            lookahead: None,
        }
    }

    fn read_next(&mut self) -> Result<LexerOutput<'src>, (LexError, Range<usize>)> {
        let res = self.inner.next();
        let span = self.inner.span();

        match res {
            Some(res) => match res {
                Ok(token) => Ok(Some(SpannedToken {
                    token,
                    lexeme: &self.source[span],
                })),
                Err(e) => Err((e, span)),
            },
            None => Ok(None),
        }
    }

    pub fn next(&mut self) -> Result<LexerOutput<'src>, (LexError, Range<usize>)> {
        if self.lookahead.is_some() {
            return Ok(self.lookahead.take());
        }

        self.read_next()
    }

    pub fn peek(&mut self) -> Result<LexerOutput<'src>, (LexError, Range<usize>)> {
        if self.lookahead.is_none() {
            self.lookahead = self.read_next()?;
        }

        Ok(self.lookahead.clone())
    }
}
