use crate::{explanation::FormattingObserver, utils};

use super::{Expression, SimplifyError, function, numeral};

impl Expression {
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
                Expression::Function(function::Function::Sqrt, args),
                Expression::Number(numeral::Numeral::Integer(2)),
            ) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Square root to the 2th power cancel", &before, &args[0]);
                }
                Ok(args[0].clone())
            }
            // root(x, a)^x => a
            (
                Expression::Function(function::Function::Root, args),
                Expression::Number(numeral::Numeral::Integer(x)),
            ) if args[0] == Expression::Number(numeral::Numeral::Integer(x)) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied("nth root to nth power cancel", &before, &Expression::integer(x));
                }
                Ok(args[1].clone())
            }
            // (a^b)^c => a^(b*c)
            (Expression::Exponentiation(base, exp), rhs) => {
                    if let Some(explanation) = explanation {
                        explanation.rule_applied("Multiply the exponent", &before, &Expression::Exponentiation(
                            base.clone(),
                            Box::new(Expression::Multiplication(vec![*exp.clone(), rhs.clone()])),
                        ));
                    }
                    Expression::Exponentiation(
                        base,
                        Box::new(Expression::Multiplication(vec![*exp, rhs])),
                    )
                    .simplify(explanation)
            }
            // (a*b)^c => a^c*b^c
            (Expression::Multiplication(terms), rhs) => {
                let mut after: Expression = Expression::Multiplication( 
                    terms
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
                let mut after = utils::multinomial_expansion(&add, n);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Use the multinomial theoerm", &before, &after);
                };
                after.simplify(explanation)
            }
            // a^b => a^b
            (lhs, rhs) => Ok(Expression::Exponentiation(Box::new(lhs), Box::new(rhs))),
        };

        result
    }
}
