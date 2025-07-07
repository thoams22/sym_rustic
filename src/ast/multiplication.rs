use crate::explanation::FormattingObserver;

use super::{Expression, SimplifyError, numeral};

impl Expression {
    /// Simplify multiplication
    pub(crate) fn simplify_multiplication(
        &self,
        simplified_terms: Vec<Expression>,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::Multiplication(simplified_terms.clone());

        if let Some(explanation) = explanation {
            explanation.step_started(&before);
        }

        let mut result = simplified_terms;
        let mut negative: bool = false;

        let mut i: usize = 0;
        while i < result.len() {
            let mut j: usize = i + 1;
            while j < result.len() {
                let before = Expression::Multiplication(result.clone());
                match (&result[i], &result[j]) {
                    // a * 0 => 0
                    (Expression::Number(numeral::Numeral::Integer(0)), _)
                    | (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiplying by zero yield zero",
                                &Expression::Multiplication(result.clone()),
                                &Expression::integer(0),
                            );
                        }
                        return Ok(Expression::integer(0));
                    }
                    // a * mult => expand mult
                    (_, Expression::Multiplication(mult)) => {
                        result.extend(mult.clone());
                        result.swap_remove(j);
                    }
                    (Expression::Multiplication(mult), _) => {
                        result.extend(mult.clone());
                        result.swap_remove(i);
                    }
                    // -a * -b => a * b
                    (Expression::Negation(a), Expression::Negation(b)) => {
                        if let Some(explanation) = explanation {
                            let after = Expression::Multiplication(vec![*a.clone(), *b.clone()]);
                            explanation.rule_applied(
                                "Multiply two negative expression cancel the negative",
                                &Expression::Multiplication(result.clone()),
                                &after,
                            );
                        };
                        let new_b = *b.clone();
                        result[i] = *a.clone();
                        result[j] = new_b;
                    }
                    // a * -b => -(a * b)
                    (_, Expression::Negation(b)) => {
                        negative = !negative;
                        result[j] = *b.clone();
                    }
                    // -a * b => -(a * b)
                    (Expression::Negation(b), _) => {
                        negative = !negative;
                        result[i] = *b.clone();
                    }
                    // 1 * a => a
                    (Expression::Number(numeral::Numeral::Integer(1)), a)
                    | (a, Expression::Number(numeral::Numeral::Integer(1))) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply by one stay the same",
                                &Expression::Multiplication(result.clone()),
                                &a,
                            );
                        }
                        result[i] = a.clone();
                        result.swap_remove(j);
                    }
                    (Expression::Number(a), Expression::Number(b)) => {
                        let after = Expression::Number(a.mul(b));
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply numbers",
                                &Expression::Multiplication(result.clone()),
                                &after,
                            );
                        };
                        result[i] = after;
                        result.swap_remove(j);
                    }
                    // a * a => a^2
                    (a, b) if a.is_equal(b) => {
                        let mut after = Expression::Exponentiation(
                            Box::new(result[i].clone()),
                            Box::new(Expression::Number(numeral::Numeral::Integer(2))),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply the same expression",
                                &Expression::Multiplication(result.clone()),
                                &after,
                            );
                        }
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // (a + b i)(c + d i) => ac - bd + ad i + bc i
                    (
                        Expression::Complex(lhs_real, lhs_imag),
                        Expression::Complex(rhs_real, rhs_imag),
                    ) => {
                        let mut after = Expression::Complex(
                            Box::new(Expression::Addition(vec![
                                Expression::Multiplication(vec![
                                    *lhs_real.clone(),
                                    *rhs_real.clone(),
                                ]),
                                Expression::Negation(Box::new(Expression::Multiplication(vec![
                                    *lhs_imag.clone(),
                                    *rhs_imag.clone(),
                                ]))),
                            ])),
                            Box::new(Expression::Addition(vec![
                                Expression::Multiplication(vec![
                                    *lhs_real.clone(),
                                    *rhs_imag.clone(),
                                ]),
                                Expression::Multiplication(vec![
                                    *lhs_imag.clone(),
                                    *rhs_real.clone(),
                                ]),
                            ])),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply two complex expression\n(a + b i)(c + d i) => ac - bd + i(ad + bc)",
                                &Expression::Multiplication(result.clone()),
                                &after
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a(b + c i) => ab + ac i
                    (a, Expression::Complex(real, imag)) | (Expression::Complex(real, imag), a) => {
                        let mut after = Expression::Complex(
                            Box::new(Expression::Multiplication(vec![a.clone(), *real.clone()])),
                            Box::new(Expression::Multiplication(vec![a.clone(), *imag.clone()])),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply with a complex expression\n(a + b i)(c + d i) => a(b + c i) => ab + iac",
                                &Expression::Multiplication(result.clone()),
                                &after
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // (a + b)(c + d) => ac + ad + bc + bd
                    (Expression::Addition(lhs), Expression::Addition(rhs)) => {
                        let mut after = Expression::Addition(
                            lhs.iter()
                                .flat_map(|l_term| {
                                    rhs.iter()
                                        .map(|r_term| {
                                            Expression::Multiplication(vec![
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
                                &Expression::Multiplication(result.clone()),
                                &after,
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a(b + c) => ab + ac
                    (a, Expression::Addition(terms)) | (Expression::Addition(terms), a) => {
                        let mut after = Expression::Addition(
                            terms
                                .iter()
                                .map(|term| {
                                    Expression::Multiplication(vec![term.clone(), a.clone()])
                                })
                                .collect(),
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiply by distributing each term",
                                &Expression::Multiplication(result.clone()),
                                &after,
                            );
                        };
                        result[i] = after.simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a^x * a^y => a^(x + y)
                    (
                        Expression::Exponentiation(lhs_base, lhs_exp),
                        Expression::Exponentiation(rhs_base, rhs_exp),
                    ) => {
                        if lhs_base.is_equal(rhs_base) {
                            let mut after = Expression::Exponentiation(
                                lhs_base.clone(),
                                Box::new(Expression::Addition(vec![
                                    *lhs_exp.clone(),
                                    *rhs_exp.clone(),
                                ])),
                            );
                            if let Some(explanation) = explanation {
                                explanation.rule_applied(
                                    "Multiply terms with the the same base by adding the exponents",
                                    &Expression::Multiplication(result.clone()),
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
                    (norm, Expression::Division(num, den))
                    | (Expression::Division(num, den), norm) => {
                        let mut after = Expression::division(
                            Expression::Multiplication(vec![norm.clone(), *num.clone()]),
                            *den.clone(),
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
                    (Expression::Exponentiation(base, exp), a)
                    | (a, Expression::Exponentiation(base, exp)) => {
                        if base.is_equal(a) {
                            let mut after = Expression::Exponentiation(
                                base.clone(),
                                Box::new(Expression::Addition(vec![
                                    *exp.clone(),
                                    Expression::Number(numeral::Numeral::Integer(1)),
                                ])),
                            );
                            if let Some(explanation) = explanation {
                                explanation.rule_applied(
                                    "Multiply terms with the the same base by adding the exponents",
                                    &Expression::Multiplication(result.clone()),
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
            Expression::Multiplication(result)
        };

        if negative {
            let mut result = Expression::negation(sol);
            if let Some(explanation) = explanation {
                explanation.step_completed(&result);
            }
            result.simplify(explanation)
        } else {
            if let Some(explanation) = explanation {
                explanation.step_completed(&sol);
            }
            Ok(sol)
        }
    }
}
