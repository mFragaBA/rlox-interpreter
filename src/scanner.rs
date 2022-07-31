use std::str::CharIndices;
use std::iter::Peekable;

use crate::{TokenType, LoxResult, errors, keywords};

use super::Token;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: Peekable<CharIndices<'a>>,
    line: usize,
    failed: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        let mut chars = source.char_indices().peekable();
        Scanner {
            source,
            tokens: Vec::new(),
            start: chars.peek().map(|(index, _c)| *index).unwrap_or(0),
            current: chars,
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
        self.current.peek().is_none()
    }

    fn scan_token(&mut self) {
        let (i, c) = self.current.next().expect("called scan_token with no remaining chars");
        match c {
            '(' => self.add_token(i, TokenType::LEFT_PAREN, None),
            ')' => self.add_token(i, TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(i, TokenType::LEFT_BRACE, None),
            '}' => self.add_token(i, TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(i, TokenType::COMMA, None),
            '.' => self.add_token(i, TokenType::DOT, None),
            '-' => self.add_token(i, TokenType::MINUS, None),
            '+' => self.add_token(i, TokenType::PLUS, None),
            ';' => self.add_token(i, TokenType::SEMICOLON, None),
            '*' => self.add_token(i, TokenType::STAR, None),

            // Operators
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(i+1, TokenType::BANG_EQUAL, None);
                } else {
                    self.add_token(i, TokenType::BANG, None);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(i+1, TokenType::EQUAL_EQUAL, None);
                } else {
                    self.add_token(i, TokenType::EQUAL, None);
                }

            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(i+1, TokenType::LESS_EQUAL, None);
                } else {
                    self.add_token(i, TokenType::LESS, None);
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(i+1, TokenType::GREATER_EQUAL, None);
                } else {
                    self.add_token(i, TokenType::GREATER, None);
                };

            }

            // slash is a special case because of comments
            '/' => {
                if self.match_next_char('/') {
                    // A comment goes until the end of the line.
                    self.start += 1;
                    loop {
                        self.start += 1;
                        match self.current.peek() {
                            Some((_, '\n')) | None => break,
                            Some(_) => { self.current.next().expect("Could not advance through source code"); }
                        }
                    }
                } else {
                    self.add_token(i, TokenType::SLASH, None);
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
                    match self.current.peek() {
                        Some((i, '"')) => {
                            let current_pos = *i;
                            self.current.next().expect("Could not advance through source code"); 

                            let value = &self.source[self.start+1..=current_pos-1];
                            self.add_token(current_pos, TokenType::STRING, Some(value));
                            break;
                        }
                        Some((_, c)) => { 
                            if *c == '\n' { self.line += 1; }
                            self.current.next().expect("Could not advance through source code"); 
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
                   match self.current.peek() {
                        Some((_, d)) if d.is_ascii_digit() => { self.current.next().expect("Could not advance through source code"); }
                        Some((_, '.')) => {
                            let after_point = self.current.nth(1);
                            if let Some((_, d)) = after_point {
                                if d.is_ascii_digit() {
                                    self.current.next().expect("Could not advance through source code"); 
                                }
                            }
                        }
                        Some((i, _)) => {
                            let pos = (*i) - 1;
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
                    match self.current.peek() {
                        Some((_, c)) if c.is_alphanumeric() => { self.current.next().expect("Could not advance through source code"); }
                        Some((i, _)) => {
                            let pos = (*i) - 1;
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

    fn match_next_char(&mut self, expected: char) -> bool {
        match self.current.peek() {
            Some((_, c)) if *c == expected => {
                self.current.next().expect("Could not advance through source code");
                true
            }
            _ => false,
        }
        
    }
}
