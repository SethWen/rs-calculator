use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer { input, position: 0, read_position: 0, ch: '\0' };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::Eq,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mul,
            '/' => Token::Div,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '\0' => Token::Eof,
            _ if self.ch.is_numeric() => {
                return self.read_num();
            }
            _ => Token::Illegal,
        };
        self.read_char();
        token
    }

    fn read_num(&mut self) -> Token {
        let mut literal = String::new();
        while self.ch.is_numeric() {
            literal.push(self.ch);
            self.read_char();
        }
        if self.ch == '.' {
            literal.push(self.ch);
            self.read_char();
            while self.ch.is_numeric() {
                literal.push(self.ch);
                self.read_char();
            }
        }
        Token::Num(literal)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "12.30 + 456 * 789 / (10 + 20)";
        let mut lexer = Lexer::new(input.to_string());
        assert_eq!(lexer.next_token(), Token::Num("12.30".to_owned()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Num("456".to_owned()));
        assert_eq!(lexer.next_token(), Token::Mul);
        assert_eq!(lexer.next_token(), Token::Num("789".to_owned()));
        assert_eq!(lexer.next_token(), Token::Div);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Num("10".to_owned()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Num("20".to_owned()));
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
