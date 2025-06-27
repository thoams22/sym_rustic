use super::{numeral, Expression, SimplifyError};

impl Expression {
    pub(crate) fn simplify_addition(
        &self,
        terms: Vec<Expression>,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
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
                result.push(Expression::Negation(Box::new(Expression::Number(
                    numeral::Numeral::Integer(sum),
                ))))
            } else {
                result.push(Expression::Number(numeral::Numeral::Integer(sum)));
            }
        } else if result.is_empty() {
            if let Some(explanation) = explanation {
                explanation.push("Simplified Addition: 0".to_string());
            }

            return Ok(Expression::integer(0));
        }

        let mut i: usize = 0;
        while i < result.len() {
            let mut j: usize = i + 1;
            while j < result.len() {
                match (&result[i], &result[j]) {
                    // a + a => 2a
                    (a, b) if a.is_equal(b) => {
                        result[i] = Expression::Multiplication(vec![
                            Expression::Number(numeral::Numeral::Integer(2)),
                            result[i].clone(),
                        ])
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + b => c
                    (Expression::Number(a), Expression::Number(b)) => {
                        result[i] = Expression::Number(a.add(b));
                        result.swap_remove(j);
                    }
                    // a - b => c
                    (Expression::Number(a), Expression::Negation(b)) => {
                        if let Expression::Number(inner_b) = **b {
                            result[i] = a.sub(&inner_b);
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // -a + b => c
                    (Expression::Negation(a), Expression::Number(b)) => {
                        if let Expression::Number(inner_a) = **a {
                            result[i] = b.sub(&inner_a);
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // Xa + Ya => (X + Y)a
                    (
                        Expression::Multiplication(lhs_terms),
                        Expression::Multiplication(rhs_terms),
                    ) => {
                        let mut lhs: Vec<Expression> = vec![];
                        let mut rhs: Vec<Expression> = vec![];
                        let mut lhs_coeff = 1;
                        let mut rhs_coeff = 1;
                        let mut lhs_neg = false;
                        let mut rhs_neg = false;

                        lhs_terms
                            .iter()
                            .zip(rhs_terms.iter())
                            .for_each(|(lhs_term, rhs_term)| {
                                match lhs_term {
                                    Expression::Negation(inner) => {
                                        if let Expression::Number(numeral::Numeral::Integer(a)) =
                                            **inner
                                        {
                                            lhs_coeff *= a;
                                            lhs_neg = !lhs_neg;
                                        } else {
                                            lhs.push(lhs_term.clone());
                                        }
                                    }
                                    Expression::Number(numeral::Numeral::Integer(a)) => {
                                        lhs_coeff *= a;
                                    }
                                    _ => {
                                        lhs.push(lhs_term.clone());
                                    }
                                }
                                match rhs_term {
                                    Expression::Negation(inner) => {
                                        if let Expression::Number(numeral::Numeral::Integer(a)) =
                                            **inner
                                        {
                                            rhs_coeff *= a;
                                            rhs_neg = !rhs_neg;
                                        } else {
                                            rhs.push(rhs_term.clone());
                                        }
                                    }
                                    Expression::Number(numeral::Numeral::Integer(a)) => {
                                        rhs_coeff *= a;
                                    }
                                    _ => {
                                        rhs.push(rhs_term.clone());
                                    }
                                }
                            });
                        if  Self::compare_expression_vectors(&lhs, &rhs)
                        {
                            rhs.push(match (lhs_coeff, rhs_coeff) {
                                (a, b) if rhs_neg && lhs_neg => Expression::Negation(Box::new(
                                    Expression::Number(numeral::Numeral::Integer(a + b)),
                                )),
                                (a, b) if rhs_neg => {
                                    if b > a {
                                        Expression::Negation(Box::new(Expression::Number(
                                            numeral::Numeral::Integer(a + b),
                                        )))
                                    } else {
                                        Expression::Number(numeral::Numeral::Integer(a + b))
                                    }
                                }
                                (a, b) if lhs_neg => {
                                    if a > b {
                                        Expression::Negation(Box::new(Expression::Number(
                                            numeral::Numeral::Integer(a + b),
                                        )))
                                    } else {
                                        Expression::Number(numeral::Numeral::Integer(a + b))
                                    }
                                }
                                _ => Expression::Number(numeral::Numeral::Integer(
                                    lhs_coeff + rhs_coeff,
                                )),
                            });
                            result[i] = Expression::Multiplication(rhs).simplify(explanation)?;
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
                            result.push(Expression::Number(numeral::Numeral::Integer(0)));
                            j += 1;
                        }
                    }

                    (a, Expression::Multiplication(terms)) => {
                        let (terms_neg, expr_neg, expr, coeff, equal) =
                            Self::reduce_add_mult(terms, a);
                        if equal {
                            // -a - Xa
                            let coeff_mult = if terms_neg && expr_neg {
                                Expression::Negation(Box::new(Expression::Number(
                                    numeral::Numeral::Integer(coeff + 1),
                                )))
                            }
                            // -a + Xa
                            // a - Xa
                            else if expr_neg || terms_neg {
                                if coeff == 1 {
                                    Expression::Number(numeral::Numeral::Integer(0))
                                } else {
                                    Expression::Negation(Box::new(Expression::Number(
                                        numeral::Numeral::Integer(coeff - 1),
                                    )))
                                }
                            }
                            // a - Xa
                            // else if terms_neg {
                            //     if coeff == 1 {
                            //         Expression::Number(numeral::Numeral::Integer(0))
                            //     } else {
                            //         Expression::Negation(Box::new(Expression::Number(
                            //             numeral::Numeral::Integer(coeff - 1),
                            //         )))
                            //     }
                            // }
                            // a + Xa
                            else {
                                Expression::Number(numeral::Numeral::Integer(coeff + 1))
                            };
                            result[i] = Expression::Multiplication(vec![coeff_mult, expr.clone()])
                                .simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    (Expression::Multiplication(terms), a) => {
                        let (terms_neg, expr_neg, expr, coeff, equal) =
                            Self::reduce_add_mult(terms, a);
                        if equal {
                            // -Xa - a
                            let coeff_mult = if terms_neg && expr_neg {
                                Expression::Negation(Box::new(Expression::Number(
                                    numeral::Numeral::Integer(coeff + 1),
                                )))
                            }
                            // -Xa + a
                            else if terms_neg {
                                if coeff == 1 {
                                    Expression::Number(numeral::Numeral::Integer(0))
                                } else {
                                    Expression::Negation(Box::new(Expression::Number(
                                        numeral::Numeral::Integer(coeff - 1),
                                    )))
                                }
                            }
                            // Xa - a
                            else if expr_neg {
                                if coeff == 1 {
                                    Expression::Number(numeral::Numeral::Integer(0))
                                } else {
                                    Expression::Number(numeral::Numeral::Integer(coeff - 1))
                                }
                            }
                            // Xa + a
                            else {
                                Expression::Number(numeral::Numeral::Integer(coeff + 1))
                            };
                            result[i] = Expression::Multiplication(vec![coeff_mult, expr.clone()])
                                .simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // a + bi + c + di => (a + c) + (b + d)i
                    (Expression::Complex(a, b), Expression::Complex(c, d)) => {
                        result[i] = Expression::Complex(
                            Box::new(Expression::Addition(vec![*a.clone(), *c.clone()])),
                            Box::new(Expression::Addition(vec![*b.clone(), *d.clone()])),
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + c + di => (a + c) + di
                    (a, Expression::Complex(c, d)) | (Expression::Complex(c, d), a) => {
                        result[i] = Expression::Complex(
                            Box::new(Expression::Addition(vec![a.clone(), *c.clone()])),
                            d.clone(),
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a/b + c/d => (ad + bc)/(bd)
                    (Expression::Division(a, b), Expression::Division(c, d)) => {
                        result[i] = Expression::Division(
                            Box::new(Expression::Addition(vec![
                                Expression::Multiplication(vec![*a.clone(), *d.clone()]),
                                Expression::Multiplication(vec![*c.clone(), *b.clone()]),
                            ])),
                            Box::new(Expression::Multiplication(vec![*b.clone(), *d.clone()])),
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a + c/d => (ad + c)/(d)
                    (a, Expression::Division(c, d)) | (Expression::Division(c, d), a) => {
                        result[i] = Expression::Division(
                            Box::new(Expression::Addition(vec![a.clone(), *c.clone()])),
                            d.clone(),
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    _ => j += 1,
                }
            }
            i += 1;
        }

        if let Some(explanation) = explanation {
            explanation.push(format!(
                "Simplified Addition: {}",
                Expression::Addition(result.clone())
            ));
        }

        Ok(if result.len() == 1 {
            result.pop().unwrap()
        } else {
            Expression::Addition(result)
        })
    }

    fn reduce_add_mult<'b>(
        terms: &[Expression],
        a: &'b Expression,
    ) -> (bool, bool, &'b Expression, u64, bool) {
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
                        } else {inner.is_equal(expr)}
                    }
                    Expression::Number(numeral::Numeral::Integer(b)) => {
                        coeff *= b;
                        true
                    }
                    _ => false,
                }
            }
        });
        (terms_neg, expr_neg, expr, coeff, equal)
    }
}