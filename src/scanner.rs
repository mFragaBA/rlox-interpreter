use std::str::Chars;
use std::iter::Peekable;

use crate::{TokenType, LoxResult, errors, keywords};

use super::Token;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    at: usize,
    current: Chars<'a>,
    line: usize,
    failed: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            at: 0,
            current: source.chars(),
            line: 1,
            failed: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&'a[Token], Box<dyn std::error::Error>> {
        while !self.is_at_end() {
            self.scan_token()
        }
        
        if self.failed {
            return Err("Scanning Failed".into())
        } 
        
        Ok(&self.tokens)
    }

    fn is_at_end(&mut self) -> bool { 
        self.peek_first_char().is_none()
    }


    fn peek_and_add_token(&mut self, expecting: char, matched: TokenType, not_matched: TokenType) {
        if self.match_next_char(expecting) {
            self.add_token(self.at-1, matched, None);
        } else {
            self.add_token(self.at-1, not_matched, None);
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance(); 
        match c {
            '(' => self.add_token(self.at-1, TokenType::LEFT_PAREN, None),
            ')' => self.add_token(self.at-1, TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(self.at-1, TokenType::LEFT_BRACE, None),
            '}' => self.add_token(self.at-1, TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(self.at-1, TokenType::COMMA, None),
            '.' => self.add_token(self.at-1, TokenType::DOT, None),
            '-' => self.add_token(self.at-1, TokenType::MINUS, None),
            '+' => self.add_token(self.at-1, TokenType::PLUS, None),
            ';' => self.add_token(self.at-1, TokenType::SEMICOLON, None),
            '*' => self.add_token(self.at-1, TokenType::STAR, None),

            // Operators
            '!' => self.peek_and_add_token('=', TokenType::BANG_EQUAL, TokenType::BANG),
            '=' => self.peek_and_add_token('=', TokenType::EQUAL_EQUAL, TokenType::EQUAL),
            '<' => self.peek_and_add_token('=', TokenType::LESS_EQUAL, TokenType::LESS),
            '>' => self.peek_and_add_token('=', TokenType::GREATER_EQUAL, TokenType::GREATER), 

            // slash is a special case because of comments
            '/' => {
                if self.match_next_char('/') {
                    // A comment goes until the end of the line.
                    self.start += 2;
                    loop {
                        self.start += 1;
                        self.at += 1;
                        match self.next_char() {
                            Some('\n') => {
                                self.line += 1;
                                break;
                            }
                            Some(_) => { }
                            None => break,
                        }
                    }
                } else if self.match_next_char('*') {
                    // A multiline comment goes until it finds a closing `*/`.
                    self.start += 2;
                    let mut depth = 1;
                    while depth > 0 {
                        self.start += 1;
                        self.at += 1;
                        match self.next_char() {
                            Some('\n') => {
                                self.line += 1;
                            }
                            Some('*') => { 
                                if self.match_next_char('/') {
                                    self.start += 1;
                                    depth -= 1;
                                }
                            }
                            Some('/') => {
                                if self.match_next_char('*') {
                                    self.start += 1;
                                    depth += 1;
                                }
                            }
                            Some(_) => { }
                            None => errors::error(self.line, String::from("Unterminated multiline comment")),
                        }
                    }
                } else {
                    self.add_token(self.at-1, TokenType::SLASH, None);
                }
            }
            ' ' | '\r' | '\t' => self.start += 1,
            '\n' => {
                self.line += 1;
                self.start += 1;
            }

            // Strings
            '"' => {
                loop {
                    match self.peek_first_char() {
                        Some('"') => {
                            let current_pos = self.at;
                            self.advance();

                            let value = &self.source[self.start+1..=current_pos-1];
                            self.add_token(current_pos, TokenType::STRING, Some(value));
                            break;
                        }
                        Some(c) => { 
                            if c == '\n' { self.line += 1; }
                            self.advance();
                        }
                        None => errors::error(self.line, String::from("Unterminated String")),
                    }
                }
            }

            // Number Literals
            // Number literals will always start with a digit
            // -123 is not a literal but an expression
            d if d.is_ascii_digit() => {
                loop {
                   match self.peek_first_char() {
                        Some(d) if d.is_ascii_digit() => { self.advance(); }
                        Some('.') => {
                            let after_point = self.peek_second_char();
                            if let Some(d) = after_point {
                                if d.is_ascii_digit() {
                                    self.advance();
                                }
                            }
                        }
                        Some(_) => {
                            let pos = self.at - 1;
                            let num_str = &self.source[self.start..=pos];
                            self.add_token(pos ,TokenType::NUMBER, Some(num_str));
                            break;
                        }
                        _ => break,
                    } 
                }
            }

            c if c.is_alphanumeric() => {
                loop {
                    match self.peek_first_char() {
                        Some(c) if c.is_alphanumeric() => { self.advance(); }
                        Some(_) => {
                            let pos = self.at - 1;
                            let text = &self.source[self.start..=pos];
                            let kind = match keywords::KEYWORDS.get(text) {
                                Some(kind) => { *kind },
                                None => TokenType::IDENTIFIER,
                            };
                            self.add_token(pos, kind, None);
                            break
                        }
                        _ => break,
                    }
                }
            }

            _ => {
                errors::error(self.line, "Unexpected character.".to_string());
                self.failed = true;
                self.start += 1;
            }
        }
    }

    fn add_token(&mut self, at: usize, kind: TokenType, literal: Option<&'a str>) {
        let text = &self.source[self.start..=at];
        let token = Token::new(kind, text, literal, self.line);

        self.start = at+1;
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        self.at += 1;
        self.next_char().expect("called scan_token with no remaining chars")
    }

    fn next_char(&mut self) -> Option<char> {
        self.current.next()
    }

    fn peek_first_char(&self) -> Option<char> {
        self.current.clone().next()
    }

    fn peek_second_char(&self) -> Option<char> {
        let mut iter_clone = self.current.clone();
        iter_clone.next();
        iter_clone.next()
    }

    fn match_next_char(&mut self, expected: char) -> bool {
        match self.peek_first_char() {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
        
    }
}
