#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,

    Eq,
    Assign,

    Float(f64),
    Int(i64),
    Ident(String),

    Illegal,
    Eof,
}
