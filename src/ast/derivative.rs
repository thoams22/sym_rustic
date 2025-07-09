use std::vec;

use crate::{ast::Expr, explanation::FormattingObserver, prints::PrettyPrints};

use super::{Expression, SimplifyError, constant::Constant, function::FunctionType};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub struct Derivative {
    pub term: Expression,
    pub variable: String,
    pub order: u32,
    pub simplified: bool,
}

// Constructor
impl Derivative {
    pub fn new(term: Expression, variable: String, order: u32, simplified: bool) -> Self {
        Self {
            term,
            variable,
            order,
            simplified,
        }
    }
}

impl Expr for Derivative {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        self.term.simplify(explanation)?
            .differentiate_n(&self.variable, self.order, explanation)
    }

    fn is_equal(&self, other: &Derivative) -> bool {
        self.order == other.order
            && self.variable == other.variable
            && self.term.is_equal(&self.term)
    }

    fn contains_var(&self, variable: &str) -> bool {
        self.term.contains_var(variable)
    }

    fn is_single(&self) -> bool {
        false
    }
}

impl std::fmt::Display for Derivative {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "d{}/d{}{} {}",
            if self.order != 1 {
                format!("^{}", self.order)
            } else {
                "".to_string()
            },
            if self.variable.len() != 1 {
                format!("({})", self.variable)
            } else {
                self.variable.to_owned()
            },
            if self.order != 1 {
                format!("^{}", self.order)
            } else {
                "".to_string()
            },
            if self.term.is_single() {
                format!("{}", self.term)
            } else {
                format!("({})", self.term)
            }
        )
    }
}

impl Expression {
    pub fn differentiate_n(
        &self,
        variable: &str,
        order: u32,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        if order == 0 {
            return Ok(self.clone());
        }

        let mut expr = self.clone();

        if let Some(explanation) = explanation {
            if order != 1 {
                explanation.open_explaination(format!("We take {} derivative", order));
            }
        }

        for i in 0..order {
            if let Some(explanation) = explanation {
                if order != 1 {
                    explanation.open_explaination(format!("{} derivative", i + 1));
                }
            }
            dbg!(&expr);
            expr = expr.differentiate(variable, explanation)?;
        }
        expr.simplify(explanation)
    }

    pub fn differentiate(
        &mut self,
        variable: &str,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::derivative(self.clone(), variable, 1);
        let mut result = match self {
            Expression::Number(_) => {
                let after = Expression::integer(0);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Derivative of a number is zero", &before, &after);
                }
                return Ok(after)
            }
            Expression::Constant(_) => {
                let after = Expression::integer(0);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Derivative of a constant is zero", &before, &after);
                }
                return Ok(after)
            }
            Expression::Variable(var) => {
                return  if var.name == variable {
                    let after = Expression::integer(1);
                    if let Some(explanation) = explanation {
                        explanation.rule_applied(
                            "Derivative of the derivation variable is one",
                            &before,
                            &after,
                        );
                    }
                    Ok(after)
                } else {
                    let after = Expression::integer(0);
                    if let Some(explanation) = explanation {
                        explanation.rule_applied(
                            "Derivative of an arbitrary variable is zero",
                            &before,
                            &after,
                        );
                    }
                    Ok(after)
                }
            }
            Expression::Negation(neg) => {
                if let Some(explanation) = explanation {
                    let after = Expression::negation(Expression::derivative(
                        neg.term.clone(),
                        variable,
                        1,
                    ));
                    explanation.rule_applied(
                        "The negative constant is highlighted",
                        &before,
                        &after,
                    );
                }

                Ok(Expression::negation(
                    neg.term.differentiate(variable, explanation)?,
                ))
            }
            Expression::Addition(add) => {
                if let Some(explanation) = explanation {
                    let after = Expression::addition(
                        add.terms
                            .iter()
                            .map(|expr| Expression::derivative(expr.clone(), variable, 1))
                            .collect(),
                    );
                    explanation.rule_applied(
                        "Derivative of sum is given by\n(f + g)' => f' + g'",
                        &before,
                        &after,
                    );
                }
                Ok(Expression::addition(
                    add.terms
                        .iter_mut()
                        .map(|expr| expr.differentiate(variable, explanation))
                        .collect::<Result<Vec<Expression>, _>>()?,
                ))
            }
            Expression::Subtraction(sub) => {
                if let Some(explanation) = explanation {
                    let after = Expression::subtraction(
                        Expression::derivative(sub.left.clone(), variable, 1),
                        Expression::derivative(sub.right.clone(), variable, 1),
                    );
                    explanation.rule_applied(
                        "Derivative of sum is given by\n(f - g)' => f' - g'",
                        &before,
                        &after,
                    );
                }
                Ok(Expression::subtraction(
                    sub.left.differentiate(variable, explanation)?,
                    sub.right.differentiate(variable, explanation)?,
                ))
            }
            Expression::Multiplication(mul) => {
                let mut first = mul.terms[0].clone();
                if mul.terms.len() > 1 {
                    if let Some(explanation) = explanation {
                        if let Some(rest_expr) = mul.terms.get(1..) {
                            let after = Expression::addition(vec![
                                Expression::multiplication(vec![
                                    first.clone(),
                                    Expression::derivative(
                                        Expression::multiplication(rest_expr.to_vec()),
                                        variable,
                                        1,
                                    ),
                                ]),
                                Expression::multiplication(vec![
                                    Expression::derivative(first.clone(), variable, 1),
                                    Expression::multiplication(rest_expr.to_vec()),
                                ]),
                            ]);
                            explanation.rule_applied(
                                "Derivative of product is given by\n(f*g)' => f' * g + f * g'",
                                &before,
                                &after,
                            );
                        }
                    }
                    if let Some(rest_expr) = mul.terms.get(1..) {
                        let mut rest = Expression::multiplication(rest_expr.to_vec());

                        let d_first = first.differentiate(variable, explanation)?;
                        let d_rest = rest.differentiate(variable, explanation)?;

                        Ok(Expression::addition(vec![
                            Expression::multiplication(vec![first, d_rest]),
                            Expression::multiplication(vec![d_first, rest]),
                        ]))
                    } else {
                        first.differentiate(variable, explanation)
                    }
                } else {
                    first.differentiate(variable, explanation)
                }
            }
            Expression::Division(div) => {
                if let Some(explanation) = explanation {
                    let after = Expression::division(
                        Expression::subtraction(
                            Expression::multiplication(vec![
                                Expression::derivative(div.num.clone(), variable, 1),
                                div.den.clone(),
                            ]),
                            Expression::multiplication(vec![
                                div.num.clone(),
                                Expression::derivative(div.den.clone(), variable, 1),
                            ]),
                        ),
                        Expression::exponentiation(div.den.clone(), Expression::integer(2)),
                    );
                    explanation.rule_applied(
                        "Derivative of product is given by\n(f/g)' => (f'*g - f*g')/g^2",
                        &before,
                        &after,
                    );
                }
                let df = div.num.differentiate(variable, explanation)?;
                let dg = div.den.differentiate(variable, explanation)?;

                Ok(Expression::division(
                    Expression::subtraction(
                        Expression::multiplication(vec![df, div.den.clone()]),
                        Expression::multiplication(vec![div.num.clone(), dg]),
                    ),
                    Expression::exponentiation(div.den.clone(), Expression::integer(2)),
                ))
            }
            Expression::Exponentiation(exp) => {
                match (
                    exp.base.contains_var(variable),
                    exp.expo.contains_var(variable),
                ) {
                    // f^g => e^(g*ln(f)) and after it should be => (g * ln(f))' * e^(g*ln(f)) => f^g * (g' * ln(f) + g * f'/f)
                    (true, true) => {
                        let after = Expression::derivative(
                            Expression::exponentiation(
                                Expression::Constant(Constant::E),
                                Expression::multiplication(vec![
                                    exp.expo.clone(),
                                    Expression::ln(exp.base.clone()),
                                ]),
                            ),
                            variable,
                            1,
                        );
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Derivative of exponentiation is given by\n(f^g)' => (e^(g*ln(f)))'", &before, &after);
                        }
                        Ok(after)
                    }
                    // f^a => a * f^(a-1)
                    (true, false) => {
                        let after = Expression::multiplication(vec![
                            exp.expo.clone(),
                            Expression::exponentiation(
                                exp.base.clone(),
                                Expression::subtraction(exp.expo.clone(), Expression::integer(1)),
                            ),
                        ]);
                        if let Some(explanation) = explanation {
                            explanation.rule_applied(
                                "Derivative of exponentiation is given by\n(f^a)' => a * f^(a-1)'",
                                &before,
                                &after,
                            );
                        }
                        Ok(after)
                    }
                    // a^f => e^(f*ln(a)) => (f * ln(a))' * e^(f*ln(a)) => f' * ln(a) * a^f
                    (false, true) => {
                        if let Expression::Constant(Constant::E) = exp.base {
                            let after = Expression::multiplication(vec![
                                Expression::exponentiation(exp.base.clone(), exp.expo.clone()),
                                Expression::derivative(exp.expo.clone(), variable, 1),
                            ]);
                            if let Some(explanation) = explanation {
                                explanation.rule_applied(
                                    "Derivative of exponentiation is given by\n(e^f)' => e^f * f'",
                                    &before,
                                    &after,
                                );
                            }
                            Ok(after)
                        } else {
                            let after = Expression::derivative(
                                Expression::multiplication(vec![
                                    Expression::exponentiation(exp.base.clone(), exp.expo.clone()),
                                    Expression::ln(exp.base.clone()),
                                    Expression::derivative(exp.expo.clone(), variable, 1),
                                ]),
                                variable,
                                1,
                            );
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Derivative of exponentiation is given by\n(a^f)' => e^(f*ln(a)) * f' * ln(a)", &before, &after);
                            }
                            Ok(Expression::multiplication(vec![
                                Expression::exponentiation(exp.base.clone(), exp.expo.clone()),
                                Expression::ln(exp.base.clone()),
                                Expression::derivative(exp.expo.clone(), variable, 1),
                            ]))
                        }
                    }
                    (false, false) => {
                        if let Some(explanation) = explanation {
                            let after = Expression::integer(0);
                            explanation.rule_applied(
                                "Derivative of a constant is zero",
                                &before,
                                &after,
                            );
                        }
                        Ok(Expression::integer(0))
                    }
                }
            }
            Expression::Function(fun) => {
                Self::differentiate_function(&fun.name, &fun.args, variable, explanation)
            }
            Expression::Equality(_equ) => Err(SimplifyError::Unsupported),
            Expression::Derivative(der) => {
                if let Some(explanation) = explanation {
                    let after = Expression::derivative(
                        Expression::derivative(der.term.clone(), variable, der.order),
                        variable,
                        der.order,
                    );
                    explanation.rule_applied(
                        "Derivative of a derivative is given by is f'' => (f')'",
                        &before,
                        &after,
                    );
                }
                let expr_diff = der.term.differentiate(variable, explanation)?;
                Ok(Expression::derivative(expr_diff, variable, der.order))
            }
            Expression::Complex(_com) => {
                // TODO
                Err(SimplifyError::Unsupported)
            }
        }?;

        result.simplify(explanation)
    }

    fn differentiate_function(
        func: &FunctionType,
        args: &[Expression],
        variable: &str,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::derivative(
            Expression::function(func.clone(), args.to_owned()),
            variable,
            1,
        );
        let mut result = match func {
            // sin(f) => f' * cos(f)
            FunctionType::Sin => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using sin(f)' => f' * cos(f)",
                        Expression::multiplication(vec![Expression::cos(args[0].clone()), expr]),
                    )
                } else {
                    ("using sin(x)' => cos(x)", Expression::cos(args[0].clone()))
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // cos(f) => -f' * sin(f)
            FunctionType::Cos => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using cos(f)' => f' * -sin(f)",
                        Expression::multiplication(vec![
                            Expression::negation(Expression::sin(args[0].clone())),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using cos(x)' => -sin(x)",
                        Expression::negation(Expression::sin(args[0].clone())),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // tan(f) => f' * 1/cos^2(f)
            FunctionType::Tan => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using tan(f)' => f' * 1/cos^2(f)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::exponentiation(
                                    Expression::cos(args[0].clone()),
                                    Expression::integer(2),
                                ),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using tan(x)' => 1/cos^2(x)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::exponentiation(
                                Expression::cos(args[0].clone()),
                                Expression::integer(2),
                            ),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // asin(f) => f' * 1/sqrt(1-f^2)
            FunctionType::Asin => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using asin(f)' => f' * 1/sqrt(1-f^2)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::sqrt(Expression::subtraction(
                                    Expression::integer(1),
                                    Expression::exponentiation(
                                        args[0].clone(),
                                        Expression::integer(2),
                                    ),
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using asin(x)' => 1/sqrt(1-x^2)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::sqrt(Expression::subtraction(
                                Expression::integer(1),
                                Expression::exponentiation(args[0].clone(), Expression::integer(2)),
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // acos(f) => -f' * 1/sqrt(1-f^2)
            FunctionType::Acos => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using acos(f)' => -f' * 1/sqrt(1-f^2)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::sqrt(Expression::subtraction(
                                    Expression::integer(1),
                                    Expression::exponentiation(
                                        args[0].clone(),
                                        Expression::integer(2),
                                    ),
                                )),
                            ),
                            Expression::negation(expr),
                        ]),
                    )
                } else {
                    (
                        "using acos(x)' => -1/sqrt(1-x^2)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::sqrt(Expression::subtraction(
                                Expression::integer(1),
                                Expression::exponentiation(args[0].clone(), Expression::integer(2)),
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // atan(f) => f' * 1/(1+f^2)
            FunctionType::Atan => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using atan(f)' => f' * 1/(1+f^2)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::addition(vec![
                                    Expression::integer(1),
                                    Expression::exponentiation(
                                        args[0].clone(),
                                        Expression::integer(2),
                                    ),
                                ]),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using atan(x)' => 1/(1+x^2)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::addition(vec![
                                Expression::integer(1),
                                Expression::exponentiation(args[0].clone(), Expression::integer(2)),
                            ]),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // sinh(f) => f' * cosh(f)
            FunctionType::Sinh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using sinh(f)' => f' * cosh(f)",
                        Expression::multiplication(vec![Expression::cosh(args[0].clone()), expr]),
                    )
                } else {
                    (
                        "using sinh(x)' => cosh(x)",
                        Expression::cosh(args[0].clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // cosh(f) => f' * sinh(f)
            FunctionType::Cosh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using cosh(f)' => f' * sinh(f)",
                        Expression::multiplication(vec![Expression::sinh(args[0].clone()), expr]),
                    )
                } else {
                    (
                        "using cosh(x)' => sinh(x)",
                        Expression::sinh(args[0].clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // tanh(f) => f' * 1/cosh^2(f)
            FunctionType::Tanh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using tanh(f)' => f' * 1/cosh^2(f)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::exponentiation(
                                    Expression::cosh(args[0].clone()),
                                    Expression::integer(2),
                                ),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using tanh(x)' => 1/cosh^2(x)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::exponentiation(
                                Expression::cosh(args[0].clone()),
                                Expression::integer(2),
                            ),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // asinh(f) => f' * 1/sqrt(1+f^2)
            FunctionType::Asinh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using asinh(f)' => f' * 1/sqrt(1+f^2)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::sqrt(Expression::addition(vec![
                                    Expression::integer(1),
                                    Expression::exponentiation(
                                        args[0].clone(),
                                        Expression::integer(2),
                                    ),
                                ])),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using asinh(x)' => 1/sqrt(1+x^2)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::sqrt(Expression::addition(vec![
                                Expression::integer(1),
                                Expression::exponentiation(args[0].clone(), Expression::integer(2)),
                            ])),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // acosh(f) => f' * 1/sqrt(f^2-1)
            FunctionType::Acosh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using acosh(f)' => f' * 1/sqrt(f^2-1)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::sqrt(Expression::subtraction(
                                    Expression::exponentiation(
                                        args[0].clone(),
                                        Expression::integer(2),
                                    ),
                                    Expression::integer(1),
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using acosh(x)' => 1/sqrt(x^2-1)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::sqrt(Expression::subtraction(
                                Expression::exponentiation(args[0].clone(), Expression::integer(2)),
                                Expression::integer(1),
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // atanh(f) => f' * 1/(1-f^2)
            FunctionType::Atanh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using atanh(f)' => f' * 1/(1-f^2)",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::subtraction(
                                    Expression::integer(1),
                                    Expression::exponentiation(
                                        args[0].clone(),
                                        Expression::integer(2),
                                    ),
                                ),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using atanh(x)' => 1/(1-x^2)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::subtraction(
                                Expression::integer(1),
                                Expression::exponentiation(args[0].clone(), Expression::integer(2)),
                            ),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // sqrt(f) => f' * 1/(2*sqrt(f))
            FunctionType::Sqrt => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using sqrt(f)' => f' * 1/(2*sqrt(f))",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::multiplication(vec![
                                    Expression::sqrt(args[0].clone()),
                                    Expression::integer(2),
                                ]),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using sqrt(x)' => 1/(2*sqrt(x))",
                        Expression::division(
                            Expression::integer(1),
                            Expression::multiplication(vec![
                                Expression::sqrt(args[0].clone()),
                                Expression::integer(2),
                            ]),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // exp(f) => f' * exp(f)
            FunctionType::Exp => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using exp(f)' => f' * exp(f)",
                        Expression::multiplication(vec![Expression::exp(args[0].clone()), expr]),
                    )
                } else {
                    ("using exp(x)' => exp(x)", Expression::exp(args[0].clone()))
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // ln(f) => f' * 1/f
            FunctionType::Ln => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using ln(f)' => f' * 1/f",
                        Expression::multiplication(vec![
                            Expression::division(Expression::integer(1), args[0].clone()),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using ln(x)' => 1/x",
                        Expression::division(Expression::integer(1), args[0].clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // log2(f) => f' * 1/(f*ln(2))
            FunctionType::Log2 => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using log2(f)' => f' * 1/(f*ln(2))",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::multiplication(vec![
                                    args[0].clone(),
                                    Expression::ln(Expression::integer(2)),
                                ]),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using log2(x)' => 1/(x*ln(2))",
                        Expression::division(
                            Expression::integer(1),
                            Expression::multiplication(vec![
                                args[0].clone(),
                                Expression::ln(Expression::integer(2)),
                            ]),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // log10(f) => f' * 1/(f*ln(10))
            FunctionType::Log10 => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using log10(f)' => f' * 1/(f*ln(10))",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::multiplication(vec![
                                    args[0].clone(),
                                    Expression::ln(Expression::integer(10)),
                                ]),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using log10(x)' => 1/(x*ln(10))",
                        Expression::division(
                            Expression::integer(1),
                            Expression::multiplication(vec![
                                args[0].clone(),
                                Expression::ln(Expression::integer(10)),
                            ]),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // pow(o, f) => pow(o-1, f) * f'
            FunctionType::Pow => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                    (
                        "using pow(o, f)' => pow(o-1, f) * f'",
                        Expression::multiplication(vec![
                            Expression::pow(
                                Expression::subtraction(args[0].clone(), Expression::integer(1)),
                                args[1].clone(),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using pow(o, x)' => pow(o-1, x)",
                        Expression::pow(
                            Expression::subtraction(args[0].clone(), Expression::integer(1)),
                            args[1].clone(),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // log(b, f) => f' * 1/(f*ln(b))
            FunctionType::Log => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                    (
                        "using log(b, f)' => f' * 1/(f*ln(b))",
                        Expression::multiplication(vec![
                            Expression::division(
                                Expression::integer(1),
                                Expression::multiplication(vec![
                                    args[1].clone(),
                                    Expression::ln(args[0].clone()),
                                ]),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using log(b, x)' => 1/(x*ln(b))",
                        Expression::division(
                            Expression::integer(1),
                            Expression::multiplication(vec![
                                args[1].clone(),
                                Expression::ln(args[0].clone()),
                            ]),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // root(o, f) => f' * 1/(pow(1/o - 1, f)*o)
            FunctionType::Root => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                    (
                        "using root(o, f)' => f' * 1/(pow(1/o - 1, f)*o)",
                        Expression::multiplication(vec![
                            Expression::division(Expression::integer(1), args[0].clone()),
                            Expression::pow(
                                Expression::subtraction(
                                    Expression::division(Expression::integer(1), args[0].clone()),
                                    Expression::integer(1),
                                ),
                                args[1].clone(),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using root(o, f)' => 1/(pow(1/o - 1, x) * o)",
                        Expression::division(
                            Expression::integer(1),
                            Expression::multiplication(vec![
                                args[0].clone(),
                                Expression::root(
                                    Expression::subtraction(
                                        Expression::division(
                                            Expression::integer(1),
                                            args[0].clone(),
                                        ),
                                        Expression::integer(1),
                                    ),
                                    args[1].clone(),
                                ),
                            ]),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // Unsupported functions
            FunctionType::Abs | FunctionType::Ceil | FunctionType::Floor => {
                return Err(SimplifyError::Unsupported);
            }
        };

        result.simplify(explanation)
    }

    fn quotient_rule(expr: &Expression, variable: &str) -> Option<Expression> {
        match expr.contains_var(variable) {
            false => None,
            true => Some(Expression::derivative(expr.clone(), variable, 1)),
        }
    }
}

impl PrettyPrints for Derivative {
    fn calculate_tree(&self, indent: usize) -> String {
        let next_indent = indent + 2;
        let next_indent_str = " ".repeat(next_indent);

        format!(
            "Derivative{}:\n{}{}\n{}' {}",
            if self.order > 1 {
                format!("({})", self.order)
            } else {
                "".to_owned()
            },
            next_indent_str,
            self.term.calculate_tree(next_indent),
            next_indent_str,
            self.variable,
        )
    }

    fn calculate_positions(
        &self,
        memoization: &mut std::collections::HashMap<Expression, (usize, usize)>,
        position: &mut Vec<(String, (usize, usize))>,
        prev_pos: (usize, usize),
    ) {
        let length = self.variable.len()
        + 2
        + if self.order == 1 {
            0
        } else {
            self.order.to_string().len()
        };

    let below_height = self.get_below_height(memoization);

    let span = self.variable.len() / 2;

    let mut pos = prev_pos;

    let new_height = pos.0 + below_height;
    pos.0 = new_height - (1 + if self.order == 1 { 0 } else { 1 });

    // d
    position.push(("d".to_string(), pos));
    pos.1 += 2;
    // d var
    for (i, c) in self.variable.chars().enumerate() {
        position.push((c.to_string(), (pos.0, pos.1 + i)));
    }
    if self.order != 1 {
        //     order
        // d var
        pos.1 += self.variable.len();
        pos.0 += 1;
        for (i, c) in self.order.to_string().chars().enumerate() {
            position.push((c.to_string(), (pos.0, pos.1 + i)));
        }
        pos.1 -= self.variable.len();
    }
    pos.1 -= 2;

    pos.0 += 1;
    // ---------
    //     order
    // d var
    for _ in 0..length {
        position.push(("-".to_string(), pos));
        pos.1 += 1;
    }

    pos.1 -= length;
    pos.0 += 1;
    //    d
    // ---------
    //     order
    // d var
    pos.1 += span;
    position.push(("d".to_string(), pos));
    if self.order != 1 {
        //    order
        //   d
        // ----------
        //      order
        // d var
        pos.1 += 1;
        pos.0 += 1;
        for (i, c) in self.order.to_string().chars().enumerate() {
            position.push((c.to_string(), (pos.0, pos.1 + i)));
        }
        pos.1 -= 1;
        pos.0 -= 1;
    }
    pos.1 -= span;
    //    order
    //   d
    // ----------  expr
    //      order
    // d var
    let height = new_height - self.term.get_below_height(memoization);
    self.term
        .calculate_positions(memoization, position, (height, pos.1 + length + 1));
    }

    fn get_below_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self
                .term
                .get_below_height(memoization)
                .max(1 + if self.order == 1 { 0 } else { 1 })
    }

    fn get_height(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        let der_below = 1 + if self.order == 1 { 0 } else { 1 };
                let expr_below = self.term.get_below_height(memoization);

                let top = (3 + if self.order == 1 { 0 } else { 2 } - der_below)
                    .max(self.term.get_height(memoization) - expr_below);
                if der_below > expr_below {
                    der_below + top
                } else {
                    expr_below + top
                }
    }

    fn get_length(&self, memoization: &mut std::collections::HashMap<Expression, (usize, usize)>) -> usize {
        self.term.get_length(memoization)
                    + self.variable.len()
                    + 3
                    + if self.order == 1 {
                        0
                    } else {
                        self.order.to_string().len()
                    }
    }
}