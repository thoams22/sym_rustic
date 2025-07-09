use crate::{
    ast::{Expr, SimplifyError},
    explanation::FormattingObserver,
};

use super::Expression;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Constant {
    Pi,
    E,
    Tau,
}

impl Expr for Constant {
    fn simplify(
        &mut self,
        _explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        Ok(Expression::Constant(self.clone()))
    }

    fn is_equal(&self, other: &Constant) -> bool {
        self == other
    }

    fn contains_var(&self, _variable: &str) -> bool {
        false
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Constant::Pi => write!(f, "pi"),
            Constant::E => write!(f, "e"),
            Constant::Tau => write!(f, "tau"),
        }
    }
}

impl Constant {
    pub fn evaluate(&self) -> f64 {
        match self {
            Constant::Pi => std::f64::consts::PI,
            Constant::E => std::f64::consts::E,
            Constant::Tau => std::f64::consts::TAU,
        }
    }
}