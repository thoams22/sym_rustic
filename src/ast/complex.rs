use crate::{
    ast::{numeral, Expr, SimplifyError},
    explanation::FormattingObserver, prints::PrettyPrints,
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

    fn is_single(&self) -> bool {
        false
    }
    
    fn contains(&self, expression: &Expression) -> bool {
        self.real.contains(expression) || self.imag.contains(expression) || 
        self.real.is_equal(expression) || self.imag.is_equal(expression)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}i*{}",
            if self.real.is_equal(&Expression::integer(0)) {
                String::new()
            } else {
                format!("{} + ", self.real)
            },
            if self.imag.is_equal(&Expression::integer(0)) {
                String::new()
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


impl PrettyPrints for Complex {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);
        
        format!(
            "Complex:\n{}{}\n{}i {}",
            next_indent_str,
            self.real.calculate_tree(next_indent),
            next_indent_str,
            self.imag.calculate_tree(next_indent)
        )
    }

    fn calculate_positions(
        &self,
        memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        let mut pos = prev_pos;
        self.real.calculate_positions(memoization, position, pos);
        pos.1 += self.real.get_length(memoization);
        position.push((" ".to_string(), pos));
        pos.1 += 1;
        position.push(("+".to_string(), pos));
        pos.1 += 1;
        position.push((" ".to_string(), pos));
        pos.1 += 1;
        self.imag.calculate_positions(memoization, position, pos);
        pos.1 += self.imag.get_length(memoization);
        position.push(("i".to_string(), pos));
    }

    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self
                .real
                .get_below_height(memoization)
                .max(self.imag.get_below_height(memoization))
    }

    fn get_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.
                real
                .get_height(memoization)
                .max(self.imag.get_height(memoization))
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.real.get_length(memoization) + self.imag.get_length(memoization) + 4

    }
}