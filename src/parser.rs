use crate::{Token, expr::Expr, TokenType};

struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a[Token]) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Box<Expr> {
        return self.equality();
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.match_any([
            TokenType::BANG_EQUAL, 
            TokenType::EQUAL_EQUAL
        ]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(Expr::Binary(expr, operator, right))
        }
        
        expr
}

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.term();

        while self.match_any([
            TokenType::GREATER, 
            TokenType::GREATER_EQUAL, 
            TokenType::LESS, 
            TokenType::LESS_EQUAL
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Expr::Binary(expr, operator, right))
        }

        expr
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.factor();

        while self.match_any([
            TokenType::MINUS,
            TokenType::PLUS
        ]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Expr::Binary(expr, operator, right))
        }
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.unary();

        while self.match_any([
            TokenType::SLASH,
            TokenType::STAR
        ]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Expr::Binary(expr, operator, right))
        }
    }

    fn unary(&mut self) -> Box<Expr> {
        if !self.match_any([
            TokenType::MINUS,
            TokenType::BANG
        ]) { return self.primary(); }

        let token = self.previous();
        let right = self.unary();
        Box::new(Expr::Unary(operator, right))
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_any([TokenType::FALSE]) { return Box::new(Expr::Literal(String::from("false"))); }
        if self.match_any([TokenType::TRUE]) { return Box::new(Expr::Literal(String::from("true"))); }
        if self.match_any([TokenType::NIL]) { return Box::new(Expr::Literal(String::from("null"))); }

        if self.match_any([
            TokenType::NUMBER,
            TokenType::STRING
        ]) {
            return Box::new(Expr::Literal(self.previous.literal()))
        }

        if self.match_any([TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect '(' after expression,");
            return Box::new(Expr::Grouping(expr));
        } else {
            panic!("ERRRROOOORRR!");
        }
    }

    fn match_any(types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            }

        }

        false
    }

    fn consume(expected_type: TokenType, error_msg: &str) { }

    fn check(&self, ttype: TokenType) -> bool {
        !self.is_at_end() && self.peek().ttype == ttype
    }

    fn advance(&mut self) -> Token<'a> {
        if !self.isAtEnd() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> Token<'a> {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token<'a> {
        self.tokens[self.current - 1]
    }
}
