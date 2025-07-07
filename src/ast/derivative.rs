use std::vec;

use crate::explanation::FormattingObserver;

use super::{Expression, SimplifyError, constant::Constant, function::Function};

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
                explanation.open_explaination(format!("We take {order} derivative"));
            }
        }

        for i in 0..order {
            if let Some(explanation) = explanation {
                if order != 1 {
                    explanation.open_explaination(format!("{} derivative", i + 1));
                }
            }
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
                Ok(after)
            }
            Expression::Constant(_) => {
                let after = Expression::integer(0);
                if let Some(explanation) = explanation {
                    explanation.rule_applied("Derivative of a constant is zero", &before, &after);
                }
                Ok(after)
            }
            Expression::Variable(v) => {
                if v == variable {
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
            Expression::Negation(expr) => {
                if let Some(explanation) = explanation {
                    let after =
                        Expression::negation(Expression::derivative(*expr.clone(), variable, 1));
                    explanation.rule_applied(
                        "The negative constant is highlighted",
                        &before,
                        &after,
                    );
                }

                Ok(Expression::negation(
                    expr.differentiate(variable, explanation)?,
                ))
            }
            Expression::Addition(exprs) => {
                if let Some(explanation) = explanation {
                    let after = Expression::Addition(
                        exprs
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
                Ok(Expression::Addition(
                    exprs
                        .iter_mut()
                        .map(|expr| expr.differentiate(variable, explanation))
                        .collect::<Result<Vec<Expression>, _>>()?,
                ))
            }
            Expression::Subtraction(expr1, expr2) => {
                if let Some(explanation) = explanation {
                    let after = Expression::subtraction(
                        Expression::derivative(*expr1.clone(), variable, 1),
                        Expression::derivative(*expr2.clone(), variable, 1),
                    );
                    explanation.rule_applied(
                        "Derivative of sum is given by\n(f - g)' => f' - g'",
                        &before,
                        &after,
                    );
                }
                Ok(Expression::subtraction(
                    expr1.differentiate(variable, explanation)?,
                    expr2.differentiate(variable, explanation)?,
                ))
            }
            Expression::Multiplication(exprs) => {
                let mut first = exprs[0].clone();
                if exprs.len() > 1 {
                    if let Some(explanation) = explanation {
                        if let Some(rest_expr) = exprs.get(1..) {
                            let after = Expression::Addition(vec![
                                Expression::Multiplication(vec![
                                    first.clone(),
                                    Expression::derivative(
                                        Expression::Multiplication(rest_expr.to_vec()),
                                        variable,
                                        1,
                                    ),
                                ]),
                                Expression::Multiplication(vec![
                                    Expression::derivative(first.clone(), variable, 1),
                                    Expression::Multiplication(rest_expr.to_vec()),
                                ]),
                            ]);
                            explanation.rule_applied(
                                "Derivative of product is given by\n(f*g)' => f' * g + f * g'",
                                &before,
                                &after,
                            );
                        }
                    }
                    if let Some(rest_expr) = exprs.get(1..) {
                        let mut rest = Expression::Multiplication(rest_expr.to_vec());

                        let d_first = first.differentiate(variable, explanation)?;
                        let d_rest = rest.differentiate(variable, explanation)?;

                        Ok(Expression::Addition(vec![
                            Expression::Multiplication(vec![first, d_rest]),
                            Expression::Multiplication(vec![d_first, rest]),
                        ]))
                    } else {
                        first.differentiate(variable, explanation)
                    }
                } else {
                    first.differentiate(variable, explanation)
                }
            }
            Expression::Division(num, den) => {
                if let Some(explanation) = explanation {
                    let after = Expression::division(
                        Expression::subtraction(
                            Expression::Multiplication(vec![
                                Expression::derivative(*num.clone(), variable, 1),
                                *den.clone(),
                            ]),
                            Expression::Multiplication(vec![
                                *num.clone(),
                                Expression::derivative(*den.clone(), variable, 1),
                            ]),
                        ),
                        Expression::exponentiation(*den.clone(), Expression::integer(2)),
                    );
                    explanation.rule_applied(
                        "Derivative of product is given by\n(f/g)' => (f'*g - f*g')/g^2",
                        &before,
                        &after,
                    );
                }
                let df = num.differentiate(variable, explanation)?;
                let dg = den.differentiate(variable, explanation)?;

                Ok(Expression::division(
                    Expression::subtraction(
                        Expression::Multiplication(vec![df, *den.clone()]),
                        Expression::Multiplication(vec![*num.clone(), dg]),
                    ),
                    Expression::exponentiation(*den.clone(), Expression::integer(2)),
                ))
            }
            Expression::Exponentiation(base, exp) => {
                match (base.contains_var(variable), exp.contains_var(variable)) {
                    // f^g => e^(g*ln(f)) and after it should be => (g * ln(f))' * e^(g*ln(f)) => f^g * (g' * ln(f) + g * f'/f)
                    (true, true) => {
                        let after = Expression::derivative(
                            Expression::exponentiation(
                                Expression::Constant(Constant::E),
                                Expression::Multiplication(vec![
                                    *exp.clone(),
                                    Expression::ln(*base.clone()),
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
                        let after = Expression::Multiplication(vec![
                            *exp.clone(),
                            Expression::exponentiation(
                                *base.clone(),
                                Expression::subtraction(*exp.clone(), Expression::integer(1)),
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
                        if let Expression::Constant(Constant::E) = **base {
                            let after = 
                                Expression::Multiplication(vec![
                                    Expression::Exponentiation(base.clone(), exp.clone()),
                                    Expression::derivative(*exp.clone(), variable, 1),
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
                                Expression::Multiplication(vec![
                                    Expression::Exponentiation(base.clone(), exp.clone()),
                                    Expression::ln(*base.clone()),
                                    Expression::Derivative(
                                        Box::new(*exp.clone()),
                                        variable.to_string(),
                                        1,
                                    ),
                                ]),
                                variable,
                                1,
                            );
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Derivative of exponentiation is given by\n(a^f)' => e^(f*ln(a)) * f' * ln(a)", &before, &after);
                            }
                            Ok(Expression::Multiplication(vec![
                                Expression::Exponentiation(base.clone(), exp.clone()),
                                Expression::ln(*base.clone()),
                                Expression::derivative(*exp.clone(), variable, 1),
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
            Expression::Function(func, args) => {
                Self::differentiate_function(func, args, variable, explanation)
            }
            Expression::Equality(_lhs, _rhs) => Err(SimplifyError::Unsupported),
            Expression::Derivative(expr, variable, order) => {
                if let Some(explanation) = explanation {
                    let after = Expression::derivative(
                        Expression::derivative(*expr.clone(), variable, *order),
                        variable,
                        *order,
                    );
                    explanation.rule_applied(
                        "Derivative of a derivative is given by is f'' => (f')'",
                        &before,
                        &after,
                    );
                }
                let expr_diff = expr.differentiate(variable, explanation)?;
                Ok(Expression::derivative(expr_diff, variable, *order))
            }
            Expression::Complex(_lhs, _rhs) => {
                // TODO
                Err(SimplifyError::Unsupported)
            }
        }?;

        result.simplify(explanation)
    }

    fn differentiate_function(
        func: &Function,
        args: &Vec<Expression>,
        variable: &str,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        let before = Expression::derivative(
            Expression::Function(func.clone(), args.clone()),
            variable,
            1,
        );
        let mut result = match func {
            // sin(f) => f' * cos(f)
            Function::Sin => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using sin(f)' => f' * cos(f)",
                        Expression::Multiplication(vec![
                            Expression::Function(Function::Cos, args.clone()),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using sin(x)' => cos(x)",
                        Expression::Function(Function::Cos, args.clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // cos(f) => -f' * sin(f)
            Function::Cos => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using cos(f)' => f' * -sin(f)",
                        Expression::Multiplication(vec![
                            Expression::Negation(Box::new(Expression::Function(
                                Function::Sin,
                                args.clone(),
                            ))),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using cos(x)' => -sin(x)",
                        Expression::Negation(Box::new(Expression::Function(
                            Function::Sin,
                            args.clone(),
                        ))),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // tan(f) => f' * 1/cos^2(f)
            Function::Tan => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using tan(f)' => f' * 1/cos^2(f)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Exponentiation(
                                    Box::new(Expression::Function(Function::Cos, args.clone())),
                                    Box::new(Expression::integer(2)),
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using tan(x)' => 1/cos^2(x)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Exponentiation(
                                Box::new(Expression::Function(Function::Cos, args.clone())),
                                Box::new(Expression::integer(2)),
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // asin(f) => f' * 1/sqrt(1-f^2)
            Function::Asin => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using asin(f)' => f' * 1/sqrt(1-f^2)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Function(
                                    Function::Sqrt,
                                    vec![Expression::Subtraction(
                                        Box::new(Expression::integer(1)),
                                        Box::new(Expression::Exponentiation(
                                            Box::new(args[0].clone()),
                                            Box::new(Expression::integer(2)),
                                        )),
                                    )],
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using asin(x)' => 1/sqrt(1-x^2)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Function(
                                Function::Sqrt,
                                vec![Expression::Subtraction(
                                    Box::new(Expression::integer(1)),
                                    Box::new(Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    )),
                                )],
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
            Function::Acos => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using acos(f)' => -f' * 1/sqrt(1-f^2)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Function(
                                    Function::Sqrt,
                                    vec![Expression::Subtraction(
                                        Box::new(Expression::integer(1)),
                                        Box::new(Expression::Exponentiation(
                                            Box::new(args[0].clone()),
                                            Box::new(Expression::integer(2)),
                                        )),
                                    )],
                                )),
                            ),
                            Expression::Negation(Box::new(expr)),
                        ]),
                    )
                } else {
                    (
                        "using acos(x)' => -1/sqrt(1-x^2)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Function(
                                Function::Sqrt,
                                vec![Expression::Subtraction(
                                    Box::new(Expression::integer(1)),
                                    Box::new(Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    )),
                                )],
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
            Function::Atan => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using atan(f)' => f' * 1/(1+f^2)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Addition(vec![
                                    Expression::integer(1),
                                    Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    ),
                                ])),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using atan(x)' => 1/(1+x^2)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Function(
                                Function::Sqrt,
                                vec![Expression::Addition(vec![
                                    Expression::integer(1),
                                    Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    ),
                                ])],
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // sinh(f) => f' * cosh(f)
            Function::Sinh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using sinh(f)' => f' * cosh(f)",
                        Expression::Multiplication(vec![
                            Expression::Function(Function::Cosh, args.clone()),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using sinh(x)' => cosh(x)",
                        Expression::Function(Function::Cosh, args.clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // cosh(f) => f' * sinh(f)
            Function::Cosh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using cosh(f)' => f' * sinh(f)",
                        Expression::Multiplication(vec![
                            Expression::Function(Function::Sinh, args.clone()),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using cosh(x)' => sinh(x)",
                        Expression::Function(Function::Sinh, args.clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // tanh(f) => f' * 1/cosh^2(f)
            Function::Tanh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using tanh(f)' => f' * 1/cosh^2(f)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Exponentiation(
                                    Box::new(Expression::Function(Function::Cosh, args.clone())),
                                    Box::new(Expression::integer(2)),
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using tanh(x)' => 1/cosh^2(x)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Exponentiation(
                                Box::new(Expression::Function(Function::Cosh, args.clone())),
                                Box::new(Expression::integer(2)),
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // asinh(f) => f' * 1/sqrt(1+f^2)
            Function::Asinh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using asinh(f)' => f' * 1/sqrt(1+f^2)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Function(
                                    Function::Sqrt,
                                    vec![Expression::Addition(vec![
                                        Expression::integer(1),
                                        Expression::Exponentiation(
                                            Box::new(args[0].clone()),
                                            Box::new(Expression::integer(2)),
                                        ),
                                    ])],
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using asinh(x)' => 1/sqrt(1+x^2)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Function(
                                Function::Sqrt,
                                vec![Expression::Addition(vec![
                                    Expression::integer(1),
                                    Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    ),
                                ])],
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // acosh(f) => f' * 1/sqrt(f^2-1)
            Function::Acosh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using acosh(f)' => f' * 1/sqrt(f^2-1)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Function(
                                    Function::Sqrt,
                                    vec![Expression::Subtraction(
                                        Box::new(Expression::Exponentiation(
                                            Box::new(args[0].clone()),
                                            Box::new(Expression::integer(2)),
                                        )),
                                        Box::new(Expression::integer(1)),
                                    )],
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using acosh(x)' => 1/sqrt(x^2-1)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Function(
                                Function::Sqrt,
                                vec![Expression::Subtraction(
                                    Box::new(Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    )),
                                    Box::new(Expression::integer(1)),
                                )],
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
            Function::Atanh => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using atanh(f)' => f' * 1/(1-f^2)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Subtraction(
                                    Box::new(Expression::integer(1)),
                                    Box::new(Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    )),
                                )),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using atanh(x)' => 1/(1-x^2)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Function(
                                Function::Sqrt,
                                vec![Expression::Subtraction(
                                    Box::new(Expression::integer(1)),
                                    Box::new(Expression::Exponentiation(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(2)),
                                    )),
                                )],
                            )),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // sqrt(f) => f' * 1/(2*sqrt(f))
            Function::Sqrt => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using sqrt(f)' => f' * 1/(2*sqrt(f))",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Multiplication(vec![
                                    Expression::Function(Function::Sqrt, vec![args[0].clone()]),
                                    Expression::integer(2),
                                ])),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using sqrt(x)' => 1/(2*sqrt(x))",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Multiplication(vec![
                                Expression::Function(Function::Sqrt, vec![args[0].clone()]),
                                Expression::integer(2),
                            ])),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // exp(f) => f' * exp(f)
            Function::Exp => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using exp(f)' => f' * exp(f)",
                        Expression::Multiplication(vec![
                            Expression::Function(Function::Exp, args.clone()),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using exp(x)' => exp(x)",
                        Expression::Function(Function::Exp, args.clone()),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // ln(f) => f' * 1/f
            Function::Ln => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using ln(f)' => f' * 1/f",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(args[0].clone()),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using ln(x)' => 1/x",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(args[0].clone()),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // log2(f) => f' * 1/(f*ln(2))
            Function::Log2 => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using log2(f)' => f' * 1/(f*ln(2))",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Multiplication(vec![
                                    args[0].clone(),
                                    Expression::Function(
                                        Function::Ln,
                                        vec![Expression::integer(2)],
                                    ),
                                ])),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using log2(x)' => 1/(x*ln(2))",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Multiplication(vec![
                                args[0].clone(),
                                Expression::Function(Function::Ln, vec![Expression::integer(2)]),
                            ])),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // log10(f) => f' * 1/(f*ln(10))
            Function::Log10 => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                    (
                        "using log10(f)' => f' * 1/(f*ln(10))",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Multiplication(vec![
                                    args[0].clone(),
                                    Expression::Function(
                                        Function::Ln,
                                        vec![Expression::integer(10)],
                                    ),
                                ])),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using log10(x)' => 1/(x*ln(10))",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Multiplication(vec![
                                args[0].clone(),
                                Expression::Function(Function::Ln, vec![Expression::integer(10)]),
                            ])),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // pow(o, f) => pow(o-1, f) * f'
            Function::Pow => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                    (
                        "using pow(o, f)' => pow(o-1, f) * f'",
                        Expression::Multiplication(vec![
                            Expression::Function(
                                Function::Pow,
                                vec![
                                    Expression::Subtraction(
                                        Box::new(args[0].clone()),
                                        Box::new(Expression::integer(1)),
                                    ),
                                    args[1].clone(),
                                ],
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using pow(o, x)' => pow(o-1, x)",
                        Expression::Function(
                            Function::Pow,
                            vec![
                                Expression::Subtraction(
                                    Box::new(args[0].clone()),
                                    Box::new(Expression::integer(1)),
                                ),
                                args[1].clone(),
                            ],
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // log(b, f) => f' * 1/(f*ln(b))
            Function::Log => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                    (
                        "using log(b, f)' => f' * 1/(f*ln(b))",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(Expression::Multiplication(vec![
                                    args[1].clone(),
                                    Expression::Function(Function::Ln, vec![args[0].clone()]),
                                ])),
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using log(b, x)' => 1/(x*ln(b))",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Multiplication(vec![
                                args[1].clone(),
                                Expression::Function(Function::Ln, vec![args[0].clone()]),
                            ])),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // root(o, f) => f' * 1/(pow(1/o - 1, f)*o)
            Function::Root => {
                let (rule, after) = if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                    (
                        "using root(o, f)' => f' * 1/(pow(1/o - 1, f)*o)",
                        Expression::Multiplication(vec![
                            Expression::Division(
                                Box::new(Expression::integer(1)),
                                Box::new(args[0].clone()),
                            ),
                            Expression::Function(
                                Function::Pow,
                                vec![
                                    Expression::Subtraction(
                                        Box::new(Expression::Division(
                                            Box::new(Expression::integer(1)),
                                            Box::new(args[0].clone()),
                                        )),
                                        Box::new(Expression::integer(1)),
                                    ),
                                    args[1].clone(),
                                ],
                            ),
                            expr,
                        ]),
                    )
                } else {
                    (
                        "using root(o, f)' => 1/(pow(1/o - 1, x) * o)",
                        Expression::Division(
                            Box::new(Expression::integer(1)),
                            Box::new(Expression::Multiplication(vec![
                                args[0].clone(),
                                Expression::Function(
                                    Function::Root,
                                    vec![
                                        Expression::Subtraction(
                                            Box::new(Expression::Division(
                                                Box::new(Expression::integer(1)),
                                                Box::new(args[0].clone()),
                                            )),
                                            Box::new(Expression::integer(1)),
                                        ),
                                        args[1].clone(),
                                    ],
                                ),
                            ])),
                        ),
                    )
                };
                if let Some(exp) = explanation {
                    exp.rule_applied(rule, &before, &after);
                }
                after
            }
            // Unsupported functions
            Function::Abs | Function::Ceil | Function::Floor => {
                return Err(SimplifyError::Unsupported);
            }
        };

        result.simplify(explanation)
    }

    fn quotient_rule(expr: &Expression, variable: &str) -> Option<Expression> {
        match expr.contains_var(variable) {
            false => None,
            true => Some(Expression::Derivative(
                Box::new(expr.clone()),
                variable.to_string(),
                1,
            )),
        }
    }
}
