use crate::{
    ast::{Expr, SimplifyError, numeral},
    explanation::FormattingObserver,
};

use super::Expression;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Complex {
    pub real: Expression,
    pub imag: Expression,
    pub simplified: bool,
}

// Constructor
impl Complex {
    pub fn new(real: Expression, imag: Expression, simplified: bool) -> Self {
        Self {
            real,
            imag,
            simplified,
        }
    }
}

impl Expr for Complex {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let real = self.real.simplify(explanation)?;
        let imag = self.imag.simplify(explanation)?;
        if imag == Expression::Number(numeral::Numeral::Integer(0)) {
            Ok(real)
        } else {
            Ok(Expression::complex(real, imag))
        }
    }

    fn is_equal(&self, other: &Complex) -> bool {
        self.real.is_equal(&other.real) && self.imag.is_equal(&other.imag)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.real.contains_var(variable) || self.imag.contains_var(variable)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}i*{}",
            if self.real.is_equal(&Expression::integer(0)) {
                format!("")
            } else {
                format!("{} + ", self.real)
            },
            if self.imag.is_equal(&Expression::integer(0)) {
                format!("")
            } else if self.imag.is_single() {
                format!("{}", self.imag)
            } else {
                format!("({})", self.imag)
            }
        )
    }
}

impl Complex {
    /// Returns the conjugate of the Complex
    pub fn conjugate(self) -> Complex {
        Self {
            real: self.real,
            imag: Expression::negation(self.imag),
            simplified: false,
        }
    }
}
