use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Expression {
    Prefix { operator: String, right: Box<Expression> },
    Infix { operator: String, left: Box<Expression>, right: Box<Expression> },
    Number(f64),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix { operator, left, right } => write!(f, "({} {} {})", left, operator, right),
            Expression::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Expression {
    // fn operator(&self) -> &'static str {
    //     match self {
    //         Expression::Prefix {.. } => "-",
    //         Expression::Infix { .. } => "+",
    //         Expression::Number(_) => "",
    //     }
    // }
}
