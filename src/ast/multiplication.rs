use crate::{ast::numeral::Numeral, explanation::FormattingObserver};

use super::{Expression, SimplifyError, numeral};

impl Expression {
    /// Simplify multiplication
    pub(crate) fn simplify_multiplication(
        &self,
        simplified_terms: Vec<Expression>,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let mut result = simplified_terms;
        let mut prod  = Numeral::Integer(1);
        let mut negative: bool = false;

        let mut i = 0;
        while i < result.len() {
            match &result[i] {
                Expression::Number(num) => {
                    prod = prod.mul(num);
                    result.swap_remove(i);
                }
                Expression::Multiplication(inner_terms) => {
                    result.extend(inner_terms.clone());
                    result.swap_remove(i);
                }
                Expression::Negation(expr) => {
                    negative = !negative;
                    result[i] = *expr.clone();
                }
                _ => i += 1,
            }
        }

        if prod.is_zero() {
            return Ok(Expression::integer(0));
        } else if !prod.is_one() {
            result.push(Expression::Number(prod));
        }

        let mut rule = "";

        let mut i: usize = 0;
        while i < result.len() {
            let mut j: usize = i + 1;
            while j < result.len() {
                match (&result[i], &result[j]) {
                    // a * 0 => 0
                    (Expression::Number(numeral::Numeral::Integer(0)), _)
                    | (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Multiplying by zero",
                                &Expression::Multiplication(result.clone()),
                                &Expression::integer(0)
                            );
                        }
                        return Ok(Expression::integer(0));
                    }
                    // a * mult => expand mult
                    (_, Expression::Multiplication(mult)) => {
                        result.extend(mult.clone());
                        result.swap_remove(j);
                    }
                    // a * a => a^2
                    (a, b) if a.is_equal(b) => {
                        rule = "using a * a => a^2";
                        result[i] = Expression::Exponentiation(
                            Box::new(result[i].clone()),
                            Box::new(Expression::Number(numeral::Numeral::Integer(2))),
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // 1 * a => a
                    (Expression::Number(numeral::Numeral::Integer(1)), a)
                    | (a, Expression::Number(numeral::Numeral::Integer(1))) => {
                        rule = "using 1 * a => a";
                        result[i] = a.clone();
                        result.swap_remove(j);
                    }
                    (Expression::Number(a), Expression::Number(b)) => {
                        rule = "using a * b where a and b are number";
                        result[i] = Expression::Number(a.mul(b)).simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // -a * -b => a * b
                    (Expression::Negation(a), Expression::Negation(b)) => {
                        rule = "using -a * -b => a * b";
                        let new_b = *b.clone();
                        result[i] = *a.clone();
                        result[j] = new_b;
                    }
                    // a * -b => -(a * b)
                    (_, Expression::Negation(b)) => {
                        rule = "using a * -b => -(a * b)";
                        negative = !negative;
                        result[j] = *b.clone();
                    }
                    (Expression::Negation(b), _) => {
                        rule = "using a * -b => -(a * b)";
                        negative = !negative;
                        result[i] = *b.clone();
                    }
                    // (a + b i)(c + d i) => ac - bd + ad i + bc i
                    (
                        Expression::Complex(lhs_real, lhs_imag),
                        Expression::Complex(rhs_real, rhs_imag),
                    ) => {
                        rule = "using (a + b i)(c + d i) => ac - bd + ad i + bc i";
                        result[i] = Expression::Complex(
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
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a(b + c i) => ab + ac i
                    (a, Expression::Complex(real, imag)) | (Expression::Complex(real, imag), a) => {
                        rule = "using a(b + c i) => ab + ac i";
                        result[i] = Expression::Complex(
                            Box::new(Expression::Multiplication(vec![a.clone(), *real.clone()])),
                            Box::new(Expression::Multiplication(vec![a.clone(), *imag.clone()])),
                        )
                        .simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // (a + b)(c + d) => ac + ad + bc + bd
                    (Expression::Addition(lhs), Expression::Addition(rhs)) => {
                        rule = "using (a + b)(c + d) => ac + ad + bc + bd";
                        let mut new_terms = vec![];
                        for l in lhs {
                            for r in rhs {
                                new_terms
                                    .push(Expression::Multiplication(vec![l.clone(), r.clone()]));
                            }
                        }
                        result[i] = Expression::Addition(new_terms).simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a(b + c) => ab + ac
                    (a, Expression::Addition(terms)) | (Expression::Addition(terms), a) => {
                        rule = "using a(b + c) => ab + ac";
                        let mut new_terms = vec![];
                        for term in terms {
                            new_terms
                                .push(Expression::Multiplication(vec![a.clone(), term.clone()]));
                        }
                        result[i] = Expression::Addition(new_terms).simplify(explanation)?;
                        result.swap_remove(j);
                    }
                    // a^x * a^y => a^(x + y)
                    (
                        Expression::Exponentiation(lhs_base, lhs_exp),
                        Expression::Exponentiation(rhs_base, rhs_exp),
                    ) => {
                        rule = "using a^x * a^y => a^(x + y)";
                        if lhs_base.is_equal(rhs_base) {
                            result[i] = Expression::Exponentiation(
                                lhs_base.clone(),
                                Box::new(Expression::Addition(vec![
                                    *lhs_exp.clone(),
                                    *rhs_exp.clone(),
                                ])),
                            )
                            .simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            j += 1;
                        }
                    }
                    // a^x * a => a^(x + 1)
                    (Expression::Exponentiation(base, exp), a)
                    | (a, Expression::Exponentiation(base, exp)) => {
                        rule = "using a^x * a => a^(x + 1)";
                        if base.is_equal(a) {
                            result[i] = Expression::Exponentiation(
                                base.clone(),
                                Box::new(Expression::Addition(vec![
                                    *exp.clone(),
                                    Expression::Number(numeral::Numeral::Integer(1)),
                                ])),
                            )
                            .simplify(explanation)?;
                            result.swap_remove(j);
                        } else {
                            rule = "";
                            j += 1;
                        }
                    }
                    _ => j += 1,
                }
                // if !rule.is_empty() {
                //     if let Some(explanation) = explanation {
                //         explanation.push(format!(
                //             "Simplifying Multiplication {}: {}",
                //             rule,
                //             Expression::Multiplication(result.clone())
                //         ));
                //     }
                //     rule = "";
                // }
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
            Expression::negation(sol).simplify(explanation)
        } else {
            Ok(sol)
        }
    }
}
