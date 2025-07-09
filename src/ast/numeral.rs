use crate::{
    ast::Expr, explanation::FormattingObserver, prints::PrettyPrints, utils
};

use super::{Expression, SimplifyError};

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Numeral {
    Integer(u64),
    Rational(u64, u64),
}

impl Numeral {
    pub fn is_zero(&self) -> bool {
        match self {
            Numeral::Integer(n) | Numeral::Rational(n, _) => *n == 0,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Numeral::Integer(n) => *n == 1,
            Numeral::Rational(n, d) => *n == *d,
        }
    }
}

impl Expr for Numeral {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        if let Numeral::Rational(n, d) = self {
            let gcd = utils::gcd(*n, *d);
            let result = if gcd == 1 {
                Ok(Numeral::Rational(*n, *d))
            } else if *d == gcd {
                let simplified = *n / gcd;
                if let Some(explanation) = explanation {
                    let intermediary = Expression::division(
                        Expression::integer(simplified),
                        Expression::integer(1),
                    );
                    explanation.rule_applied(
                        &format!("Simplified by the common factor: {}", gcd),
                        &Expression::rational(*n, *d),
                        &intermediary,
                    );
                    explanation.rule_applied(
                        "Something divided by one is unchanged",
                        &intermediary,
                        &Expression::integer(simplified),
                    );
                }
                Ok(Numeral::Integer(simplified))
            } else if *n == 0 {
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        "Zero divided by something is Zero",
                        &Expression::Number(*self),
                        &Expression::integer(0),
                    );
                }
                Ok(Numeral::Integer(0))
            } else if *d == 0 {
                Err(SimplifyError::DivisionByZero)
            } else {
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        &format!("Simplified by the common factor: {}", gcd),
                        &Expression::rational(*n, *d),
                        &Expression::rational(*n / gcd, *d / gcd),
                    );
                }
                Ok(Numeral::Rational(*n / gcd, *d / gcd))
            }?;

            Ok(Expression::Number(result))
        } else {
            Ok(Expression::Number(*self))
        }
    }
    
    fn is_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (Numeral::Integer(a), Numeral::Integer(b)) => a == b,
            (Numeral::Rational(a, b), Numeral::Rational(c, d)) => {
                a == c && b == d
            }
            _ => false,
    }}
    
    fn contains_var(&self, _variable: &str) -> bool {
        false
    }

    fn is_single(&self) -> bool {
        match self {
            Numeral::Integer(_) => true,
            Numeral::Rational(_, _) => false,
        }
    }
    
}

impl Numeral {
    pub fn add(&self, other: &Numeral) -> Numeral {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => Numeral::Integer(n + m),
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => {
                Numeral::Rational(n * p + m * d, d * p)
            }
            (Numeral::Integer(m), Numeral::Rational(n, d))
            | (Numeral::Rational(n, d), Numeral::Integer(m)) => Numeral::Rational(m * d + n, *d),
        }
    }

    // TODO: Check for overflow
    pub fn sub(&self, other: &Numeral) -> Expression {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => {
                if m > n {
                    Expression::negation(Expression::integer(m - n))
                } else {
                    Expression::integer(n - m)
                }
            }
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => {
                if m * d > n * p {
                    Expression::negation(Expression::rational(m * d - n * p, d * p))
                } else {
                    Expression::rational(n * p - m * d, d * p)
                }
            }
            (Numeral::Integer(m), Numeral::Rational(n, d)) => {
                if m * d > *n {
                    Expression::rational(m * d - n, *d)
                } else {
                    Expression::negation(Expression::rational(n - m * d, *d))
                }
            }
            (Numeral::Rational(n, d), Numeral::Integer(m)) => {
                if m * d > *n {
                    Expression::negation(Expression::rational(m * d - n, *d))
                } else {
                    Expression::rational(n - m * d, *d)
                }
            }
        }
    }

    pub fn mul(&self, other: &Numeral) -> Numeral {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => Numeral::Integer(n * m),
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => Numeral::Rational(n * m, d * p),
            (Numeral::Integer(m), Numeral::Rational(n, d))
            | (Numeral::Rational(n, d), Numeral::Integer(m)) => Numeral::Rational(m * n, *d),
        }
    }

    pub fn div(&self, other: &Numeral) -> Numeral {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => Numeral::Rational(*n, *m),
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => Numeral::Rational(n * p, m * d),
            (Numeral::Integer(m), Numeral::Rational(n, d)) => Numeral::Rational(m * d, *n),
            (Numeral::Rational(n, d), Numeral::Integer(m)) => Numeral::Rational(*n, m * d),
        }
    }
}

impl std::fmt::Display for Numeral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Numeral::Integer(n) => write!(f, "{}", n),
            Numeral::Rational(n, d) => write!(f, "{}/{}", n, d),
        }
    }
}

impl PrettyPrints for Numeral {
    fn calculate_tree(&self, _indent: usize) -> String {
        format!("{}", self)

    }

    fn calculate_positions(
        &self,
        _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        for (i, c) in self.to_string().chars().enumerate() {
            position.push((c.to_string(), (prev_pos.0, prev_pos.1 + i)));
        }
    }

    fn get_below_height(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Numeral::Integer(_) => 0,
            Numeral::Rational(_num, _den) => 1,
        }
    }

    fn get_height(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Numeral::Integer(_) => 1,
            Numeral::Rational(_num, _den) => 3,
        }
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Numeral::Integer(num) => num.to_string().len(),
            Numeral::Rational(num, den) => {
                Expression::division(Expression::integer(*num), Expression::integer(*den))
                    .get_length(memoization)
            }
        }
    }
}