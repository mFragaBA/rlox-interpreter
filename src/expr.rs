use super::Token;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(String),
    Unary(Token<'a>, Box<Expr<'a>>),
}

impl<'a> std::fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Expr::Binary(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(ref expr) => write!(f, "(group {})", expr),
            Expr::Literal(ref value) => write!(f, "{}", value),
            Expr::Unary(ref operator, ref expr) => write!(f, "({} {})", operator.lexeme, expr),
        }
    }
}

pub trait ExprVisitor<'a, T> {
    fn visit(&mut self, expr: &Expr<'a>) -> T;
}

struct ASTPrinter;
impl<'a> ExprVisitor<'a, String> for ASTPrinter {
    fn visit(&mut self, expr: &Expr<'a>) -> String {
        match expr {
            Expr::Binary(ref left, ref operator, ref right) => {
                format!("({} {} {})", 
                    operator.lexeme, 
                    self.visit(left), 
                    self.visit(right)
                )
            }
            Expr::Grouping(ref expr) => {
                format!("(group {})", self.visit(expr))
            }
            Expr::Literal(ref value) => format!("{}", value),
            Expr::Unary(ref operator, ref expr) => {
                format!("({} {})", 
                    operator.lexeme, 
                    self.visit(expr)
                )
            }
        }
    }
}

struct RPNPrinter;
impl <'a> ExprVisitor<'a, String> for RPNPrinter {
    fn visit(&mut self, expr: &Expr<'a>) -> String {
        match expr {
            Expr::Binary(ref left, ref operator, ref right) => {
                format!("{} {} {}", self.visit(left), self.visit(right), operator.lexeme)
            }
            Expr::Grouping(ref expr) => {
                format!("{}", self.visit(expr))
            }
            Expr::Literal(ref value) => {
                format!("{}", value)
            }
            Expr::Unary(ref operator, ref expr) => {
                format!("{}({})", operator.lexeme, self.visit(expr))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_visit_simple_expression() {
        let simple_expression = Expr::Literal(String::from("Potato"));
        let mut visitor = ASTPrinter{};

        assert_eq!(visitor.visit(&simple_expression), format!("{}", &simple_expression));
    }

    #[test]
    fn can_visit_composite_expression() {
        let composite_expression = Expr::Binary(
            Box::new(Expr::Literal(String::from("Potato"))),
            Token::new(crate::TokenType::PLUS, "+", None, 0),
            Box::new(Expr::Literal(String::from("Potato")))
        );
        let mut visitor = ASTPrinter{};

        assert_eq!(visitor.visit(&composite_expression), format!("{}", &composite_expression));
    }

    #[test]
    fn can_visit_complex_expression() {
        // Try visiting the expression -variableA + (variableB * variableC)
        let complex_expression = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(crate::TokenType::PLUS, "+", None, 0),
                Box::new(Expr::Literal(String::from("variableA")))
            )),
            Token::new(crate::TokenType::PLUS, "+", None, 0),
            Box::new(Expr::Binary(
                Box::new(Expr::Literal(String::from("variableB"))), 
                Token::new(crate::TokenType::STAR, "*", None, 0),
                Box::new(Expr::Literal(String::from("variableC"))), 
            ))
        );
        let mut visitor = ASTPrinter{};

        assert_eq!(visitor.visit(&complex_expression), format!("{}", &complex_expression));
    }

    #[test]
    fn test_book_example() {
        // Try visiting the expression -variableA + (variableB * variableC)
        let expression = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(crate::TokenType::MINUS, "-", None, 0),
                Box::new(Expr::Literal(String::from("123")))
            )),
            Token::new(crate::TokenType::STAR, "*", None, 0),
            Box::new(Expr::Grouping(
                Box::new(Expr::Literal(String::from("45.67")))
            ))
        );
        let mut visitor = ASTPrinter{};

        assert_eq!(visitor.visit(&expression), "(* (- 123) (group 45.67))");
    }

    #[test]
    fn test_book_example_rpn() {
        // expression is (1 + 2) * (4 - 3)
        let expression = Box::new(Expr::Binary(
            Box::new(Expr::Grouping(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(String::from("1"))), 
                    Token::new(crate::TokenType::PLUS, "+", None, 0), 
                    Box::new(Expr::Literal(String::from("2"))), 
                ))
            )),
            Token::new(crate::TokenType::STAR, "*", None, 0), 
            Box::new(Expr::Grouping(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(String::from("4"))), 
                    Token::new(crate::TokenType::MINUS, "-", None, 0), 
                    Box::new(Expr::Literal(String::from("3"))), 
                ))
            )),
        ));

        let mut visitor = RPNPrinter{};

        assert_eq!(visitor.visit(&expression), "1 2 + 4 3 - *");
    }
}
