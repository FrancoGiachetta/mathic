pub mod error;
pub mod grammar;
pub mod types;

use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

use grammar::Function;
use lazy_static::lazy_static;

use crate::lexer::token::{Token, TokenType};

use {
    error::ParseError,
    grammar::{FuncDecl, Program},
};

macro_rules! rules {
    ($($tk_ty:path => {$prefix:expr, $infix:expr, $prec:path};)+ ) => {
        std::collections::HashMap::from([
            $((
                $tk_ty,
                crate::parser::ParsingRule {
                    prefix: $prefix,
                    infix: $infix,
                    precedence: $prec
                }
            ),)*
        ])
    };
}

lazy_static! {
    static ref parsing_rules: HashMap<TokenType, ParsingRule> = rules! {
        TokenType::LeftParen => { None, None, Precedence::None };
        TokenType::RightParen => { None, None, Precedence::None };
        TokenType::LeftBrace => { None, None, Precedence::None };
        TokenType::RightBrace => { None, None, Precedence::None };
        TokenType::Comma => { None, None, Precedence::None };
        TokenType::Minus => { None, None, Precedence::None };
        TokenType::Plus => { None, None, Precedence::None };
        TokenType::Star => { None, None, Precedence::None };
        TokenType::Slash => { None, None, Precedence::None };
        TokenType::SemiColon => { None, None, Precedence::None };
        TokenType::Equal => { None, None, Precedence::None };
        TokenType::Greater => { None, None, Precedence::None };
        TokenType::GreaterEqual => { None, None, Precedence::None };
        TokenType::Less => { None, None, Precedence::None };
        TokenType::LessEqual => { None, None, Precedence::None };
        TokenType::Identifier => { None, None, Precedence::None };
        TokenType::String => { None, None, Precedence::None };
        TokenType::Number => { None, None, Precedence::None };
        TokenType::True => { None, None, Precedence::None };
        TokenType::False => { None, None, Precedence::None };
        TokenType::If => { None, None, Precedence::None };
        TokenType::Else => { None, None, Precedence::None };
        TokenType::While => { None, None, Precedence::None };
        TokenType::For => { None, None, Precedence::None };
        TokenType::Def => { None, None, Precedence::None };
        TokenType::Return => { None, None, Precedence::None };
        TokenType::Eof => { None, None, Precedence::None };
    };
}

type Result<T> = std::result::Result<T, ParseError>;

enum Precedence {
    Assign, // +
    Or,     // or
    And,    // and
    Eq,     // == !=
    Comp,   // < > <= >=
    Term,   // + -
    Factor, // * /
    Unary,  // ! -
    Call,   // . ()
    Primary,
    None,
}

struct ParsingRule {
    prefix: Option<Box<dyn Fn() + Sync>>,
    infix: Option<Box<dyn Fn() + Sync>>,
    precedence: Precedence,
}

pub struct MathicParser {
    tokens: Peekable<IntoIter<Token>>,
    previous: Option<Token>,
    panic_mode: bool,
}

impl MathicParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens.into_iter().peekable();

        Self {
            tokens,
            previous: None,
            panic_mode: false,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut funcs = Vec::new();

        loop {
            let Some(tk) = self.next() else {
                return Err(ParseError::UnexpectedEnd);
            };

            match tk.r#type {
                TokenType::Def => funcs.push(self.parse_precedence(Precedence::Assign)),
                TokenType::Eof => return Ok(Program { funcs }),
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        tk.line,
                        tk.end,
                        "function declaration".into(),
                    ));
                }
            }
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        let rule = self.get_rule().unwrap();

    }

    fn func_declaration(&mut self) -> Function {
        let rule = self.get_rule();
        todo!()
    }

    fn get_rule(&mut self) -> Result<&ParsingRule> {
        let Some(tk) = self.peek() else {
            return Err(ParseError::UnexpectedEnd);
        };
        
        // this Err(_) is unreachable since it would mean the token type
        // has not been taken as parsing rule
        parsing_rules.get(&tk.r#type).ok_or_else(|| unreachable!())
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
