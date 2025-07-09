use crate::{
    ast::{numeral, Expr, SimplifyError},
    explanation::FormattingObserver, prints::PrettyPrints,
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

    fn is_single(&self) -> bool {
        false
    }
}

impl std::fmt::Display for Subtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            self.left,
            if self.right.is_single() {
                format!("{}", self.right)
            } else {
                format!("({})", self.right)
            }
        )
    }
}

impl PrettyPrints for Subtraction {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        format!(
            "Subtraction:\n{}{}\n{}- {}",
            next_indent_str,
            self.left.calculate_tree(next_indent),
            next_indent_str,
            self.right.calculate_tree(next_indent)
        )
    }

    fn calculate_positions(
        &self,
        memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        
        let mut pos = prev_pos;
        self.left.calculate_positions(memoization, position, pos);
        pos.1 += self.left.get_length(memoization);
        position.push((" ".to_string(), pos));
        pos.1 += 1;
        position.push(("-".to_string(), pos));
        pos.1 += 1;
        position.push((" ".to_string(), pos));
        pos.1 += 1;
        self.right.calculate_positions(memoization, position, pos);
    }

    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self
                .left
                .get_below_height(memoization)
                .max(self.right.get_below_height(memoization))
    }

    fn get_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self
                .left
                .get_height(memoization)
                .max(self.right.get_height(memoization))
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.left.get_length(memoization) + 3 + self.right.get_length(memoization)
    }
}