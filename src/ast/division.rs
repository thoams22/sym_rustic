use crate::ast::{Expression, SimplifyError, numeral};

impl Expression {
    pub fn simplify_division(
        &mut self,
        lhs: Expression,
        rhs: Expression,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
        let mut rule = "";
        let result = match (lhs, rhs) {
            // a/0 => DivisionByZero
            (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                Err(SimplifyError::DivisionByZero)
            }
            // a/1 => a
            (lhs, Expression::Number(numeral::Numeral::Integer(1))) => Ok(lhs),
            // 0/a => 0
            (Expression::Number(numeral::Numeral::Integer(0)), _) => Ok(Expression::integer(0)),
            // a/a => 1
            (lhs, rhs) if lhs.is_equal(&rhs) => Ok(Expression::integer(1)),
            (
                Expression::Number(numeral::Numeral::Integer(a)),
                Expression::Number(numeral::Numeral::Integer(b)),
            ) => Ok(Expression::integer(a / b)),
            // a/(b/c) => (a*c)/b
            (lhs, Expression::Division(rhs1, rhs2)) => Expression::Division(
                Box::new(Expression::Multiplication(vec![lhs, *rhs2.clone()])),
                rhs1,
            )
            .simplify(explanation),
            // (a/b)/c => a/(b*c)
            (Expression::Division(lhs1, lhs2), rhs) => {
                rule = "using (a/b)/c => a/(b*c)";
                Expression::Division(lhs1, Box::new(Expression::Multiplication(vec![*lhs2, rhs])))
                    .simplify(explanation)
            }
            // (-a)/b => -(a/b)
            (Expression::Negation(inner), rhs) => {
                rule = "using (-a)/b => -(a/b)";
                Expression::Negation(Box::new(Expression::Division(
                    Box::new(*inner),
                    Box::new(rhs),
                )))
                .simplify(explanation)
            }
            // a/(-b) => -(a/b)
            (lhs, Expression::Negation(inner)) => {
                rule = "using a/(-b) => -(a/b)";
                Expression::Negation(Box::new(Expression::Division(
                    Box::new(lhs),
                    Box::new(*inner),
                )))
                .simplify(explanation)
            }
            // a^x / a^y => a^(x-y)
            (
                Expression::Exponentiation(lhs_base, lhs_exp),
                Expression::Exponentiation(rhs_base, rhs_exp),
            ) if lhs_base.is_equal(&rhs_base) => {
                rule = "using a^x / a^y => a^(x-y)";
                Expression::Exponentiation(
                    lhs_base,
                    Box::new(Expression::Addition(vec![
                        *lhs_exp,
                        Expression::Negation(Box::new(*rhs_exp)),
                    ])),
                )
                .simplify(explanation)
            }
            // a^x / a => a^(x-1)
            (Expression::Exponentiation(base, exp), rhs) if base.is_equal(&rhs) => {
                rule = "using a^x / a => a^(x-1)";
                Expression::Exponentiation(
                    base,
                    Box::new(Expression::Addition(vec![
                        *exp,
                        Expression::Negation(Box::new(Expression::Number(
                            numeral::Numeral::Integer(1),
                        ))),
                    ])),
                )
                .simplify(explanation)
            }
            // // a/sqrt(b) => a*sqrt(b)/b
            // (a, Expression::Function(function::Function::Sqrt, args)) => {
            //     rule = "using a/sqrt(b) => a*sqrt(b)/b";
            //     Expression::Division(
            //         Box::new(Expression::Multiplication(vec![
            //             a,
            //             Expression::Function(function::Function::Sqrt, args.clone()),
            //         ])),
            //         Box::new(args[0].clone()),
            //     )
            //     .simplify(explanation)
            // }
            // c/complex(a, b) => (c*complex(a, b))/(complex(a, b)*complcomplex(a, b))
            (lhs, Expression::Complex(real, imag)) => {
                rule = "using c/(a + b i) => (c*(a + b i))/((a + b i)(a - b i))";
                Expression::Division(
                    Box::new(Expression::Multiplication(vec![
                        lhs,
                        Expression::Complex(real.clone(), imag.clone()),
                    ])),
                    Box::new(Expression::Multiplication(vec![
                        Expression::Complex(real.clone(), imag.clone()),
                        Self::complex_conjugate(Expression::Complex(real, imag)).unwrap(),
                    ])),
                )
                .simplify(explanation)
            }
            // Default case
            (lhs, rhs) => Expression::Multiplication(vec![
                lhs,
                Expression::Exponentiation(
                    Box::new(rhs),
                    Box::new(Expression::Negation(Box::new(Expression::Number(
                        numeral::Numeral::Integer(1),
                    )))),
                ),
            ])
            .simplify(explanation),
        };

        if let Some(explanation) = explanation {
            explanation.push(format!(
                "Simplifying Division {}: {}",
                rule,
                result.clone()?
            ));
        }

        result
    }
}
