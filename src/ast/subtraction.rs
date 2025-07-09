use crate::{
    ast::{Expr, SimplifyError, numeral},
    explanation::FormattingObserver,
};

use super::Expression;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Subtraction {
    pub left: Expression,
    pub right: Expression,
    pub simplified: bool,
}

// Constructor
impl Subtraction {
    pub fn new(left: Expression, right: Expression, simplified: bool) -> Self {
        Self {
            left,
            right,
            simplified,
        }
    }
}

impl Expr for Subtraction {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let left = self.left.simplify(explanation)?;
        let right = self.right.simplify(explanation)?;

        let before = &Expression::subtraction(left.clone(), right.clone());
        match (left, right) {
            // a - 0
            (lhs, Expression::Number(numeral::Numeral::Integer(0))) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Subtracting zero stay the same", before, &lhs);
                }
                Ok(lhs)},
            // 0 - a
            (Expression::Number(numeral::Numeral::Integer(0)), rhs) => {
                let after  = Expression::negation(rhs);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Adding zero stay the same", before, &after);
                }
                Ok(after)
            }
            // a - b => c 
            (Expression::Number(lhs), Expression::Number(rhs)) => {
                let after  = lhs.sub(&rhs);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Subtracting numbers", before, &after);
                }
                Ok(after)
            }
            // -a - b => -(c)
            (Expression::Negation(lhs), Expression::Number(rhs)) => {
                if let Expression::Number(inner_lhs) = lhs.term {
                    let after  = Expression::negation(Expression::Number(inner_lhs.add(&rhs)));
                    if let Some(explanation) = explanation {
                        explanation.rule_applied("Subtracting numbers", before, &after);
                    }
                    Ok(after)
                } else {
                    Expression::addition(vec![Expression::Negation(lhs), Expression::negation(Expression::Number(rhs))])
                    .simplify(explanation)
                }
            }
            (lhs, rhs) => {
                Expression::addition(vec![lhs, Expression::negation(rhs)])
                    .simplify(explanation)
            }
        }
    }

    fn is_equal(&self, other: &Subtraction) -> bool {
        self.left.is_equal(&other.left) && self.right.is_equal(&other.right)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.left.contains_var(variable) || self.right.contains_var(variable)
    }
}

impl std::fmt::Display for Subtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}i*{}",
            if self.left.is_equal(&Expression::integer(0)) {
                format!("")
            } else {
                format!("{} + ", self.left)
            },
            if self.right.is_equal(&Expression::integer(0)) {
                format!("")
            } else if self.right.is_single() {
                format!("{}", self.right)
            } else {
                format!("({})", self.right)
            }
        )
    }
}