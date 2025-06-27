use crate::utils;

use super::{Expression, SimplifyError};

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Numeral {
    Integer(u64),
    Rational(u64, u64),
}

impl Numeral {
    pub fn simplify(&mut self, explanation: &mut Option<Vec<String>>) -> Result<Numeral, SimplifyError> {

        if let Numeral::Rational(n, d) = self {
            let gcd = utils::gcd(*n, *d);
            let result = if *d / gcd == 1 {
                Ok(Numeral::Integer(1))
            } else if *n / gcd == 0 {
                Ok(Numeral::Integer(0))
            } else if *d / gcd == 0 {
                Err(SimplifyError::DivisionByZero)
            } else {
                Ok(Numeral::Rational(*n / gcd, *d / gcd))
            }?;

            if let Some(explanation) = explanation {
                explanation.push(format!(
                    "Simplified Numeral: {}",
                    result.clone()
                ));
            }

            Ok(result)

        } else {
            Ok(*self)
        }
    }

    pub fn add(&self, other: &Numeral) -> Numeral {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => Numeral::Integer(n + m),
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => {
                Numeral::Rational(n * p + m * d, d * p)
            }
            (Numeral::Integer(m), Numeral::Rational(n, d)) |
            (Numeral::Rational(n, d), Numeral::Integer(m))  => {
                Numeral::Rational(m * d + n, *d)
            }

        }
    }

    // TODO: Check for overflow
    pub fn sub(&self, other: &Numeral) -> Expression {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => {
                if m > n {
                    return Expression::negation(Expression::integer(m - n));
                } else {
                    return Expression::integer(n - m);
                }
            },
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => {
                if m * d > n * p {
                    return Expression::negation(Expression::rational(m * d - n * p, d * p));
                } else {
                    return Expression::rational(n * p - m * d, d * p);
                }
            }
            (Numeral::Integer(m), Numeral::Rational(n, d)) => {
                if m * d > *n {
                    return Expression::rational(m * d - n, *d);
                } else {
                    return Expression::negation(Expression::rational(n - m * d, *d));
                }
            }
            (Numeral::Rational(n, d), Numeral::Integer(m))  => {
                if m * d > *n {
                    return Expression::negation(Expression::rational(m * d - n, *d));
                } else {
                    return Expression::rational(n - m * d, *d);
                }
            }
        }
    }

    pub fn mul(&self, other: &Numeral) -> Numeral {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => Numeral::Integer(n * m),
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => {
                Numeral::Rational(n * m, d * p)
            }
            (Numeral::Integer(m), Numeral::Rational(n, d)) |
            (Numeral::Rational(n, d), Numeral::Integer(m))  => {
                Numeral::Rational(m * n, *d)
            }
        }
    }   

    pub fn div(&self, other: &Numeral) -> Numeral {
        match (self, other) {
            (Numeral::Integer(n), Numeral::Integer(m)) => Numeral::Rational(*n, *m),
            (Numeral::Rational(n, d), Numeral::Rational(m, p)) => {
                Numeral::Rational(n * p, m * d)
            }
            (Numeral::Integer(m), Numeral::Rational(n, d))   => {
                Numeral::Rational(m * d, *n)
            }
            (Numeral::Rational(n, d), Numeral::Integer(m)) => {
                Numeral::Rational(*n, m * d)
            }
        }
    }
}

impl std::fmt::Display for Numeral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Numeral::Integer(n) => write!(f, "{}", n),
            Numeral::Rational(n, d) => write!(f, "{} / {}", n, d),
        }
    }
}