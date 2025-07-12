use crate::{ast::Expr, explanation::FormattingObserver, prints::PrettyPrints};

use super::{Expression, SimplifyError, numeral};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Multiplication {
    pub terms: Vec<Expression>,
    pub simplified: bool,
}

impl Multiplication {
    pub fn new(terms: Vec<Expression>, simplified: bool) -> Self {
        Self { terms, simplified }
    }
}

impl Expr for Multiplication {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let simplified_terms: Vec<Expression> = self.terms
        .iter_mut()
        .map(|term| term.simplify(explanation))
        .collect::<Result<Vec<Expression>, _>>()?;
    self.simplify_multiplication(simplified_terms, explanation)
    }

    fn is_equal(&self, other: &Multiplication) -> bool {
        Expression::compare_expression_vectors(&self.terms, &other.terms)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.terms.iter().any(|expr| expr.contains_var(variable))
    }

    fn is_single(&self) -> bool {
        self.terms.len() <= 1
    }
    
    fn contains(&self, expression: &Expression) -> bool {
        self.terms.iter().any(|term| {
            term.is_equal(expression) || term.contains(expression)
        })
        // Check if (a * x * y) is in (a * y * z * x)
        || if let Expression::Multiplication(mul) = expression {
            // Must be at least the same size for it to contains the expression
            if self.terms.len() >= mul.terms.len() {
                mul.terms.iter().all(|mul_term| {
                    self.terms.iter().any(|self_term| {
                        self_term.is_equal(mul_term)
                    })
                }) 
            } else {
                false
            }
        } else {
            false
        }
    }
    
}

impl std::fmt::Display for Multiplication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let terms: Vec<String> = self.terms
        .iter()
        .map(|term| {
            if term.is_single() {
                term.to_string()
            } else {
                format!("({})", term)
            }
        })
        .collect();
    write!(f, "{}", terms.join(" * "))
    }
}

impl Multiplication {
    /// Simplify multiplication
    pub(crate) fn simplify_multiplication(
        &self,
        simplified_terms: Vec<Expression>,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::multiplication(simplified_terms.clone());

        if let Some(explanation) = explanation {
            explanation.simplify_step_started(&before);
        }

        let mut result = simplified_terms;
        let mut negative: bool = false;

        let mut i: usize = 0;
        while i < result.len() {
            let mut j: usize = i + 1;
            while j < result.len() {
                let before = Expression::multiplication(result.clone());
                match (&result[i], &result[j]) {
                    // a * 0 => 0
                    (Expression::Number(numeral::Numeral::Integer(0)), _)
                    | (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiplying by zero yield zero",
                                &before,
                                &Expression::integer(0),
                            );
                        }
                        return Ok(Expression::integer(0));
                    }
                    // a * mult => expand mult
                    (_, Expression::Multiplication(mult)) => {
                        result.extend(mult.terms.clone());
                        result.swap_remove(j);
                    }
                    (Expression::Multiplication(mult), _) => {
                        result.extend(mult.terms.clone());
                        result.swap_remove(i);
                    }
                    // -a * -b => a * b
                    (Expression::Negation(a), Expression::Negation(b)) => {
                        if let Some(explanation) = explanation {
                            let after = Expression::multiplication(vec![a.term.clone(), b.term.clone()]);
                            explanation.rule_applied(
                                "Multiply two negative expression cancel the negative",
                                &before,
                                &after,
                            );
                        };
                        let new_b = b.term.clone();
                        result[i] = a.term.clone();
                        result[j] = new_b;
                    }
                    // a * -b => -(a * b)
                    (_, Expression::Negation(b)) => {
                        negative = !negative;
                        result[j] = b.term.clone();
                    }
                    // -a * b => -(a * b)
                    (Expression::Negation(b), _) => {
                        negative = !negative;
                        result[i] = b.term.clone();
                    }
                    // 1 * a => a
                    (Expression::Number(numeral::Numeral::Integer(1)), a)
                    | (a, Expression::Number(numeral::Numeral::Integer(1))) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply by one stay the same",
                                &before,
                                a,
                            );
                        }
                        result[i] = a.clone();
                        result.swap_remove(j);
                    }
                    (Expression::Number(a), Expression::Number(b)) => {
                        let mut after = Expression::Number(a.mul(b));
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply numbers",
                                &before,
                                &after,
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a * a => a^2
                    (a, b) if a.is_equal(b) => {
                        let mut after = Expression::exponentiation(
                            result[i].clone(),
                            Expression::integer(2),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply the same expression",
                                &before,
                                &after,
                            );
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // (a + b i)(c + d i) => ac - bd + ad i + bc i
                    (
                        Expression::Complex(lhs),
                        Expression::Complex(rhs),
                    ) => {
                        let mut after = Expression::complex(
                            Expression::addition(vec![
                                Expression::multiplication(vec![
                                    lhs.real.clone(),
                                    rhs.real.clone(),
                                ]),
                                Expression::negation(Expression::multiplication(vec![
                                    lhs.imag.clone(),
                                    rhs.imag.clone(),
                                ])),
                            ]),
                            Expression::addition(vec![
                                Expression::multiplication(vec![
                                    lhs.real.clone(),
                                    rhs.imag.clone(),
                                ]),
                                Expression::multiplication(vec![
                                    lhs.imag.clone(),
                                    rhs.real.clone(),
                                ]),
                            ]),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply two complex expression\n(a + b i)(c + d i) => ac - bd + i(ad + bc)",
                                &before,
                                &after
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a(b + c i) => ab + ac i
                    (a, Expression::Complex(comp)) | (Expression::Complex(comp), a) => {
                        let mut after = Expression::complex(
                            Expression::multiplication(vec![a.clone(), comp.real.clone()]),
                            Expression::multiplication(vec![a.clone(), comp.imag.clone()]),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply with a complex expression\n(a + b i)(c + d i) => a(b + c i) => ab + iac",
                                &before,
                                &after
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // (a + b)(c + d) => ac + ad + bc + bd
                    (Expression::Addition(lhs), Expression::Addition(rhs)) => {
                        let mut after = Expression::addition(
                            lhs.terms.iter()
                                .flat_map(|l_term| {
                                    rhs.terms.iter()
                                        .map(|r_term| {
                                            Expression::multiplication(vec![
                                                l_term.clone(),
                                                r_term.clone(),
                                            ])
                                        })
                                        .collect::<Vec<Expression>>()
                                })
                                .collect(),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply additions by distributing each term",
                                &before,
                                &after,
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a(b + c) => ab + ac
                    (a, Expression::Addition(add)) | (Expression::Addition(add), a) => {
                        let mut after = Expression::addition(
                            add.terms
                                .iter()
                                .map(|term| {
                                    Expression::multiplication(vec![term.clone(), a.clone()])
                                })
                                .collect(),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply by distributing each term",
                                &before,
                                &after,
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a^x * a^y => a^(x + y)
                    (
                        Expression::Exponentiation(lhs),
                        Expression::Exponentiation(rhs),
                    ) => {
                        if lhs.base.is_equal(&rhs.base) {
                            let mut after = Expression::exponentiation(
                                lhs.base.clone(),
                                Expression::addition(vec![
                                    lhs.expo.clone(),
                                    rhs.expo.clone(),
                                ]),
                            );
                            if let Some(explanation) = explanation {
                                explanation.rule_applied(
                                    "Multiply terms with the the same base by adding the exponents",
                                    &before,
                                    &after,
                                );
                            };
                            result[i] = after.simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // a*(b/c) => (a*c)/b
                    (norm, Expression::Division(div))
                    | (Expression::Division(div), norm) => {
                        let mut after = Expression::division(
                            Expression::multiplication(vec![norm.clone(), div.num.clone()]),
                            div.den.clone(),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiplying by a fraction is given by\na*(b/c) => (a*b)/c",
                                &before,
                                &after,
                            );
                        }
                        result.swap_remove(j);
                        result[i] = after.simplify(explanation)?;
                    }
                    // a^x * a => a^(x + 1)
                    (Expression::Exponentiation(exp), a)
                    | (a, Expression::Exponentiation(exp)) => {
                        if exp.base.is_equal(a) {
                            let mut after = Expression::exponentiation(
                                exp.base.clone(),
                                Expression::addition(vec![
                                    exp.expo.clone(),
                                    Expression::integer(1),
                                ]),
                            );
                            if let Some(explanation) = explanation {
                                explanation.rule_applied(
                                    "Multiply terms with the the same base by adding the exponents",
                                    &before,
                                    &after,
                                );
                            }
                            result[i] = after.simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    _ => j += 1,
                }
            }
            i += 1
        }

        if result.len() <= 3 {
            result = result
                .iter()
                .filter(|term| !matches!(term, Expression::Number(numeral::Numeral::Integer(1))))
                .cloned()
                .collect();
        }

        let sol = if result.len() == 1 {
            result.pop().unwrap()
        } else if result.is_empty() {
            Expression::integer(1)
        } else {
            Expression::multiplication(result)
        };

        if negative {
            let mut result = Expression::negation(sol);
            if let Some(explanation) = explanation {
                explanation.simplify_step_completed(&result);
            }
            result.simplify(explanation)
        } else {
            if let Some(explanation) = explanation {
                explanation.simplify_step_completed(&sol);
            }
            Ok(sol)
        }
    }
}

impl PrettyPrints for Multiplication {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        if self.terms.is_empty() {
            "1".to_string()
        } else if self.terms.len() == 1 {
            self.terms[0].calculate_tree(indent)
        } else {
            let mut result = String::from("Multiplication:\n");
            for (i, term) in self.terms.iter().enumerate() {
                result.push_str(&format!(
                    "{}{}{}",
                    next_indent_str,
                    "* ",
                    term.calculate_tree(next_indent)
                ));
                if i < self.terms.len() - 1 {
                    result.push('\n');
                }
            }
            result
        }
    }

    fn calculate_positions(
        &self,
        memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        let mut pos = prev_pos;
        let below_height = self.get_below_height(memoization);

        self.terms.iter().enumerate().for_each(|(i, x)| {
            let new_height = pos.0 + below_height - x.get_below_height(memoization);
            x.calculate_positions(memoization, position, (new_height, pos.1));
            pos.1 += x.get_length(memoization);
            if i < self.terms.len() - 1 {
                position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                pos.1 += 1;
                position.push(("*".to_string(), (pos.0 + below_height, pos.1)));
                pos.1 += 1;
                position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                pos.1 += 1;
            }
        });
    }

    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self
                .terms
                .iter()
                .map(|x| x.get_below_height(memoization))
                .max()
                .unwrap_or(0)
    }

    fn get_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        let mut max_top_height = 0;
        let mut max_below_height = 0;

        self.terms.iter().for_each(|x| {
            let x_height = x.get_height(memoization);
            let x_below = x.get_below_height(memoization);

            max_top_height = (x_height - x_below).max(max_top_height);
            max_below_height = x_below.max(max_below_height);
        });

        max_top_height + max_below_height
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        let length = self
        .terms
        .iter()
        .map(|x| x.get_length(memoization))
        .sum::<usize>();

    self.terms.len() * 3 - 3 + length
    }
}