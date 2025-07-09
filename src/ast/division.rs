use crate::{
    ast::{complex::Complex, numeral, Expr, Expression, SimplifyError},
    explanation::FormattingObserver,
};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Division {
    pub num: Expression,
    pub den: Expression,
    pub simplified: bool,
}

// Constructor
impl Division {
    pub fn new(num: Expression, den: Expression, simplified: bool) -> Self {
        Self {
            num,
            den,
            simplified,
        }
    }
}

impl Expr for Division {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let num = self.num.simplify(explanation)?;
        let den = self.den.simplify(explanation)?;
        self.simplify_division(num, den, explanation)
    }

    fn is_equal(&self, other: &Division) -> bool {
        self.num.is_equal(&other.num) && self.den.is_equal(&other.den)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.num.contains_var(variable) || self.den.contains_var(variable)
    }
}

impl std::fmt::Display for Division {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}",
            if self.num.is_single() {
                format!("{}", self.num)
            } else {
                format!("({})", self.num)
            },
            if self.den.is_single() {
                format!("{}", self.den)
            } else {
                format!("({})", self.den)
            }
        )
    }
}

impl Division {
    fn simplify_division(
        &mut self,
        lhs: Expression,
        rhs: Expression,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::division(lhs.clone(), rhs.clone());
        let result = match (lhs, rhs) {
            // a/0 => DivisionByZero
            (_, Expression::Number(numeral::Numeral::Integer(0))) => {
                Err(SimplifyError::DivisionByZero)
            }
            // a/1 => a
            (lhs, Expression::Number(numeral::Numeral::Integer(1))) => {
                let after = lhs;
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Division by one stay the same", &before, &after);
                }

                Ok(after)
            }
            // 0/a => 0
            (Expression::Number(numeral::Numeral::Integer(0)), _) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        "Zero divided by something stay zero",
                        &before,
                        &Expression::integer(0),
                    );
                }
                Ok(Expression::integer(0))
            }
            // a/a => 1
            (lhs, rhs) if lhs.is_equal(&rhs) => {
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        &format!("Simplify by the common factor {}", lhs),
                        &before,
                        &Expression::integer(1),
                    );
                }
                Ok(Expression::integer(1))
            }
            // a/b where a & b are numeral
            (Expression::Number(lhs), Expression::Number(rhs)) => {
                Expression::Number(lhs.div(&rhs)).simplify(explanation)
            }
            // a/(b/c) => (a*c)/b
            (lhs, Expression::Division(rhs)) => {
                let mut after = Expression::division(
                    Expression::multiplication(vec![lhs, rhs.den.clone()]),
                    rhs.num,
                );
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        "Dividing by a fraction is given by\na/(b/c) => (a*c)/b",
                        &before,
                        &after,
                    );
                }
                after.simplify(explanation)
            }
            // (a/b)/c => a/(b*c)
            (Expression::Division(lhs), rhs) => {
                let mut after =
                    Expression::division(lhs.num, Expression::multiplication(vec![lhs.den, rhs]));
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        "A fraction divided by something is given by\n(a/b)/c => a/(b*c)",
                        &before,
                        &after,
                    );
                }
                after.simplify(explanation)
            }
            // (-a)/b => -(a/b)
            (Expression::Negation(inner), rhs) => {
                let mut after = Expression::negation(Expression::division(inner.term, rhs));
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        "We take the negation out of the division",
                        &before,
                        &after,
                    );
                }
                after.simplify(explanation)
            }
            // a/(-b) => -(a/b)
            (lhs, Expression::Negation(inner)) => {
                let mut after = Expression::negation(Expression::division(lhs, inner.term));
                if let Some(explanation) = explanation {
                    explanation.rule_applied(
                        "We take the negation out of the division",
                        &before,
                        &after,
                    );
                }
                after.simplify(explanation)
            }
            // // a^x / a => a^(x-1)
            // (Expression::Exponentiation(base, exp), rhs) if base.is_equal(&rhs) => {
            //     rule = "using a^x / a => a^(x-1)";
            //     Expression::Exponentiation(
            //         base,
            //         Box::new(Expression::Addition(vec![
            //             *exp,
            //             Expression::Negation(Box::new(Expression::Number(
            //                 numeral::Numeral::Integer(1),
            //             ))),
            //         ])),
            //     )
            //     .simplify(explanation)
            // }
            // // a^x / a^y => a^(x-y)
            // (
            //     Expression::Exponentiation(lhs_base, lhs_exp),
            //     Expression::Exponentiation(rhs_base, rhs_exp),
            // ) if lhs_base.is_equal(&rhs_base) => {
            //     rule = "using a^x / a^y => a^(x-y)";
            //     Expression::Exponentiation(
            //         lhs_base,
            //         Box::new(Expression::Addition(vec![
            //             *lhs_exp,
            //             Expression::Negation(Box::new(*rhs_exp)),
            //         ])),
            //     )
            //     .simplify(explanation)
            // }
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
            (lhs, Expression::Complex(comp)) => {
                let conj = Expression::Complex(Box::new(Complex::new(comp.real.clone(), comp.imag.clone(), false).conjugate()));
                let mut after = Expression::division(
                    Expression::multiplication(vec![lhs, conj.clone()]),
                    Expression::multiplication(vec![
                        Expression::complex(comp.real.clone(), comp.imag.clone()),
                        conj,
                    ]),
                );
                if let Some(explanation) = explanation {
                    explanation.rule_applied("We take the complex out of the denominator by multiplying by the conjugate\nc/(a + b i) => (c*(a - b i))/((a + b i)(a - b i))", &before, &after);
                }
                after.simplify(explanation)
            }
            // Default case
            (lhs, rhs) => {
                //     Expression::Multiplication(vec![
                //     lhs,
                //     Expression::Exponentiation(
                //         Box::new(rhs),
                //         Box::new(Expression::Negation(Box::new(Expression::Number(
                //             numeral::Numeral::Integer(1),
                //         )))),
                //     ),
                // ])
                // .simplify(explanation)
                Ok(Expression::division(lhs, rhs))
            }
        };

        result
    }
}
