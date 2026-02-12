use std::cell::RefCell;

use error::ParseError;
use grammar::Program;
use lexer::{MathicLexer, SpannedToken};
use token::Token;

use crate::parser::lexer::LexerOutput;

pub mod error;
pub mod grammar;
pub mod lexer;
pub mod parsing;
pub mod token;

pub type ParserResult<T> = Result<T, ParseError>;

pub struct MathicParser<'a> {
    lexer: RefCell<MathicLexer<'a>>,
    _panic_mode: bool,
}

impl<'a> MathicParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: RefCell::new(MathicLexer::new(source)),
            _panic_mode: false,
        }
    }

    pub fn parse(&self) -> ParserResult<Program> {
        let mut funcs = Vec::new();
        let mut _structs = Vec::new();

        while let Some(SpannedToken { token, lexeme }) = self.peek()? {
            match token {
                Token::Df => funcs.push(self.parse_func()?),
                Token::Struct => todo!("parse struct"),
                _ => return Err(ParseError::UnexpectedToken(lexeme.into())),
            }
        }

        Ok(Program {
            funcs,
            structs: _structs,
        })
    }

    fn next(&self) -> ParserResult<LexerOutput<'a>> {
        self.lexer
            .borrow_mut()
            .next()
            .map_err(|(e, span)| ParseError::LexerError((e, span)))
    }

    fn peek(&self) -> ParserResult<LexerOutput<'a>> {
        self.lexer
            .borrow_mut()
            .peek()
            .map_err(|(e, span)| ParseError::LexerError((e, span)))
    }

    fn consume_token(&self, expected: Token) -> ParserResult<SpannedToken<'a>> {
        if let Some(res) = self.next()? {
            if res.token == expected {
                Ok(res)
            } else {
                Err(ParseError::UnexpectedToken(res.lexeme.into()))
            }
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    fn match_token(&self, expected: Token) -> ParserResult<LexerOutput<'a>> {
        if self.check_next(expected)? {
            return self.next();
        }

        Ok(None)
    }

    fn match_any_token(&self, expected: &[Token]) -> ParserResult<LexerOutput<'a>> {
        for t in expected.iter() {
            if self.check_next(t.to_owned())? {
                return self.next();
            }
        }

        Ok(None)
    }

    fn check_next(&self, expected: Token) -> ParserResult<bool> {
        Ok(if let Some(res) = self.peek()? {
            res.token == expected
        } else {
            false
        })
    }
}
