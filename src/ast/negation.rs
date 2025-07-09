use crate::{
    ast::{numeral::Numeral, Expr, SimplifyError},
    explanation::FormattingObserver, prints::PrettyPrints,
};

use super::Expression;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Negation {
    pub term: Expression,
    pub simplified: bool,
}

// Constructor
impl Negation {
    pub fn new(term: Expression, simplified: bool) -> Self {
        Self {
            term,
            simplified,
        }
    }
}

impl Expr for Negation {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let expr = self.term.simplify(explanation)?;
        match expr {
            // --a => a
            Expression::Negation(a) => Ok(a.term),
            // -(a b) => -(a b)
            // -(Num b c) => -(Num) b c
            Expression::Multiplication(mut a) => {
                // Find a Expression::integer and transform it to Expression::Negation(Expression::integer)
                if a.terms.iter_mut().any(|term| {
                    if let Expression::Number(Numeral::Integer(n)) = term {
                        *term = Expression::negation(Expression::integer(*n),
                        );
                        true
                    } else {
                        false
                    }
                }) {
                    Ok(Expression::Multiplication(a))
                } else {
                    Ok(Expression::negation(Expression::multiplication(
                        a.terms,
                    )))
                }
            }
            // -(a + b i) => -a -(b) i
            Expression::Complex(comp) => Expression::complex(
                Expression::negation(comp.real),
                Expression::negation(comp.imag),
            )
            .simplify(explanation),
            // -(a + b) => -a - b
            Expression::Addition(add) => {
                let terms = add.terms
                    .iter()
                    .map(|elem| Expression::negation(elem.clone()))
                    .collect();
                Expression::addition(terms).simplify(explanation)
            }
            // -0 => 0
            Expression::Number(Numeral::Integer(0)) => {
                Ok(Expression::integer(0))
            }
            expr => Ok(Expression::Negation(Box::new(Negation::new(expr, true)))),
        }
    }

    fn is_equal(&self, other: &Negation) -> bool {
        self.term.is_equal(&other.term) 
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.term.contains_var(variable)
    }

    fn is_single(&self) -> bool {
        true
    }
}

impl std::fmt::Display for Negation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "-{}",
            if self.term.is_single() {
                format!("{}", self.term)
            } else {
                format!("({})", self.term)
            }
        )
    }
}

impl PrettyPrints for Negation {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        format!(
            "Negation:\n{}- {}",
            next_indent_str,
            self.term.calculate_tree(indent)
        )
    }

    fn calculate_positions(
        &self,
        memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        let mut pos = prev_pos;
        position.push(("-".to_string(), pos));
        pos.1 += 1;
        position.push((" ".to_string(), pos));
        pos.1 += 1;
        self.term.calculate_positions(memoization, position, pos);
    }

    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.term.get_below_height(memoization)
    }

    fn get_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.term.get_height(memoization)
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.term.get_length(memoization) + 2
    }
}