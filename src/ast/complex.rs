use super::Expression;

impl Expression {

    /// Returns `true`  if the expression contains a `Expression::Complex` term.
    /// Should be used on simplified expressions.
    pub fn is_complex(&self) -> bool {
        match self {
            Expression::Complex(_, _) => true,
            Expression::Addition(terms) | Expression::Multiplication(terms) => {
                terms.iter().any(|term| term.is_complex())
            }
            Expression::Negation(term) => term.is_complex(),
            Expression::Division(a, b) => a.is_complex() || b.is_complex(),
            Expression::Exponentiation(a, b) => a.is_complex() || b.is_complex(),
            _ => false,
        }
    }

    /// Returns An `Option` containing the conjugate if expr is `Expression::Complex`, `None` otherwise
    pub fn complex_conjugate(expr: Expression) -> Option<Expression> {
        match expr {
            Expression::Complex(real, imag) => Some(Expression::Complex(
                real,
                Box::new(Expression::Negation(Box::new(*imag))),
            )),
            _ => None,
        }
    }
}