use core::fmt;

use super::TokenType;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<&'a str>, 
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, lexeme: &'a str, literal: Option<&'a str>, line: usize) -> Self {
        Token { kind , lexeme, literal, line }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.kind, self.lexeme, self.literal.unwrap_or(" "))
    }
}

