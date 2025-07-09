use crate::{ast::Expr, explanation::FormattingObserver, prints::PrettyPrints, utils::transform_multiplication};

use super::{Expression, SimplifyError, numeral};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Addition {
    pub terms: Vec<Expression>,
    pub simplified: bool,
}

impl Addition {
    pub fn new(terms: Vec<Expression>, simplified: bool) -> Self {
        Self {
            terms,
            simplified
        }
    }
}

impl Expr for Addition {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let simplified_terms: Vec<Expression> = self.terms
        .iter_mut()
        .map(|term| term.simplify(explanation))
        .collect::<Result<Vec<Expression>, _>>()?;
    self.simplify_addition(simplified_terms, explanation)
    }

    fn is_equal(&self, other: &Addition) -> bool {
        Expression::compare_expression_vectors(&self.terms, &other.terms)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.terms.iter().any(|expr| expr.contains_var(variable))
    }

    fn is_single(&self) -> bool {
        self.terms.len() <= 1
    }
}

impl PrettyPrints for Addition {
   
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        if self.terms.is_empty() {
            "0".to_string()
        } else if self.terms.len() == 1 {
            self.terms[0].calculate_tree(indent)
        } else {
            let mut result = String::from("Addition:\n");
            for (i, term) in self.terms.iter().enumerate() {
                result.push_str(&format!(
                    "{}{}{}",
                    next_indent_str,
                    "+ ",
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
                        position.push(("+".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                        position.push((" ".to_string(), (pos.0 + below_height, pos.1)));
                        pos.1 += 1;
                    }
                });
    }
    
    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.terms
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
        let length = self.terms
        .iter()
        .map(|x| x.get_length(memoization))
        .sum::<usize>();

        self.terms.len() * 3 - 3 + length
    }    
}

impl std::fmt::Display for Addition {
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
    write!(f, "{}", terms.join(" + "))
    }
}

impl Addition {
    pub fn simplify_addition(
        &self,
        terms: Vec<Expression>,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before: Expression = Expression::addition(terms.clone());

        if let Some(explanation) = explanation {
            explanation.simplify_step_started(&before);
        }

        let mut result = terms;

        let mut i: usize = 0;
        while i < result.len() {
            let mut j: usize = i + 1;
            while j < result.len() {
                let before = Expression::addition(result.clone());
                match (&result[i], &result[j]) {
                    (Expression::Addition(add), _) => {
                        result.extend(add.terms.clone());
                        result.swap_remove(i);
                    }
                    (_, Expression::Addition(add)) => {
                        result.extend(add.terms.clone());
                        result.swap_remove(j);
                    }
                    // a + 0 => a
                    (a, Expression::Number(numeral::Numeral::Integer(0)))
                    | (Expression::Number(numeral::Numeral::Integer(0)), a) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Adding zero stay the same", &before, a);
                        }
                        result[i] = a.clone();
                        result.swap_remove(j);
                    }
                    // a + b => c
                    (Expression::Number(a), Expression::Number(b)) => {
                        let after = Expression::Number(a.add(b));
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add numbers", &before, &after);
                        }
                        result[i] = after;
                        result.swap_remove(j);
                    }
                    // a + a => 2a
                    (a, b) if a.is_equal(b) => {
                        let mut after = Expression::multiplication(vec![
                            Expression::integer(2),
                            result[i].clone(),
                        ]);
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add same expression", &before, &after);
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a - b => c
                    // -a + b => c
                    (Expression::Number(a), Expression::Negation(b))
                    | (Expression::Negation(b), Expression::Number(a)) => {
                        if let Expression::Number(inner_b) = b.term {
                            let after = a.sub(&inner_b);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Add numbers", &before, &after);
                            }
                            result[i] = after;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // aX + bX => (a + b)X
                    // aX - bX => (a - b)X
                    // -aX + bX => (- a + b)X
                    // -aX - bX => -(a + b)X
                    (
                        Expression::Multiplication(lhs_mul),
                        Expression::Multiplication(rhs_mul),
                    ) => {
                        let (lhs_neg, lhs_coeff, lhs) =
                            transform_multiplication(lhs_mul.terms.to_vec());
                        let (rhs_neg, rhs_coeff, mut rhs) =
                            transform_multiplication(rhs_mul.terms.to_vec());

                        if Expression::compare_expression_vectors(&lhs, &rhs) {
                            rhs.push(match (lhs_coeff, rhs_coeff) {
                                // -aX - bX => -(a + b)X
                                (a, b) if rhs_neg && lhs_neg => {
                                    Expression::negation(Expression::integer(a + b))
                                }
                                // aX - bX => (a - b)X
                                (a, b) if rhs_neg => {
                                    if b > a {
                                        Expression::negation(Expression::integer(b - a))
                                    } else {
                                        Expression::integer(a - b)
                                    }
                                }
                                // -bX + aX => (a - b)X
                                (b, a) if lhs_neg => {
                                    if b > a {
                                        Expression::negation(Expression::integer(b - a))
                                    } else {
                                        Expression::integer(a - b)
                                    }
                                }
                                // aX + bX => (a + b)X
                                _ => Expression::integer(lhs_coeff + rhs_coeff),
                            });
                            let mut after = Expression::multiplication(rhs);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Add similar expression", &before, &after);
                            }
                            result[i] = after.simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1
                        }
                    }
                    // a - a => 0
                    // -a + a => 0
                    (a, Expression::Negation(b)) | (Expression::Negation(b), a)
                        if a.is_equal(&b.term) =>
                    {
                        result.swap_remove(j);
                        result.swap_remove(i);
                        if result.is_empty() {
                            let after = Expression::integer(0);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Add to zero", &before, &after);
                            }
                            result.push(after);
                            j += 1;
                        }
                    }

                    (a, Expression::Multiplication(mul)) => {
                        let reduced = Expression::reduce_add_mult(&mul.terms, a);
                        if let Some((terms_neg, expr_neg, expr, coeff)) = reduced {
                            // -a - Xa
                            let coeff_mult = if terms_neg && expr_neg {
                                Expression::negation(Expression::integer(coeff + 1))
                            }
                            // a - Xa
                            else if terms_neg {
                                if coeff == 1 {
                                    Expression::integer(0)
                                } else {
                                    Expression::negation(Expression::integer(coeff - 1))
                                }
                            }
                            // -a + Xa
                            else if expr_neg {
                                if coeff == 1 {
                                    Expression::integer(0)
                                } else {
                                    Expression::integer(coeff - 1)
                                }
                            }
                            // a + Xa
                            else {
                                Expression::integer(coeff + 1)
                            };
                            let mut after =
                                Expression::multiplication(vec![coeff_mult, expr.clone()]);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Add similar expression", &before, &after);
                            }
                            result[i] = after.simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    (Expression::Multiplication(mul), a) => {
                        let reduced = Expression::reduce_add_mult(&mul.terms, a);
                        if let Some((terms_neg, expr_neg, expr, coeff)) = reduced {
                            // -Xa - a
                            let coeff_mult = if terms_neg && expr_neg {
                                Expression::negation(Expression::integer(coeff + 1))
                            }
                            // -Xa + a
                            else if terms_neg {
                                if coeff == 1 {
                                    Expression::integer(0)
                                } else {
                                    Expression::negation(Expression::integer(coeff - 1))
                                }
                            }
                            // Xa - a
                            else if expr_neg {
                                if coeff == 1 {
                                    Expression::integer(0)
                                } else {
                                    Expression::integer(coeff - 1)
                                }
                            }
                            // Xa + a
                            else {
                                Expression::integer(coeff + 1)
                            };
                            let mut after =
                                Expression::multiplication(vec![coeff_mult, expr.clone()]);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Add similar expression", &before, &after);
                            }
                            result[i] = after.simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // a + bi + c + di => (a + c) + (b + d)i
                    (Expression::Complex(lhs), Expression::Complex(rhs)) => {
                        let mut after = Expression::complex(
                            Expression::addition(vec![lhs.real.clone(), rhs.real.clone()]),
                            Expression::addition(vec![lhs.imag.clone(), rhs.imag.clone()]),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add complex expression", &before, &after);
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + c + di => (a + c) + di
                    (a, Expression::Complex(comp)) | (Expression::Complex(comp), a) => {
                        let mut after = if comp.real.is_equal(&Expression::integer(0)) {
                            Expression::complex(a.clone(), comp.imag.clone())
                        } else {
                            Expression::complex(
                                Expression::addition(vec![a.clone(), comp.real.clone()]),
                                comp.imag.clone(),
                            )
                        };
                        // if let Some(explanation) = explanation {
                        //     explanation.rule_applied(
                        //         "Add with a complex expression",
                        //         &before,
                        //         &after,
                        //     );
                        // }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                        if j >= result.len() && j > i + 1 {
                            j -= 1;
                        }
                    }
                    // a/b + c/d => (ad + bc)/(bd)
                    (Expression::Division(lhs), Expression::Division(rhs)) => {
                        let mut after = Expression::division(
                            Expression::addition(vec![
                                Expression::multiplication(vec![lhs.num.clone(), rhs.den.clone()]),
                                Expression::multiplication(vec![lhs.den.clone(), rhs.num.clone()]),
                            ]),
                            Expression::multiplication(vec![lhs.den.clone(), rhs.den.clone()]),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add two fraction", &before, &after);
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + c/d => (ad + c)/(d)
                    (a, Expression::Division(div)) | (Expression::Division(div), a) => {
                        let mut after = Expression::division(
                            Expression::addition(vec![a.clone(), div.num.clone()]),
                            div.den.clone(),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add with a fraction", &before, &after);
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    _ => j += 1,
                }
            }
            i += 1;
        }

        Ok(if result.len() == 1 {
            let res = result.pop().unwrap();
            if let Some(explanation) = explanation {
                explanation.simplify_step_completed(&res);
            }
            res
        } else {
            let result = Expression::addition(result);
            if let Some(explanation) = explanation {
                explanation.simplify_step_completed(&result);
            }
            result
        })
    }
}
