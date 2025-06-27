use std::vec;

use super::{constant::Constant, function::Function, Expression, SimplifyError};

impl Expression {
    pub fn differentiate_n(
        &self,
        variable: &str,
        order: u32,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
        if order == 0 {
            return Ok(self.clone());
        }

        let mut expr = self.clone();

        for _ in 0..order {
            expr = expr.differentiate(variable, explanation)?;
        }
        expr.simplify(explanation)
    }

    pub fn differentiate(
        &self,
        variable: &str,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
        let mut rule = "";
        let mut result =  match self {
            Expression::Number(_) => {
                rule = "using c' => 0";
                Expression::integer(0)},
            Expression::Constant(_) => {
                rule = "using c' => 0";
                Expression::integer(0)},
            Expression::Variable(v) => {
                if v == variable {
                    rule = "using x' => 1";
                    Expression::integer(1)
                } else {
                    rule = "using c' => 0";
                    Expression::integer(0)
                }
            }
            Expression::Negation(expr) => {
                rule = "using (-f)' => -(f')";
                Expression::Negation(Box::new(
                expr.differentiate(variable, explanation)?,
            ))},
            Expression::Addition(exprs) => {
                rule = "using (f+g)' => f' + g'";
                let mut derivatives = Vec::new();
                for expr in exprs {
                    derivatives.push(expr.differentiate(variable, explanation)?);
                }
                Expression::Addition(derivatives) 
            }
            Expression::Multiplication(exprs) => {
                rule = "using (f*g)' => f'*g + f*g'";
                let first = exprs[0].clone();
                if exprs.len() >= 2 {
                    if let Some(rest_expr) = exprs.get(1..) {
                        let rest = Expression::Multiplication(rest_expr.to_vec());
                        // let rest = Expression::Multiplication(vec![rest_expr.clone()]);
                        let d_first = first.differentiate(variable, explanation)?;
                        let d_rest = rest.differentiate(variable, explanation)?;

                        Expression::Addition(vec![
                            Expression::Multiplication(vec![d_first, rest]),
                            Expression::Multiplication(vec![first, d_rest]),
                        ])
                         
                    } else {
                        first.differentiate(variable, explanation)?
                    }
                } else {
                    first.differentiate(variable, explanation)?
                }
            }
            Expression::Subtraction(expr1, expr2) => {
                rule = "using (f-g)' => f' - g'";
                Expression::Subtraction(
                Box::new(expr1.differentiate(variable, explanation)?),
                Box::new(expr2.differentiate(variable, explanation)?),
            )
             },
            Expression::Division(num, den) => {
                rule = "using (f/g)' => (f'*g - f*g')/g^2";
                let df = num.differentiate(variable, explanation)?;
                let dg = den.differentiate(variable, explanation)?;

                Expression::Division(
                    Box::new(Expression::Subtraction(
                        Box::new(Expression::Multiplication(vec![df, *den.clone()])),
                        Box::new(Expression::Multiplication(vec![*num.clone(), dg])),
                    )),
                    Box::new(Expression::Exponentiation(
                        den.clone(),
                        Box::new(Expression::integer(2)),
                    )),
                )
                 
            }
            Expression::Exponentiation(base, exp) => {
                
                match (base.contains_var(variable), exp.contains_var(variable)) {
                    // f^g => e^(g*ln(f)) => (g * ln(f))' * e^(g*ln(f)) => f^g * (g' * ln(f) + g * f'/f)
                    (true, true) => {
                        rule = "using (f^g)' => (e^(g*ln(f)))' ";
                        Expression::Exponentiation(
                       Box::new(Expression::Constant(Constant::E)),
                       Box::new(Expression::Multiplication(vec![
                        *exp.clone(),
                        Expression::Function(Function::Ln, vec![*base.clone()]),
                       ]))
                    ) },
                    // f^a => a * f^(a-1)
                    (true, false) => {
                        rule = "using (f^a)' => a * f^(a-1)";
                        Expression::Multiplication(vec![
                        *exp.clone(),
                        Expression::Exponentiation(
                            Box::new(*base.clone()), 
                            Box::new(Expression::Subtraction(
                                exp.clone(),
                                Box::new(Expression::integer(1)))
                            ))]) }
                    ,
                    // a^f => e^(f*ln(a)) => (f * ln(a))' * e^(f*ln(a)) => f' * ln(a) * a^f
                    (false, true) => {
                        if let Expression::Constant(Constant::E) = **base {   
                        rule = "using (e^f)' => e^f * f'";                         
                            Expression::Multiplication(vec![
                        self.clone(),
                        Expression::Derivative(Box::new(*exp.clone()), variable.to_string(), 1),
                    ])
                     } else {
                        rule = "using (a^f)' => e^(f*ln(a)) * f' * ln(a)";
                        Expression::Multiplication(vec![
                            self.clone(),
                            Expression::Function(Function::Ln, vec![*base.clone()]),
                            Expression::Derivative(Box::new(*exp.clone()), variable.to_string(), 1),
                        ])
                         
                    }},
                    (false, false) => {
                        rule = "using (c^d)' => 1";
                        Expression::integer(1)},
                }
            }
            Expression::Function(func, args) => {
                Self::differentiate_function(func, args, variable, explanation)?
            }
            Expression::Equality(lhs, rhs) => {
                rule = "using (f=g)' => f' = g'";
                let lhs_diff = lhs.differentiate(variable, explanation)?;
                let rhs_diff = rhs.differentiate(variable, explanation)?;
                Expression::Equality(Box::new(lhs_diff), Box::new(rhs_diff)) 
            }
            Expression::Derivative(expr, variable, order) => {
                rule = "using f'' => (f')' ";
                let expr_diff = expr.differentiate(variable, explanation)?;
                Expression::Derivative(Box::new(expr_diff), variable.to_string(), *order)
                     
            }
            Expression::Complex(_lhs, _rhs) => {
                // TODO not handle yet
                self.clone()
            }
        };

        if let Some(explanation) = explanation {
            if !rule.is_empty() {
                explanation.push(rule.to_string());
            }
        }
        result.simplify(explanation)
    }

    fn differentiate_function(
        func: &Function,
        args: &Vec<Expression>,
        variable: &str,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
        let rule;
        let mut result = match func {
            // sin(f) => f' * cos(f)
            Function::Sin => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using sin(f)' => f' * cos(f)";
                Expression::Multiplication(vec![
                    Expression::Function(Function::Cos, args.clone()),
                    expr,
                ])
            } else {
                rule = "using sin(x)' => cos(x)";
                Expression::Function(Function::Cos, args.clone())
            }
             ,
            // cos(f) => -f' * sin(f)
            Function::Cos => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using cos(f)' => f' * -sin(f)";
                Expression::Multiplication(vec![
                    Expression::Negation(Box::new(Expression::Function(
                        Function::Sin,
                        args.clone(),
                    ))),
                    expr,
                ])
            } else {
                rule = "using cos(x)' => -sin(x)";
                Expression::Negation(Box::new(Expression::Function(Function::Sin, args.clone())))
            }
             ,
            // tan(f) => f' * 1/cos^2(f)
            Function::Tan => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using tan(f)' => f' * 1/cos^2(f)";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Exponentiation(
                            Box::new(Expression::Function(Function::Cos, args.clone())),
                            Box::new(Expression::integer(2)),
                        )),
                    ),
                    expr,
                ])
            } else {
                rule = "using tan(x)' => 1/cos^2(x)";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Exponentiation(
                        Box::new(Expression::Function(Function::Cos, args.clone())),
                        Box::new(Expression::integer(2)),
                    )),
                )
            }
             ,
            // asin(f) => f' * 1/sqrt(1-f^2)
            Function::Asin => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using asin(f)' => f' * 1/sqrt(1-f^2)";
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
                ])
            } else {
                rule = "using asin(x)' => 1/sqrt(1-x^2)";
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
                )
            }
             ,
            // acos(f) => -f' * 1/sqrt(1-f^2)
            Function::Acos => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using acos(f)' => -f' * 1/sqrt(1-f^2)";
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
                ])
            } else {
                rule = "using acos(x)' => -1/sqrt(1-x^2)";
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
                )
            }
             ,
            // atan(f) => f' * 1/(1+f^2)
            Function::Atan => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using atan(f)' => f' * 1/(1+f^2)";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Addition(vec![
                                Expression::integer(1),
                                Expression::Exponentiation(
                                    Box::new(args[0].clone()),
                                    Box::new(Expression::integer(2)),
                                ),
                            ],
                        )),
                    ),
                    expr,
                ])
            } else {
                rule = "using atan(x)' => 1/(1+x^2)";
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
                )
            }
             ,
            // sinh(f) => f' * cosh(f)
            Function::Sinh => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using sinh(f)' => f' * cosh(f)";
                Expression::Multiplication(vec![
                    Expression::Function(Function::Cosh, args.clone()),
                    expr,
                ])
            } else {
                rule = "using sinh(x)' => cosh(x)";
                Expression::Function(Function::Cosh, args.clone())
            }
             ,
            // cosh(f) => f' * sinh(f)
            Function::Cosh => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using cosh(f)' => f' * sinh(f)";
                Expression::Multiplication(vec![
                    Expression::Function(Function::Sinh, args.clone()),
                    expr,
                ])
            } else {
                rule = "using cosh(x)' => sinh(x)";
                Expression::Function(Function::Sinh, args.clone())
            }
             ,
            // tanh(f) => f' * 1/cosh^2(f)
            Function::Tanh => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using tanh(f)' => f' * 1/cosh^2(f)";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Exponentiation(
                            Box::new(Expression::Function(Function::Cosh, args.clone())),
                            Box::new(Expression::integer(2)),
                        )),
                    ),
                    expr,
                ])
            } else {
                rule = "using tanh(x)' => 1/cosh^2(x)";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Exponentiation(
                        Box::new(Expression::Function(Function::Cosh, args.clone())),
                        Box::new(Expression::integer(2)),
                    )),
                )
            }
             ,
            // asinh(f) => f' * 1/sqrt(1+f^2)
            Function::Asinh => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using asinh(f)' => f' * 1/sqrt(1+f^2)";
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
                ])
            } else {
                rule = "using asinh(x)' => 1/sqrt(1+x^2)";
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
                )
            }
             ,
            // acosh(f) => f' * 1/sqrt(f^2-1)
            Function::Acosh => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using acosh(f)' => f' * 1/sqrt(f^2-1)";
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
                ])
            } else {
                rule = "using acosh(x)' => 1/sqrt(x^2-1)";
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
                )
            }
             ,
            // atanh(f) => f' * 1/(1-f^2)
            Function::Atanh => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using atanh(f)' => f' * 1/(1-f^2)";
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
                ])
            } else {
                rule = "using atanh(x)' => 1/(1-x^2)";
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
                )
            }
             ,
            // sqrt(f) => f' * 1/(2*sqrt(f))
            Function::Sqrt => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using sqrt(f)' => f' * 1/(2*sqrt(f))";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Multiplication(vec![
                            Expression::Function(Function::Sqrt, vec![args[0].clone()]),
                            Expression::integer(2),
                        ])),
                    ),
                    expr,
                ])
            } else {
                rule = "using sqrt(x)' => 1/(2*sqrt(x))";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Multiplication(vec![
                        Expression::Function(Function::Sqrt, vec![args[0].clone()]),
                        Expression::integer(2),
                    ])),
                )
            }
             ,
            // exp(f) => f' * exp(f)
            Function::Exp => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using exp(f)' => f' * exp(f)";
                Expression::Multiplication(vec![
                    Expression::Function(Function::Exp, args.clone()),
                    expr,
                ])
            } else {   
                rule = "using exp(x)' => exp(x)";
                Expression::Function(Function::Exp, args.clone())
            }
             ,
            // ln(f) => f' * 1/f
            Function::Ln => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using ln(f)' => f' * 1/f";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(args[0].clone()),
                    ),
                    expr,
                ])
            } else {
                rule = "using ln(x)' => 1/x";
                Expression::Division(Box::new(Expression::integer(1)), Box::new(args[0].clone()))
            }
             ,
            // log2(f) => f' * 1/(f*ln(2))
            Function::Log2 => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using log2(f)' => f' * 1/(f*ln(2))";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Multiplication(vec![
                            args[0].clone(),
                            Expression::Function(Function::Ln, vec![Expression::integer(2)]),
                        ])),
                    ),
                    expr,
                ])
            } else {
                rule = "using log2(x)' => 1/(x*ln(2))";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Multiplication(vec![
                        args[0].clone(),
                        Expression::Function(Function::Ln, vec![Expression::integer(2)]),
                    ])),
                )
            }
             ,
            // log10(f) => f' * 1/(f*ln(10))
            Function::Log10 => if let Some(expr) = Self::quotient_rule(&args[0], variable) {
                rule = "using log10(f)' => f' * 1/(f*ln(10))";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Multiplication(vec![
                            args[0].clone(),
                            Expression::Function(Function::Ln, vec![Expression::integer(10)]),
                        ])),
                    ),
                    expr,
                ])
            } else {
                rule = "using log10(x)' => 1/(x*ln(10))";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Multiplication(vec![
                        args[0].clone(),
                        Expression::Function(Function::Ln, vec![Expression::integer(10)]),
                    ])),
                )
            }
             ,
            // pow(o, f) => pow(o-1, f) * f'
            Function::Pow => if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                rule = "using pow(o, f)' => pow(o-1, f) * f'";
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
                ])
            } else {
                rule = "using pow(o, x)' => pow(o-1, x)";
                Expression::Function(
                    Function::Pow,
                    vec![
                        Expression::Subtraction(
                            Box::new(args[0].clone()),
                            Box::new(Expression::integer(1)),
                        ),
                        args[1].clone(),
                    ],
                )
            }
             ,
            // log(b, f) => f' * 1/(f*ln(b))
            Function::Log => if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                rule = "using log(b, f)' => f' * 1/(f*ln(b))";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(Expression::Multiplication(vec![
                            args[1].clone(),
                            Expression::Function(Function::Ln, vec![args[0].clone()]),
                        ])),
                    ),
                    expr,
                ])
            } else {
                rule = "using log(b, x)' => 1/(x*ln(b))";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Multiplication(vec![
                        args[1].clone(),
                        Expression::Function(Function::Ln, vec![args[0].clone()]),
                    ])),
                )
            }
            ,
            // root(o, f) => f' * 1/(pow(1/o - 1, f)*o)
            Function::Root => if let Some(expr) = Self::quotient_rule(&args[1], variable) {
                rule = "using root(o, f) => f' * 1/(pow(1/o - 1, f)*o)";
                Expression::Multiplication(vec![
                    Expression::Division(
                        Box::new(Expression::integer(1)),
                        Box::new(                            args[0].clone(),
                            
                        ),
                    ),
                    Expression::Function(
                        Function::Pow,
                        vec![
                            Expression::Subtraction(
                                Box::new(
                                    Expression::Division(Box::new(Expression::integer(1)), Box::new(args[0].clone()))),
                                Box::new(Expression::integer(1)),
                            ),
                            args[1].clone(),
                        ],
                    ),
                    expr,
                ])
            } else {
                rule = "using root(o, f) => 1/(pow(1/o - 1, x) * o)";
                Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::Multiplication(vec![
                        args[0].clone(),
                        Expression::Function(
                            Function::Root,
                            vec![
                                Expression::Subtraction(
                                    Box::new(
                                        Expression::Division(Box::new(Expression::integer(1)), Box::new(args[0].clone()))),
                                    Box::new(Expression::integer(1)),
                                ),
                                args[1].clone(),
                            ],
                        ),
                    ])),
                )
            }
             ,
            Function::Abs | Function::Ceil | Function::Floor => return Err(SimplifyError::Unsupported),
        };

        if let Some(explanation) = explanation {
            if !rule.is_empty() {
                explanation.push(rule.to_string());
            }
        }

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
