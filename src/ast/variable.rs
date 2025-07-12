use std::fmt;

use crate::{ast::{Expr, Expression, SimplifyError}, explanation::FormattingObserver, prints::PrettyPrints};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}

impl Expr for Variable {
    fn simplify(
        &mut self,
        _explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        Ok(Expression::Variable(self.clone()))
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.name == other.name 
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.name == variable
    }

    fn is_single(&self) -> bool {
        true
    }
    
    fn contains(&self, expression: &Expression) -> bool {
        if let Expression::Variable(var) = expression {
            self.name == var.name
        } else {
            false
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PrettyPrints for Variable {
    fn calculate_tree(&self, _indent: usize) -> String {
        self.name.to_string()
    }

    fn calculate_positions(
        &self,
        _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        for (i, c) in self.name.chars().enumerate() {
            position.push((c.to_string(), (prev_pos.0, prev_pos.1 + i)));
        }
    }

    fn get_below_height(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        0
    }

    fn get_height(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        1
    }

    fn get_length(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.name.len()
    }
}