use crate::{
    explanation::{FormattingObserver, SimplificationObserver},
    utils::transform_multiplication,
};

use super::{Expression, SimplifyError, numeral};

impl Expression {
    pub(crate) fn simplify_addition(
        &self,
        terms: Vec<Expression>,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::Addition(terms.clone());

        if let Some(explanation) = explanation {
            explanation.step_started(&before);
        }

        let mut result = vec![];
        let mut sum: u64 = 0;
        let mut neg: bool = false;

        for term in terms {
            match term {
                Expression::Number(numeral::Numeral::Integer(n)) => {
                    if neg && n > sum {
                        sum = n - sum;
                        neg = false;
                    } else if n == sum && neg {
                        sum = 0;
                        neg = false;
                    } else if neg {
                        sum -= n;
                    } else {
                        sum += n;
                    }
                }
                Expression::Negation(inner) => {
                    if let Expression::Number(numeral::Numeral::Integer(a)) = *inner {
                        if a > sum && !neg {
                            sum = a - sum;
                            neg = true;
                        } else if a == sum && !neg {
                            sum = 0;
                            neg = false;
                        } else if neg {
                            sum += a;
                        } else {
                            sum -= a;
                        }
                    } else {
                        result.push(Expression::Negation(inner))
                    }
                }
                Expression::Addition(inner_terms) => result.extend(inner_terms),
                _ => result.push(term),
            }
        }

        if sum != 0 {
            if neg {
                let after = Expression::negation(Expression::integer(sum));
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Add numbers ", &before, &after);
                }
                result.push(after)
            } else {
                let after = Expression::integer(sum);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Add numbers ", &before, &after);
                }
                result.push(after)
            }
        } else if result.is_empty() {
            if let Some(explanation) = explanation {
                let after = Expression::Number(numeral::Numeral::Integer(0));
                explanation.rule_applied("Add numbers to 0", &before, &after);
            }

            return Ok(Expression::integer(0));
        }

        let mut i: usize = 0;
        while i < result.len() {
            let mut j: usize = i + 1;
            while j < result.len() {
                let before = Expression::Multiplication(result.clone());
                match (&result[i], &result[j]) {
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
                        let mut after = Expression::Multiplication(vec![
                            Expression::Number(numeral::Numeral::Integer(2)),
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
                        if let Expression::Number(inner_b) = **b {
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
                        Expression::Multiplication(lhs_terms),
                        Expression::Multiplication(rhs_terms),
                    ) => {
                        let (lhs_neg, lhs_coeff, lhs) =
                            transform_multiplication(lhs_terms.to_vec());
                        let (rhs_neg, rhs_coeff, mut rhs) =
                            transform_multiplication(rhs_terms.to_vec());

                        if Self::compare_expression_vectors(&lhs, &rhs) {
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
                            let mut after = Expression::Multiplication(rhs);
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
                        if a.is_equal(b) =>
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

                    (a, Expression::Multiplication(terms)) => {
                        let reduced = Self::reduce_add_mult(terms, a);
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
                                Expression::Multiplication(vec![coeff_mult, expr.clone()]);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Add similar expression", &before, &after);
                            }
                            result[i] = after.simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    (Expression::Multiplication(terms), a) => {
                        let reduced = Self::reduce_add_mult(terms, a);
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
                                Expression::Multiplication(vec![coeff_mult, expr.clone()]);
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
                    (Expression::Complex(a, b), Expression::Complex(c, d)) => {
                        let mut after = Expression::Complex(
                            Box::new(Expression::Addition(vec![*a.clone(), *c.clone()])),
                            Box::new(Expression::Addition(vec![*b.clone(), *d.clone()])),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add complex expression", &before, &after);
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + c + di => (a + c) + di
                    (a, Expression::Complex(c, d)) | (Expression::Complex(c, d), a) => {
                        let mut after = Expression::Complex(
                            Box::new(Expression::Addition(vec![a.clone(), *c.clone()])),
                            d.clone(),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Add with a complex expression",
                                &before,
                                &after,
                            );
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a/b + c/d => (ad + bc)/(bd)
                    (Expression::Division(a, b), Expression::Division(c, d)) => {
                        let mut after = Expression::Division(
                            Box::new(Expression::Addition(vec![
                                Expression::Multiplication(vec![*a.clone(), *d.clone()]),
                                Expression::Multiplication(vec![*c.clone(), *b.clone()]),
                            ])),
                            Box::new(Expression::Multiplication(vec![*b.clone(), *d.clone()])),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Add two fraction", &before, &after);
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + c/d => (ad + c)/(d)
                    (a, Expression::Division(c, d)) | (Expression::Division(c, d), a) => {
                        let mut after = Expression::Division(
                            Box::new(Expression::Addition(vec![a.clone(), *c.clone()])),
                            d.clone(),
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
                explanation.step_completed(&res);
            }
            res
        } else {
            let result = Expression::Addition(result);
            if let Some(explanation) = explanation {
                explanation.step_completed(&result);
            }
            result
        })
    }

    /// Transform a `&[Expression]` representing an `Expression::Multiplication` and an Expression into a `Option<tuple>` representing 
    /// the Expression in common, if each one is negative and the coefficient between them
    /// 
    /// (a_negative, terms_negative, common_expr, coeff)
    fn reduce_add_mult<'b>(
        terms: &[Expression],
        a: &'b Expression,
    ) -> Option<(bool, bool, &'b Expression, u64)> {
        let mut coeff = 1;
        let mut terms_neg = false;

        let (expr_neg, expr): (bool, &Expression) = if let Expression::Negation(expr) = a {
            (true, expr)
        } else {
            (false, a)
        };

        let equal = terms.iter().all(|term| {
            if term.is_equal(expr) {
                true
            } else {
                match term {
                    Expression::Negation(inner) => {
                        if let Expression::Number(numeral::Numeral::Integer(b)) = **inner {
                            coeff *= b;
                            terms_neg = !terms_neg;
                            true
                        } else {
                            inner.is_equal(expr)
                        }
                    }
                    Expression::Number(numeral::Numeral::Integer(b)) => {
                        coeff *= b;
                        true
                    }
                    _ => false,
                }
            }
        });

        if equal {
            Some((terms_neg, expr_neg, expr, coeff))
        } else {
            None
        }
    }
}
