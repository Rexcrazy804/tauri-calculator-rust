pub mod intopos;
pub mod poseval;

pub fn compute(expression: &str) -> f32 {
    let postfix = intopos::get_postfix(expression);
    poseval::evaluate(&postfix)
}
