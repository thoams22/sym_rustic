use function::Function;

use crate::{
    ast::{constant::Constant, numeral::Numeral},
    explanation::FormattingObserver,
};

mod addition;
mod complex;
pub mod constant;
mod derivative;
mod division;
mod equality;
mod exponentiation;
pub mod function;
mod multiplication;
mod negation;
pub mod numeral;

#[derive(Debug, PartialEq, Clone)]
pub enum SimplifyError {
    DivisionByZero,
    ZeroExponentiationZero,
    InvalidDerivative,
    Unsupported,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Expression {
    // Unary
    Negation(Box<Expression>),
    Number(numeral::Numeral),
    Variable(String),
    Constant(Constant),
    // Binary
    Subtraction(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Exponentiation(Box<Expression>, Box<Expression>),
    Equality(Box<Expression>, Box<Expression>),
    Complex(Box<Expression>, Box<Expression>),
    // Multinary
    Addition(Vec<Expression>),
    Multiplication(Vec<Expression>),
    // Function
    Function(function::Function, Vec<Expression>),
    // Calculus
    Derivative(Box<Expression>, String, u32),
    // Integral(Box<Expression>, String),
    // Limit

    // Series
    // Summation
    // Product
    // Matrix
    // Vector

    // Logic
    // And
    // Or
    // Not
    // Xor
    // Binary
    // Hexadecimal
}

// Helper constructor for some expression.
impl Expression {
    pub fn integer(n: u64) -> Expression {
        Expression::Number(numeral::Numeral::Integer(n))
    }

    pub fn rational(n: u64, d: u64) -> Expression {
        Expression::Number(numeral::Numeral::Rational(n, d))
    }

    pub fn subtraction(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Subtraction(Box::new(lhs), Box::new(rhs))
    }

    pub fn negation(arg: Expression) -> Expression {
        Expression::Negation(Box::new(arg))
    }

    pub fn equality(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Equality(Box::new(lhs), Box::new(rhs))
    }

    pub fn complex(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Complex(Box::new(lhs), Box::new(rhs))
    }

    pub fn variable(name: &str) -> Expression {
        Expression::Variable(name.to_string())
    }

    pub fn exponentiation(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Exponentiation(Box::new(lhs), Box::new(rhs))
    }

    pub fn division(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Division(Box::new(lhs), Box::new(rhs))
    }

    pub fn derivative(expr: Expression, variable: &str, order: u32) -> Expression {
        Expression::Derivative(Box::new(expr), variable.to_string(), order)
    }

    pub fn sin(arg: Expression) -> Expression {
        Expression::Function(function::Function::Sin, vec![arg])
    }

    pub fn cos(arg: Expression) -> Expression {
        Expression::Function(function::Function::Cos, vec![arg])
    }

    pub fn tan(arg: Expression) -> Expression {
        Expression::Function(function::Function::Tan, vec![arg])
    }

    pub fn asin(arg: Expression) -> Expression {
        Expression::Function(function::Function::Asin, vec![arg])
    }

    pub fn acos(arg: Expression) -> Expression {
        Expression::Function(function::Function::Acos, vec![arg])
    }

    pub fn atan(arg: Expression) -> Expression {
        Expression::Function(function::Function::Atan, vec![arg])
    }

    pub fn sinh(arg: Expression) -> Expression {
        Expression::Function(function::Function::Sinh, vec![arg])
    }

    pub fn cosh(arg: Expression) -> Expression {
        Expression::Function(function::Function::Cosh, vec![arg])
    }

    pub fn tanh(arg: Expression) -> Expression {
        Expression::Function(function::Function::Tanh, vec![arg])
    }

    pub fn asinh(arg: Expression) -> Expression {
        Expression::Function(function::Function::Asinh, vec![arg])
    }

    pub fn acosh(arg: Expression) -> Expression {
        Expression::Function(function::Function::Acosh, vec![arg])
    }

    pub fn atanh(arg: Expression) -> Expression {
        Expression::Function(function::Function::Atanh, vec![arg])
    }

    pub fn sqrt(arg: Expression) -> Expression {
        Expression::Function(function::Function::Sqrt, vec![arg])
    }

    pub fn exp(arg: Expression) -> Expression {
        Expression::Function(function::Function::Exp, vec![arg])
    }

    pub fn ln(arg: Expression) -> Expression {
        Expression::Function(function::Function::Ln, vec![arg])
    }

    pub fn log2(arg: Expression) -> Expression {
        Expression::Function(function::Function::Log2, vec![arg])
    }

    pub fn log10(arg: Expression) -> Expression {
        Expression::Function(function::Function::Log10, vec![arg])
    }

    pub fn abs(arg: Expression) -> Expression {
        Expression::Function(function::Function::Abs, vec![arg])
    }

    pub fn ceil(arg: Expression) -> Expression {
        Expression::Function(function::Function::Ceil, vec![arg])
    }

    pub fn floor(arg: Expression) -> Expression {
        Expression::Function(function::Function::Floor, vec![arg])
    }

    pub fn log(arg: Expression, base: Expression) -> Expression {
        Expression::Function(function::Function::Log, vec![base, arg])
    }

    pub fn pow(arg: Expression, order: Expression) -> Expression {
        Expression::Function(function::Function::Pow, vec![order, arg])
    }

    pub fn root(arg: Expression, order: Expression) -> Expression {
        Expression::Function(function::Function::Log, vec![order, arg])
    }

    pub fn e() -> Expression {
        Expression::Constant(Constant::E)
    }
    pub fn pi() -> Expression {
        Expression::Constant(Constant::Pi)
    }
    pub fn tau() -> Expression {
        Expression::Constant(Constant::Tau)
    }
}

// print functions
impl Expression {
    /// Check wether the `Expression` can be printed as one continuous
    fn is_single(&self) -> bool {
        match self {
            Expression::Variable(_)
            | Expression::Constant(_)
            | Expression::Function(_, _)
            | Expression::Number(Numeral::Integer(_)) => true,
            Expression::Exponentiation(base, exp) => base.is_single() && exp.is_single(),
            _ => false,
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Addition(terms) => {
                let terms: Vec<String> = terms
                    .iter()
                    .map(|term| {
                        if term.is_single() {
                            term.to_string()
                        } else {
                            format!("({})", term)
                        }
                    })
                    .collect();
                write!(f, "{}", terms.join(" + "))
            }
            Expression::Multiplication(terms) => {
                let terms: Vec<String> = terms
                    .iter()
                    .map(|term| {
                        if term.is_single() {
                            term.to_string()
                        } else {
                            format!("({})", term)
                        }
                    })
                    .collect();
                write!(f, "{}", terms.join(" * "))
            }
            Expression::Subtraction(lhs, rhs) => write!(
                f,
                "{} - {}",
                lhs,
                if rhs.is_single() {
                    format!("{}", rhs)
                } else {
                    format!("({})", rhs)
                }
            ),
            Expression::Division(lhs, rhs) => write!(
                f,
                "{}/{}",
                if lhs.is_single() {
                    format!("{}", lhs)
                } else {
                    format!("({})", lhs)
                },
                if rhs.is_single() {
                    format!("{}", rhs)
                } else {
                    format!("({})", rhs)
                }
            ),
            Expression::Exponentiation(lhs, rhs) => write!(
                f,
                "{}^{}",
                if lhs.is_single() {
                    format!("{}", lhs)
                } else {
                    format!("({})", lhs)
                },
                if rhs.is_single() {
                    format!("{}", rhs)
                } else {
                    format!("({})", rhs)
                }
            ),
            Expression::Equality(lhs, rhs) => write!(f, "{} = {}", lhs, rhs),
            Expression::Complex(real, imag) => write!(
                f,
                "{}i*{}",
                if real.is_equal(&Expression::integer(0)) {
                    format!("")
                } else {
                    format!("{} + ", real)
                },
                if imag.is_single() {
                    format!("{}", imag)
                } else {
                    format!("({})", imag)
                }
            ),
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Constant(constant) => write!(f, "{}", constant),
            Expression::Negation(expr) => write!(
                f,
                "-{}",
                if expr.is_single() {
                    format!("{}", expr)
                } else {
                    format!("({})", expr)
                }
            ),
            Expression::Function(func, args) => {
                let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
                write!(f, "{}({})", func, args.join(", "))
            }
            Expression::Derivative(expr, variable, order) => {
                write!(
                    f,
                    "d{}/d{}{} {}",
                    if *order != 1 {
                        format!("^{}", order)
                    }else {
                        "".to_string()
                    },
                    if variable.len() != 1 {
                        format!("({})", variable)
                    } else {
                        variable.to_owned()
                    },
                    if *order != 1 {
                        format!("^{}", order)
                    }else {
                        "".to_string()
                    },
                    if expr.is_single() {
                        format!("{}", expr)
                    } else {
                        format!("({})", expr)
                    }
                )
            }
        }
    }
}

// Simplification Functions
impl Expression {
    pub fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        match self {
            Expression::Addition(terms) => {
                let simplified_terms: Vec<Expression> = terms
                    .iter_mut()
                    .map(|term| term.simplify(explanation))
                    .collect::<Result<Vec<Expression>, _>>()?;
                self.simplify_addition(simplified_terms, explanation)
            }
            Expression::Subtraction(lhs, rhs) => {
                let lhs = lhs.simplify(explanation)?;
                let rhs = rhs.simplify(explanation)?;
                let before = &Expression::subtraction(lhs.clone(), rhs.clone());
                match (lhs, rhs) {
                    // a - 0
                    (lhs, Expression::Number(numeral::Numeral::Integer(0))) => {
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Substracting zero stay the same", before, &lhs);
                        }
                        Ok(lhs)},
                    // 0 - a
                    (Expression::Number(numeral::Numeral::Integer(0)), rhs) => {
                        let after  =Expression::Negation(Box::new(rhs));
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Adding zero stay the same", before, &after);
                        }
                        Ok(after)
                    }
                    // a - b => c 
                    (Expression::Number(lhs), Expression::Number(rhs)) => {
                        let after  = lhs.sub(&rhs);
                        if let Some(explanation) = explanation {
                            explanation.rule_applied("Substracting numbers", before, &after);
                        }
                        Ok(after)
                    }
                    // -a - b => -(c)
                    (Expression::Negation(lhs), Expression::Number(rhs)) => {
                        if let Expression::Number(inner_lhs) = *lhs {
                            let after  = Expression::negation(Expression::Number(inner_lhs.add(&rhs)));
                            if let Some(explanation) = explanation {
                                explanation.rule_applied("Substracting numbers", before, &after);
                            }
                            Ok(after)
                        } else {
                            Expression::Addition(vec![*lhs, Expression::negation(Expression::Number(rhs))])
                            .simplify(explanation)
                        }
                    }
                    (lhs, rhs) => {
                        Expression::Addition(vec![lhs, Expression::Negation(Box::new(rhs))])
                            .simplify(explanation)
                    }
                }
            }
            Expression::Multiplication(terms) => {
                let simplified_terms: Vec<Expression> = terms
                    .iter_mut()
                    .map(|term| term.simplify(explanation))
                    .collect::<Result<Vec<Expression>, _>>()?;
                self.simplify_multiplication(simplified_terms, explanation)
            }
            Expression::Division(lhs, rhs) => {
                let lhs = lhs.simplify(explanation)?;
                let rhs = rhs.simplify(explanation)?;
                self.simplify_division(lhs, rhs, explanation)
            }
            Expression::Exponentiation(lhs, rhs) => {
                let lhs = lhs.simplify(explanation)?;
                let rhs = rhs.simplify(explanation)?;

                self.simplify_exponentiation(lhs, rhs, explanation)
            }
            Expression::Negation(expr) => {
                let expr = expr.simplify(explanation)?;
                match expr {
                    // --a => a
                    Expression::Negation(a) => Ok(*a),
                    // -(a b) => -(a b)
                    // -(Num b c) => -(Num) b c
                    Expression::Multiplication(mut a) => {
                        // Find a Expression::integer and transform it to Expression::Negation(Expression::integer)
                        if a.iter_mut().any(|term| {
                            if let Expression::Number(numeral::Numeral::Integer(n)) = term {
                                *term = Expression::Negation(Box::new(Expression::Number(
                                    numeral::Numeral::Integer(*n),
                                )));
                                true
                            } else {
                                false
                            }
                        }) {
                            Ok(Expression::Multiplication(a))
                        } else {
                            Ok(Expression::Negation(Box::new(Expression::Multiplication(
                                a,
                            ))))
                        }
                    }
                    // -(a + b i) => -a -(b) i
                    Expression::Complex(real, imag) => Expression::Complex(
                        Box::new(Expression::Negation(real)),
                        Box::new(Expression::Negation(imag)),
                    )
                    .simplify(explanation),
                    // -(a + b) => -a - b
                    Expression::Addition(add) => {
                        let terms = add
                            .iter()
                            .map(|elem| Expression::negation(elem.clone()))
                            .collect();
                        Expression::Addition(terms).simplify(explanation)
                    }
                    // -0 => 0
                    Expression::Number(numeral::Numeral::Integer(0)) => {
                        Ok(Expression::Number(numeral::Numeral::Integer(0)))
                    }
                    expr => Ok(Expression::Negation(Box::new(expr))),
                }
            }
            Expression::Complex(real, imag) => {
                let real = real.simplify(explanation)?;
                let imag = imag.simplify(explanation)?;
                if imag == Expression::Number(numeral::Numeral::Integer(0)) {
                    Ok(real)
                } else {
                    Ok(Expression::Complex(Box::new(real), Box::new(imag)))
                }
            }
            Expression::Equality(lhs, rhs) => {
                let lhs = lhs.simplify(explanation)?;
                let rhs = rhs.simplify(explanation)?;
                Ok(Expression::Equality(Box::new(lhs), Box::new(rhs)))
            }
            Expression::Function(func, args) => {
                let args: Vec<Expression> = args
                    .iter_mut()
                    .map(|arg| arg.simplify(explanation))
                    .collect::<Result<Vec<Expression>, _>>()?;
                let name = func.clone();
                Ok(Expression::Function(name, args))
            }
            Expression::Number(num) => num.simplify(explanation).map(Expression::Number),
            Expression::Variable(_) | Expression::Constant(_) => Ok(self.clone()),
            Expression::Derivative(expr, variable, order) => {
                let expr = expr.simplify(explanation)?;
                expr.differentiate_n(variable, *order, explanation)
            }
        }
    }

    pub fn simplify_function(
        &mut self,
        func: Function,
        args: Vec<Expression>,
        explanation: &mut Option<Vec<String>>,
    ) -> Result<Expression, SimplifyError> {
        let mut rule = "";

        let result = match func {
            Function::Pow => {
                match (&args[0], &args[1]) {
                    // 0^0 => ZeroExponentiationZero
                    (
                        Expression::Number(numeral::Numeral::Integer(0)),
                        Expression::Number(numeral::Numeral::Integer(0)),
                    ) => Err(SimplifyError::ZeroExponentiationZero),
                    // a^0 => 1
                    (Expression::Number(numeral::Numeral::Integer(0)), _) => {
                        rule = "using a^0 => 1";
                        Ok(Expression::integer(1))
                    }
                    // 1^x
                    (Expression::Number(_), Expression::Number(numeral::Numeral::Integer(1))) => {
                        rule = "using 1^x => 1";
                        Ok(Expression::integer(1))
                    }
                    // a^1 => a
                    (Expression::Number(numeral::Numeral::Integer(1)), lhs) => {
                        rule = "using a^1 => a";
                        Ok(lhs.clone())
                    }
                    _ => Ok(Expression::pow(args[1].clone(), args[0].clone())),
                }
            }
            _ => Ok(Expression::Function(func, args)),
        };

        if !rule.is_empty() {
            if let Some(explanation) = explanation {
                explanation.push(format!("Simplifiyng Exponentiation {}", rule,));
            }
        }

        result
    }
}

impl Expression {
    /// Returns `true` if the two `Expression` are equal and `false` otherwise
    pub fn is_equal(&self, other: &Expression) -> bool {
        match (self, other) {
            (Expression::Number(lhs), Expression::Number(rhs)) => match (lhs, rhs) {
                (numeral::Numeral::Integer(a), numeral::Numeral::Integer(b)) => a == b,
                (numeral::Numeral::Rational(a, b), numeral::Numeral::Rational(c, d)) => {
                    a == c && b == d
                }
                _ => false,
            },
            (Expression::Variable(lhs), Expression::Variable(rhs)) => lhs == rhs,
            (Expression::Constant(lhs), Expression::Constant(rhs)) => lhs == rhs,
            (Expression::Addition(lhs), Expression::Addition(rhs)) => {
                Self::compare_expression_vectors(lhs, rhs)
            }
            (Expression::Subtraction(lhs1, lhs2), Expression::Subtraction(rhs1, rhs2)) => {
                lhs1.is_equal(rhs1) && lhs2.is_equal(rhs2)
            }
            (Expression::Multiplication(lhs), Expression::Multiplication(rhs)) => {
                Self::compare_expression_vectors(lhs, rhs)
            }
            (Expression::Division(lhs1, lhs2), Expression::Division(rhs1, rhs2)) => {
                lhs1.is_equal(rhs1) && lhs2.is_equal(rhs2)
            }
            (Expression::Exponentiation(lhs1, lhs2), Expression::Exponentiation(rhs1, rhs2)) => {
                lhs1.is_equal(rhs1) && lhs2.is_equal(rhs2)
            }
            (Expression::Negation(lhs), Expression::Negation(rhs)) => lhs.is_equal(rhs),
            (Expression::Complex(lhs1, lhs2), Expression::Complex(rhs1, rhs2)) => {
                lhs1.is_equal(rhs1) && lhs2.is_equal(rhs2)
            }
            (Expression::Equality(lhs1, lhs2), Expression::Equality(rhs1, rhs2)) => {
                lhs1.is_equal(rhs1) && lhs2.is_equal(rhs2)
                    || lhs1.is_equal(rhs2) && lhs2.is_equal(rhs1)
            }
            (Expression::Function(lhs1, lhs2), Expression::Function(rhs1, rhs2)) => {
                lhs1 == rhs1 && Self::compare_expression_vectors(lhs2, rhs2)
            }
            (
                Expression::Derivative(lhs, lhs_var, lhs_order),
                Expression::Derivative(rhs, rhs_var, rhs_order),
            ) => lhs_order == rhs_order && lhs_var == rhs_var && lhs.is_equal(rhs),
            _ => false,
        }
    }

    // TODO Refactor to order the inside also
    /// Returns `true` if the two vector are equal and `false` otherwise
    pub fn compare_expression_vectors(lhs: &Vec<Expression>, rhs: &Vec<Expression>) -> bool {
        if rhs.len() != lhs.len() {
            return false;
        }

        let mut rhs = rhs.clone();

        lhs.iter().all(|expr| {
            let pos = rhs.iter().position(|expr2| expr.is_equal(expr2));
            if let Some(p) = pos {
                rhs.swap_remove(p);
                true
            } else {
                false
            }
        })
    }

    pub fn contains_var(&self, variable: &str) -> bool {
        match self {
            Expression::Negation(expression) => expression.contains_var(variable),
            Expression::Constant(_) | Expression::Number(_) => false,
            Expression::Variable(var) => variable == var,
            Expression::Multiplication(expressions) | Expression::Addition(expressions) => {
                expressions.iter().any(|expr| expr.contains_var(variable))
            }
            Expression::Subtraction(lhs, rhs)
            | Expression::Division(lhs, rhs)
            | Expression::Exponentiation(lhs, rhs)
            | Expression::Equality(lhs, rhs)
            | Expression::Complex(lhs, rhs) => {
                lhs.contains_var(variable) || rhs.contains_var(variable)
            }
            Expression::Function(func, expressions) => match func.number_of_arguments() {
                1 => expressions[0].contains_var(variable),
                2 => expressions[1].contains_var(variable),
                _ => panic!("Sould'nt have more than 2 arguments"),
            },
            Expression::Derivative(expression, _, _) => expression.contains_var(variable),
        }
    }
}
