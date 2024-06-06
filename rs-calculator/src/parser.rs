use std::cmp::Ordering;

use crate::expression::Expression;
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, PartialEq)]
enum Precedence {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,

    PlusMinus = 3,
    MulDiv = 4,
    Power = 5,

    Prefix = 6,
    Paren = 7,
}

impl From<&Precedence> for i32 {
    fn from(p: &Precedence) -> i32 {
        match p {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::PlusMinus => 3,
            Precedence::MulDiv => 4,
            Precedence::Power => 5,
            Precedence::Prefix => 6,
            Precedence::Paren => 7,
        }
    }
}

impl Precedence {
    fn get_precedence(t: &Token) -> Precedence {
        match t {
            Token::Plus | Token::Minus => Precedence::PlusMinus,
            Token::Mul | Token::Div => Precedence::MulDiv,
            Token::Power => Precedence::Power,
            Token::LParen | Token::RParen => Precedence::Paren,
            _ => Precedence::Lowest,
        }
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_num = i32::from(self);
        let other_num = i32::from(other);
        std::cmp::PartialOrd::partial_cmp(&self_num, &other_num)
    }
}

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self { lexer, cur_token, peek_token }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::get_precedence(&self.peek_token)
    }

    pub fn peek_token_is(&self, t: Token) -> bool {
        self.peek_token == t
    }

    pub fn parse(&mut self) -> Expression {
        self.parse_expression(Precedence::Lowest)
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        let mut left = match self.parse_prefix_expression() {
            Some(expr) => expr,
            None => panic!("No prefix parse function for {:?}", self.cur_token),
        };

        while !self.peek_token_is(Token::Eof) && precedence < self.peek_precedence() {
            self.next_token();
            left = match self.parse_infix_expression(left.clone()) {
                Some(expr) => expr,
                None => return left,
            }
        }

        left
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        match self.cur_token.clone() {
            Token::Minus => {
                let operator: String = Token::Minus.into();
                self.next_token();
                let right = Box::new(self.parse_expression(Precedence::Prefix));
                Some(Expression::Prefix { operator, right })
            }
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Lowest);
                self.next_token();
                Some(expr)
            }
            Token::Num(literal) => Some(Expression::Number(literal.parse().unwrap())),
            _ => None,
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        match self.cur_token.clone() {
            Token::Plus | Token::Minus | Token::Mul | Token::Div | Token::Power => {
                let precedence = Precedence::get_precedence(&self.cur_token);
                let right = self.parse_expression(precedence);
                self.next_token();
                Some(Expression::Infix {
                    left: Box::new(left),
                    operator: self.cur_token.clone().into(),
                    right: Box::new(right),
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence() {
        assert!(Precedence::get_precedence(&Token::Plus) == Precedence::PlusMinus);
        assert!(Precedence::get_precedence(&Token::Minus) == Precedence::PlusMinus);
        assert!(Precedence::get_precedence(&Token::Mul) == Precedence::MulDiv);
        assert!(Precedence::get_precedence(&Token::Div) == Precedence::MulDiv);
        assert!(Precedence::get_precedence(&Token::Power) == Precedence::Power);
        assert!(Precedence::get_precedence(&Token::LParen) == Precedence::Paren);
        assert!(Precedence::get_precedence(&Token::RParen) == Precedence::Paren);

        assert!(Precedence::Paren > Precedence::Power);
        assert!(Precedence::Paren > Precedence::MulDiv);
        assert!(Precedence::Lowest < Precedence::PlusMinus);
        assert!(Precedence::PlusMinus < Precedence::MulDiv);
        assert!(Precedence::Power > Precedence::MulDiv);
    }

    #[test]
    fn test_parser() {
        let lexer = Lexer::new(String::from("1 + 2"));
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        dbg!(result);
    }
}
