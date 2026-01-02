use logos::{Logos, SpannedIter};

use crate::parser::token::Token;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum LexError {
    #[default]
    TokenError,
}

pub struct Lexer<'src> {
    stream: SpannedIter<'src, Token<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(program: &'src str) -> Self {
        Self {
            stream: Token::lexer(program).spanned(),
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Spanned<Token<'src>, usize, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream
            .next()
            .map(|(tok, span)| Ok((span.start, tok?, span.end)))
    }
}
