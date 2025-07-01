use crate::{explanation::{FormattingObserver, SimplificationObserver}, utils};

use super::{Expression, SimplifyError, function, numeral};

impl Expression {
    pub(crate) fn simplify_exponentiation(
        &mut self,
        lhs: Expression,
        rhs: Expression,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let mut rule = "";
        let result = match (lhs, rhs) {
            // 0^0 => ZeroExponentiationZero
            (
                Expression::Number(numeral::Numeral::Integer(0)),
                Expression::Number(numeral::Numeral::Integer(0)),
            ) => Err(SimplifyError::ZeroExponentiationZero),
            // a^0 => 1
            (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                rule = "using a^0 => 1";
                Ok(Expression::integer(1))
            }
            // 1^x
            (Expression::Number(numeral::Numeral::Integer(1)), Expression::Number(_))  => {
                rule = "using 1^x => 1";
                Ok(Expression::integer(1))
            }
            // (Expression::Number(numeral::Numeral::Integer(1)), Expression::Negation(x)) if *x == Expression::Number(_) => {
            //     rule = "using 1^(-x) => 1";
            //     Ok(Expression::integer(1))
            // }
            // a^1 => a
            (lhs, Expression::Number(numeral::Numeral::Integer(1))) => {
                rule = "using a^1 => a";
                Ok(lhs)
            }
            // sqrt(a)^2 => a
            (
                Expression::Function(function::Function::Sqrt, args),
                Expression::Number(numeral::Numeral::Integer(2)),
            ) => {
                rule = "using sqrt(a)^2 => a";
                Ok(args[0].clone())
            }
            // root(x, a)^x => a
            (
                Expression::Function(function::Function::Root, args),
                Expression::Number(numeral::Numeral::Integer(x)),
            ) if args[0] == Expression::Number(numeral::Numeral::Integer(x)) => {
                rule = "using root(x, a)^x => a";
                Ok(args[1].clone())
            }
            // (a^b)^c => a^(b*c)
            (Expression::Exponentiation(base, exp), rhs) => {
                rule = "using (a^b)^c => a^(b*c)";
                if let Expression::Number(numeral::Numeral::Integer(_)) = rhs {
                    Expression::Exponentiation(
                        base,
                        Box::new(Expression::Multiplication(vec![*exp, rhs])),
                    )
                    .simplify(explanation)
                } else if let Expression::Negation(neg) = rhs {
                    if let Expression::Number(numeral::Numeral::Integer(_)) = *neg {
                        Expression::Exponentiation(
                            base,
                            Box::new(Expression::Multiplication(vec![*exp, Expression::Negation(neg)])),
                        )
                        .simplify(explanation)
                    } else {
                        Ok(Expression::Exponentiation(
                            Box::new(Expression::Exponentiation(base, exp)),
                            Box::new(Expression::Negation(neg)),
                        ))
                    }
                } 
                else {
                    Ok(Expression::Exponentiation(
                        Box::new(Expression::Exponentiation(base, exp)),
                        Box::new(rhs),
                    ))
                }
            }
            // (a*b)^c => a^c*b^c
            (Expression::Multiplication(terms), rhs) => {
                rule = "using (a*b)^c => a^c*b^c";
                if let Expression::Number(numeral::Numeral::Integer(a)) = rhs {
                    let mut new_terms = vec![];
                    for term in terms {
                        new_terms.push(Expression::Exponentiation(
                            Box::new(term),
                            Box::new(Expression::integer(a)),
                        ));
                    }
                    Expression::Multiplication(new_terms).simplify(explanation)
                } else if let Expression::Negation(x) = rhs {
                    if let Expression::Number(numeral::Numeral::Integer(a)) = *x {
                        let mut new_terms = vec![];
                        for term in terms {
                            new_terms.push(Expression::Exponentiation(
                                Box::new(term),
                                Box::new(Expression::Negation(Box::new(Expression::integer(a)))),
                            ));
                        }
                        Expression::Multiplication(new_terms).simplify(explanation)
                    } else {
                        Ok(Expression::Exponentiation(
                            Box::new(Expression::Multiplication(terms)),
                            Box::new(Expression::Negation(x.clone())),
                        ))
                    }
                } else {
                    Ok(Expression::Exponentiation(
                        Box::new(Expression::Multiplication(terms)),
                        Box::new(rhs),
                    ))
                }
            }
            // (a + b)^n where n is a integer
            (Expression::Addition(add), Expression::Number(numeral::Numeral::Integer(n))) => {
                rule = "using (a + b)^n => multinomial expansion";
                utils::multinomial_expansion(&add, n).simplify(explanation)
            }
            // a^b => a^b
            (lhs, rhs) => Ok(Expression::Exponentiation(Box::new(lhs), Box::new(rhs))),
        };

        // if !rule.is_empty() {
        //     if let Some(explanation) = explanation {
        //         explanation.push(format!(
        //             "Simplifiyng Exponentiation {}",
        //             rule,
        //         ));
        //     }
        // }

        result
    }
}
