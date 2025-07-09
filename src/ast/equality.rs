use crate::{
    ast::{Expr, SimplifyError},
    explanation::FormattingObserver, prints::PrettyPrints,
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

    fn is_single(&self) -> bool {
        false
    }
}

impl std::fmt::Display for Equality {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}

impl PrettyPrints for Equality {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        format!(
            "Equality:\n{}{}\n{}= {}",
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
        position.push(("=".to_string(), pos));
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