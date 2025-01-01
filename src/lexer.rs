use std::{collections::HashMap, iter::Peekable, str::Chars};

use lazy_static::lazy_static;
use thiserror::Error;

use crate::token::{Token, TokenType};

type LexResult<T> = Result<T, LexError>;

lazy_static! {
    static ref keywords: HashMap<&'static str, TokenType> = {
        let words = vec![
            ("struct", TokenType::Struct),
            ("let", TokenType::Let),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("for", TokenType::For),
            ("while", TokenType::While),
            ("sym", TokenType::Sym),
            ("def", TokenType::Def),
            ("return", TokenType::Return),
            ("true", TokenType::True),
            ("false", TokenType::False),
        ];

        HashMap::from_iter(words)
    };
}

// (error description, line, column)
#[derive(Error, Debug)]
pub enum LexError {
    #[error("[line: {1}, column: {2}] '{0}' is not a valid lex")]
    UnexpectedLex(Box<str>, u32, u16),
    #[error("[line: {1}, column: {2}] {0}")]
    InvalidNumber(Box<str>, u32, u16),
    #[error("[line: {1}, column: {2}] {0}")]
    InvalidString(Box<str>, u32, u16),
    #[error("[line: {1}, column: {2}] '{0}' is not a valid character for an identifier")]
    InvalidIdentifier(Box<str>, u32, u16),
    #[error("[line: {0}] Unterminated muti-line comment, it should end with \"///\"")]
    InvalidComment(u32),
    #[error("[line: {0}] Index out of bounds")]
    UnableToSubString(u32),
}

pub struct Lexer<'lex> {
    input: Peekable<Chars<'lex>>,
    raw_input: &'lex str,
    tokens: Vec<Token>,
    line: u32,
    start: u16,
    curr: u16,
}

impl<'lex> Lexer<'lex> {
    pub fn new(input: &'lex str) -> Self {
        let input_iter = input.chars().peekable();

        Self {
            input: input_iter,
            raw_input: input,
            tokens: Vec::new(),
            line: 1,
            start: 0,
            curr: 0,
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn lex(&mut self) -> LexResult<()> {
        while let Some(c) = self.next_char() {
            // next_char increases curr in 1 so we need to
            // decrease it before assigning it to start
            self.start = self.curr - 1;

            match c {
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '=' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                '!' => {
                    if self.match_char('=') {
                        self.add_token(TokenType::NotEqual);
                    } else {
                        self.add_token(TokenType::Not);
                    }
                }
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                '*' => self.add_token(TokenType::Star),
                '/' => {
                    if self.match_char('/') {
                        self.scan_comment()?;
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                ';' => self.add_token(TokenType::SemiColon),
                c if c.is_ascii_alphabetic() => self.scan_identifier()?,
                c if c.is_ascii_digit() => self.scan_number()?,
                '"' => self.scan_string()?,
                c if c.is_whitespace() => {
                    dbg!("entered {}", self.curr);
                    if c == '\n' {
                        self.line += 1;
                    }
                }
                _ => self.scan_and_error(LexError::UnexpectedLex(
                    Box::from(self.substring(self.start, self.curr)?),
                    self.line,
                    self.curr,
                ))?,
            }
        }

        self.add_token(TokenType::Eof);

        Ok(())
    }

    fn scan_comment(&mut self) -> LexResult<()> {
        if self.match_char('/') {
            loop {
                match self.next_char() {
                    Some('\n') => self.line += 1,
                    Some('/') => {
                        if !self.match_chars('/', '/') {
                            break;
                        }
                    }
                    Some(_) => {}
                    None => {
                        return Err(LexError::InvalidComment(self.line));
                    }
                }
            }
        } else {
            while self.next_char().is_some_and(|c| c != '\t') {}
            self.line += 1;
        }

        Ok(())
    }

    fn scan_string(&mut self) -> LexResult<()> {
        loop {
            match self.next_char() {
                Some('"') => break,
                Some(_) => {}
                None => {
                    return Err(LexError::InvalidString(
                        Box::from(format!(
                            "unterminated string '{}'",
                            self.substring(self.start, self.curr)?
                        )),
                        self.line,
                        self.curr,
                    ));
                }
            }
        }
        let string = self.substring(self.start + 1, self.curr - 1)?;
        let token = Token {
            r#type: TokenType::String(string),
            line: self.line,
            start: self.start,
            end: self.curr,
        };

        //self.check_error(&token)?;

        self.tokens.push(token);

        Ok(())
    }

    fn scan_number(&mut self) -> LexResult<()> {
        loop {
            match self.peek() {
                Some(c) if c.is_ascii_digit() => {
                    self.next_char();
                }
                _ => break,
            }
        }

        let number = self.substring(self.start, self.curr)?;
        let token = Token {
            r#type: TokenType::Number(number),
            line: self.line,
            start: self.start,
            end: self.curr,
        };

        self.check_error(&token)?;

        self.tokens.push(token);

        Ok(())
    }

    fn scan_identifier(&mut self) -> LexResult<()> {
        loop {
            let Some(c) = self.peek() else {
                return Err(LexError::InvalidIdentifier(
                    Box::from("Unterminated identifier"),
                    self.line,
                    self.curr,
                ));
            };

            match c {
                c if c.is_ascii_alphanumeric() => {
                    self.next_char();
                }
                _ => break,
            };
            dbg!(self.curr);
        }

        let ident = self.substring(self.start, self.curr)?;
        let token = Token {
            r#type: TokenType::Identifier(ident.clone()),
            line: self.line,
            start: self.start,
            end: self.curr,
        };

        self.check_error(&token)?;

        if let Some(kw) = keywords.get(ident.as_str()) {
            self.add_token(kw.clone());
            return Ok(());
        }

        self.tokens.push(token);

        Ok(())
    }

    fn add_token(&mut self, r#type: TokenType) {
        self.tokens.push(Token {
            r#type,
            line: self.line,
            start: self.start,
            end: self.curr,
        })
    }

    fn substring(&self, start: u16, end: u16) -> LexResult<String> {
        let substring = if start == end {
            self.raw_input
                .chars()
                .nth(start as usize)
                .map(|o| o.to_string())
        } else {
            self.raw_input
                .get(start as usize..end as usize)
                .map(|s| s.to_string())
        };

        match substring {
            Some(s) => Ok(s.to_string()),
            None => Err(LexError::UnableToSubString(self.line)),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.curr += 1;
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_pair(&mut self) -> (Option<char>, Option<char>) {
        (
            self.input.peek().cloned(),
            self.input.nth(self.curr as usize + 2),
        )
    }

    fn match_char(&mut self, expected: char) -> bool {
        self.curr += 1;
        self.input.next_if_eq(&expected).is_some()
    }

    fn check_error(&mut self, token: &Token) -> LexResult<()> {
        let (fst_next_char, snd_next_char) = self.peek_pair();

        let error = match (fst_next_char, snd_next_char, &token.r#type) {
            (Some(fst_c), Some(snd_c), TokenType::Number(_))
                if !fst_c.is_whitespace() && snd_c.is_ascii_digit() =>
            {
                Some(LexError::InvalidNumber(
                    Box::from(format!(
                        "'{}' is not valid number",
                        self.substring(self.start, self.curr)?
                    )),
                    self.line,
                    self.curr,
                ))
            }
            (Some(fst_c), Some(snd_c), TokenType::Identifier(_))
                if !(fst_c.is_ascii_alphanumeric() && fst_c.is_whitespace())
                    && snd_c.is_ascii_alphanumeric() =>
            {
                Some(LexError::InvalidIdentifier(
                    Box::from(self.substring(self.start, self.curr)?),
                    self.line,
                    self.curr,
                ))
            }
            _ => None,
        };

        if let Some(err) = error {
            self.scan_and_error(err)?;
        }

        Ok(())
    }

    fn scan_and_error(&mut self, err: LexError) -> LexResult<()> {
        while self.peek().is_some_and(|c| !c.is_whitespace()) {
            self.next_char();
        }

        return Err(err);
    }

    fn match_chars(&mut self, exp_fst_char: char, exp_snd_char: char) -> bool {
        let fst_char = self.input.nth(self.curr as usize + 1);
        let snd_char = self.input.nth(self.curr as usize + 2);

        if let (Some(c1), Some(c2)) = (fst_char, snd_char) {
            if c1 == exp_fst_char && c2 == exp_snd_char {
                self.input.next();
                self.input.next();

                return true;
            }

            return false;
        }

        false
    }
}
