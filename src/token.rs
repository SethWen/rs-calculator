#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,

    Power,

    LParen,
    RParen,

    Eq,

    Float(f64),
    Int(i64),

    Illegal,
    Eof,
}
