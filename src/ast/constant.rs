use crate::{
    ast::{Expr, SimplifyError},
    explanation::FormattingObserver, prints::PrettyPrints,
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
        Ok(Expression::Constant(*self))
    }

    fn is_equal(&self, other: &Constant) -> bool {
        self == other
    }

    fn contains_var(&self, _variable: &str) -> bool {
        false
    }
    
    fn is_single(&self) -> bool {
        true
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

impl PrettyPrints for Constant {
    fn calculate_tree(&self, _indent: usize) -> String {
        self.to_string()
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
        0
    }

    fn get_height(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        1
    }

    fn get_length(&self, _memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        match self {
            Constant::Pi => 2,
            Constant::E => 1,
            Constant::Tau => 3,
        }
    }
}