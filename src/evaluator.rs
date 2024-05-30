use crate::expression::Expression;

pub fn eval(expr: Expression) -> f64 {
    match expr {
        Expression::Prefix { operator, right } => {
            todo!()
        }
        Expression::Infix {
            operator,
            left,
            right,
        } => todo!(),
        Expression::Number(num) => num,
    }
}

pub fn op(expr: Expression) {
    todo!()
}

pub fn minus(expr: Expression) {
    todo!()
}
