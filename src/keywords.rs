use super::TokenType::{self, *};

use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", AND);
        m.insert("class", CLASS);
        m.insert("else", ELSE);
        m.insert("false", FALSE);
        m.insert("for", FOR);
        m.insert("fun", FUN);
        m.insert("if", IF);
        m.insert("nil", NIL);
        m.insert("or", OR);
        m.insert("print", PRINT);
        m.insert("return", RETURN);
        m.insert("super", SUPER);
        m.insert("this", THIS);
        m.insert("true", TRUE);
        m.insert("var", VAR);
        m.insert("while", WHILE);
        m
    };
}
