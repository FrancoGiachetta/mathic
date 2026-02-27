use std::cell::RefCell;

use ast::Program;
use lexer::{MathicLexer, Span, SpannedToken};
use token::Token;

use crate::diagnostics::parse::{ExpectedToken, FoundToken, ParseError, SyntaxError};
use crate::parser::lexer::LexerOutput;

pub mod ast;
pub mod lexer;
pub mod parsing;
pub mod token;

pub type ParserResult<T> = Result<T, ParseError>;

pub struct MathicParser<'a> {
    lexer: RefCell<MathicLexer<'a>>,
    current_span: RefCell<Span>,
    _panic_mode: bool,
}

impl<'a> MathicParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: RefCell::new(MathicLexer::new(source)),
            current_span: RefCell::new(0..0),
            _panic_mode: false,
        }
    }

    pub fn parse(&self) -> ParserResult<Program> {
        let mut funcs = Vec::new();
        let mut _structs = Vec::new();

        while let Ok(Some(SpannedToken {
            token,
            lexeme,
            span,
        })) = self.peek()
        {
            match token {
                Token::Df => funcs.push(self.parse_func()?),
                Token::Struct => todo!("parse struct"),
                _ => {
                    return Err(ParseError::Syntax(SyntaxError::UnexpectedToken {
                        found: FoundToken {
                            lexeme: lexeme.to_string(),
                            span,
                        },
                        expected: ExpectedToken::Custom(
                            "function or struct definition".to_string(),
                        ),
                    }));
                }
            }
        }

        Ok(Program {
            funcs,
            structs: _structs,
        })
    }

    /// Returns the next token, advancing the lexer.
    fn next(&self) -> ParserResult<LexerOutput<'a>> {
        self.lexer
            .borrow_mut()
            .next()
            .map_err(|(e, span)| ParseError::Lexical(e, span))?
            .inspect(|t| {
                self.current_span.replace(t.span.clone());
            })
            .ok_or(ParseError::Syntax(SyntaxError::UnexpectedEnd {
                span: self.current_span(),
            }))
    }

    /// Returns the next token without advancing the lexer.
    fn peek(&self) -> ParserResult<Option<LexerOutput<'a>>> {
        self.lexer
            .borrow_mut()
            .peek()
            .map_err(|(e, span)| ParseError::Lexical(e, span))
    }

    /// Returns the current position in the source.
    ///
    /// This is convenient when returning errors which depend on the code.
    fn current_span(&self) -> Span {
        let span = self.current_span.borrow();

        span.start..span.end
    }

    /// Merges two spans into one that covers both.
    fn merge_spans(&self, start: &Span, end: &Span) -> Span {
        start.start.min(end.start)..start.end.max(end.end)
    }

    /// Consumes the next token.
    ///
    /// Returns a parser error if the token does not match the expected one.
    fn consume_token(&self, expected: Token) -> ParserResult<LexerOutput<'a>> {
        if let Ok(res) = self.next() {
            if res.token == expected {
                Ok(res)
            } else {
                Err(ParseError::Syntax(SyntaxError::MissingToken {
                    expected,
                    span: res.span,
                }))
            }
        } else {
            Err(ParseError::Syntax(SyntaxError::UnexpectedEnd {
                span: self.current_span(),
            }))
        }
    }

    /// Tries to match the expected token.
    ///
    /// Consumes the token and returning it if there's a match.
    fn match_token(&self, expected: Token) -> ParserResult<Option<LexerOutput<'a>>> {
        if self.check_next(expected)? {
            return Ok(Some(self.next()?));
        }

        Ok(None)
    }

    /// Tries to match any of the expected tokens.
    ///
    /// Consumes and returns the first matched token.
    fn match_any_token(&self, expected: &[Token]) -> ParserResult<Option<LexerOutput<'a>>> {
        for t in expected.iter() {
            if self.check_next(t.to_owned())? {
                return Ok(Some(self.next()?));
            }
        }

        Ok(None)
    }

    /// Checks if the next token matches the expected token.
    fn check_next(&self, expected: Token) -> ParserResult<bool> {
        Ok(if let Ok(Some(res)) = self.peek() {
            res.token == expected
        } else {
            false
        })
    }
}
