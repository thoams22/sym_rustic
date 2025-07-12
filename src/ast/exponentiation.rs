use crate::{ast::{function::{Function, FunctionType}, Expr}, explanation::FormattingObserver, prints::PrettyPrints, utils};

use super::{Expression, SimplifyError, numeral};


#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Exponentiation {
    pub base: Expression,
    pub expo: Expression,
    pub simplified: bool,
}

// Constructor
impl Exponentiation {
    pub fn new(base: Expression, expo: Expression, simplified: bool) -> Self {
        Self {
            base,
            expo,
            simplified,
        }
    }
}

impl Expr for Exponentiation {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let base = self.base.simplify(explanation)?;
        let expo = self.expo.simplify(explanation)?;

        self.simplify_exponentiation(base, expo, explanation)
    }

    fn is_equal(&self, other: &Exponentiation) -> bool {
        self.base.is_equal(&other.base) && self.expo.is_equal(&other.expo)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.base.contains_var(variable) || self.expo.contains_var(variable)
    }

    fn is_single(&self) -> bool {
        false
    }
    
    fn contains(&self, expression: &Expression) -> bool {
        self.base.contains(expression) || self.expo.contains(expression) || 
        self.base.is_equal(expression) || self.expo.is_equal(expression) ||
        if let Expression::Exponentiation(exp) = expression {
            // Check if e^2 is in e^(2 * 3) 
            if let Expression::Multiplication(mul) = &exp.expo {
                mul.contains(&self.expo) && exp.base.is_equal(&self.base)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl std::fmt::Display for Exponentiation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}^{}",
            if self.base.is_single() {
                format!("{}", self.base)
            } else {
                format!("({})", self.base)
            },
            if self.expo.is_single() {
                format!("{}", self.expo)
            } else {
                format!("({})", self.expo)
            }
        )
    }
}

impl Exponentiation {
    pub(crate) fn simplify_exponentiation(
        &mut self,
        lhs: Expression,
        rhs: Expression,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::exponentiation(lhs.clone(), rhs.clone());

        let result = match (lhs, rhs) {
            // 0^0 => ZeroExponentiationZero
            (
                Expression::Number(numeral::Numeral::Integer(0)),
                Expression::Number(numeral::Numeral::Integer(0)),
            ) => Err(SimplifyError::ZeroExponentiationZero),
            // a^0 => 1
            (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Something to the 0th power is one", &before, &Expression::integer(1));
                }
                Ok(Expression::integer(1))
            }
            // 1^x
            (Expression::Number(numeral::Numeral::Integer(1)), Expression::Number(_))  => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("One to any power is one", &before, &Expression::integer(1));
                }
                Ok(Expression::integer(1))
            }
            // (Expression::Number(numeral::Numeral::Integer(1)), Expression::Negation(x)) if *x == Expression::Number(_) => {
            //     rule = "using 1^(-x) => 1";
            //     Ok(Expression::integer(1))
            // }
            // a^1 => a
            (lhs, Expression::Number(numeral::Numeral::Integer(1))) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Anything to the 1st power stay the same", &before, &lhs);
                }
                Ok(lhs)
            }
            // sqrt(a)^2 => a
            (
                Expression::Function(Function { name: FunctionType::Sqrt, args, simplified: _ }),
                Expression::Number(numeral::Numeral::Integer(2)),
            ) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Square root to the 2th power cancel", &before, &args[0]);
                }
                Ok(args[0].clone())
            }
            // root(x, a)^x => a
            (
                Expression::Function(Function { name: FunctionType::Root, args, simplified: _ }),
                Expression::Number(numeral::Numeral::Integer(x)),
            ) if args[0] == Expression::Number(numeral::Numeral::Integer(x)) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("nth root to nth power cancel", &before, &Expression::integer(x));
                }
                Ok(args[1].clone())
            }
            // (a^b)^c => a^(b*c)
            (Expression::Exponentiation(exp), rhs) => {
                    if let Some(explanation) = explanation {
                        explanation.rule_applied("Multiply the exponent", &before, &Expression::exponentiation(
                            exp.base.clone(),
                            Expression::multiplication(vec![exp.expo.clone(), rhs.clone()]),
                        ));
                    }
                    Expression::exponentiation(
                        exp.base,
                        Expression::multiplication(vec![exp.expo, rhs]),
                    )
                    .simplify(explanation)
            }
            // (a*b)^c => a^c*b^c
            (Expression::Multiplication(mul), rhs) => {
                let mut after: Expression = Expression::multiplication( 
                    mul.terms
                    .iter()
                    .map(|term| {
                    Expression::exponentiation(term.clone(), rhs.clone())
                }).collect());
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Distribute the exponent", &before, &after);
                };
                after.simplify(explanation)
            }
            // (a + b)^n where n is a integer
            (Expression::Addition(add), Expression::Number(numeral::Numeral::Integer(n))) => {
                let mut after = utils::multinomial_expansion(&add.terms, n);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Use the multinomial theoerm", &before, &after);
                };
                after.simplify(explanation)
            }
            // a^b => a^b
            (lhs, rhs) => Ok(Expression::Exponentiation(Box::new(Exponentiation::new(lhs, rhs, true)))),
        };

        result
    }
}

impl PrettyPrints for Exponentiation {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        format!(
            "Exponentiation:\n{}{}\n{}^ {}",
            next_indent_str,
            self.base.calculate_tree(next_indent),
            next_indent_str,
            self.expo.calculate_tree(next_indent)
        )
    }

    fn calculate_positions(
        &self,
        memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        let mut pos = prev_pos;
                if matches!(
                    self.base,
                    Expression::Addition(_)
                        | Expression::Multiplication(_)
                        | Expression::Exponentiation(_)
                        | Expression::Complex(_)
                        | Expression::Division(_)
                ) {
                    Self::calculate_parenthesis(
                        position,
                        pos,
                        true,
                        self.base.get_height(memoization),
                    );
                    pos.1 += 1;
                }
                self.base.calculate_positions(memoization, position, pos);
                pos.1 += self.base.get_length(memoization);
                if matches!(
                    self.base,
                    Expression::Addition(_)
                        | Expression::Multiplication(_)
                        | Expression::Exponentiation(_)
                        | Expression::Complex(_)
                        | Expression::Division(_)
                ) {
                    Self::calculate_parenthesis(
                        position,
                        pos,
                        false,
                        self.base.get_height(memoization),
                    );
                    pos.1 += 1;
                }
                pos.0 += self.base.get_height(memoization);
                self.expo.calculate_positions(memoization, position, pos);
    }

    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.base.get_below_height(memoization)
    }

    fn get_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.base.get_height(memoization) + self.expo.get_height(memoization)
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.base.get_length(memoization)
        + self.expo.get_length(memoization)
        + if matches!(
            self.base,
            Expression::Addition(_)
                | Expression::Multiplication(_)
                | Expression::Exponentiation(_)
                | Expression::Complex(_)
                | Expression::Division(_)
        ) {
            2
        } else {
            0
        }
    }
}