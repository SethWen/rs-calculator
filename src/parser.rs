use std::cmp::Ordering;

use crate::{lexer::Lexer, token::Token};

// public class Precedence {

//     public static final int LOWEST = 0;
//     public static final int EQUALS = 1;
//     public static final int LESS_GREATER = 2;
//     public static final int SUM = 3;

//     public static final int PRODUCT = 4;

//     public static final int POWER = 5;

//     public static final int PREFIX = 6;
//     public static final int CALL = 7;

//     static HashMap<TokenType, Integer> precedences = new HashMap<>();

//     static {
//         precedences.put(TokenType.PLUS, SUM);
//         precedences.put(TokenType.MINUS, SUM);
//         precedences.put(TokenType.SLASH, PRODUCT);
//         precedences.put(TokenType.ASTERISK, PRODUCT);
//         precedences.put(TokenType.HAT, POWER);
//         precedences.put(TokenType.LPAREN, CALL);
//     }

//     static int getPrecedence(TokenType t) {
//         return precedences.getOrDefault(t, LOWEST);
//     }

// }

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
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        Self { lexer }
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
}
