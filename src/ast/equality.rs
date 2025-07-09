use crate::{
    ast::{Expr, SimplifyError},
    explanation::FormattingObserver,
};

use super::Expression;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Equality {
    pub left: Expression,
    pub right: Expression,
    pub simplified: bool,
}

// Constructor
impl Equality {
    pub fn new(left: Expression, right: Expression, simplified: bool) -> Self {
        Self {
            left,
            right,
            simplified,
        }
    }
}

impl Expr for Equality {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let left = self.left.simplify(explanation)?;
        let right = self.right.simplify(explanation)?;
        Ok(Expression::equality(left, right))
        
    }

    fn is_equal(&self, other: &Equality) -> bool {
        self.left.is_equal(&other.left) && self.right.is_equal(&other.right)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.left.contains_var(variable) || self.right.contains_var(variable)
    }
}

impl std::fmt::Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}