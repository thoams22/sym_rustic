use std::collections::HashMap;

use function::Function;

use crate::{
    ast::{
        addition::Addition, complex::Complex, constant::Constant, derivative::Derivative,
        division::Division, equality::Equality, exponentiation::Exponentiation,
        function::FunctionType, multiplication::Multiplication, negation::Negation,
        numeral::Numeral, subtraction::Subtraction, variable::Variable,
    },
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
mod subtraction;
mod variable;

#[derive(Debug, PartialEq, Clone)]
pub enum SimplifyError {
    DivisionByZero,
    ZeroExponentiationZero,
    InvalidDerivative,
    Unsupported,
}

pub trait Expr: std::fmt::Display {
    fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError>;

    fn is_equal(&self, other: &Self) -> bool;
    fn contains_var(&self, variable: &str) -> bool;
    fn is_single(&self) -> bool;
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Expression {
    // Unary
    Negation(Box<Negation>),
    Number(Numeral),
    Variable(Variable),
    Constant(Constant),
    // Multinary
    Addition(Addition),
    Multiplication(Multiplication),
    // Binary
    Subtraction(Box<Subtraction>),
    Division(Box<Division>),
    Exponentiation(Box<Exponentiation>),
    Equality(Box<Equality>),
    Complex(Box<Complex>),
    // Function
    Function(Function),
    // Calculus
    Derivative(Box<Derivative>),
    // Integral(Box<Integral>),
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
        Expression::Subtraction(Box::new(Subtraction::new(lhs, rhs, false)))
    }

    pub fn equality(left: Expression, right: Expression) -> Expression {
        Expression::Equality(Box::new(Equality::new(left, right, false)))
    }

    pub fn negation(arg: Expression) -> Expression {
        Expression::Negation(Box::new(Negation::new(arg, false)))
    }

    pub fn equality(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Equality(Box::new(lhs), Box::new(rhs))
    }

    pub fn complex(real: Expression, imag: Expression) -> Expression {
        Expression::Complex(Box::new(Complex::new(real, imag, false)))
    }

    pub fn addition(terms: Vec<Expression>) -> Expression {
        Expression::Addition(Addition::new(terms, false))
    }

    pub fn multiplication(terms: Vec<Expression>) -> Expression {
        Expression::Multiplication(Multiplication::new(terms, false))
    }

    pub fn variable(name: &str) -> Expression {
        Expression::Variable(Variable::new(name))
    }

    pub fn exponentiation(base: Expression, expo: Expression) -> Expression {
        Expression::Exponentiation(Box::new(Exponentiation::new(base, expo, false)))
    }

    pub fn division(num: Expression, den: Expression) -> Expression {
        Expression::Division(Box::new(Division::new(num, den, false)))
    }

    pub fn derivative(term: Expression, variable: &str, order: u32) -> Expression {
        Expression::Derivative(Box::new(Derivative::new(
            term,
            variable.to_owned(),
            order,
            false,
        )))
    }

    pub fn function(name: FunctionType, args: Vec<Expression>) -> Expression {
        Expression::Function(Function::new(name, args, false))
    }

    pub fn sin(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Sin, vec![arg], false))
    }

    pub fn cos(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Cos, vec![arg], false))
    }

    pub fn tan(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Tan, vec![arg], false))
    }

    pub fn asin(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Asin, vec![arg], false))
    }

    pub fn acos(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Acos, vec![arg], false))
    }

    pub fn atan(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Atan, vec![arg], false))
    }

    pub fn sinh(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Sinh, vec![arg], false))
    }

    pub fn cosh(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Cosh, vec![arg], false))
    }

    pub fn tanh(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Tanh, vec![arg], false))
    }

    pub fn asinh(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Asinh, vec![arg], false))
    }

    pub fn acosh(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Acosh, vec![arg], false))
    }

    pub fn atanh(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Atanh, vec![arg], false))
    }

    pub fn sqrt(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Sqrt, vec![arg], false))
    }

    pub fn exp(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Exp, vec![arg], false))
    }

    pub fn ln(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Ln, vec![arg], false))
    }

    pub fn log2(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Log2, vec![arg], false))
    }

    pub fn log10(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Log10, vec![arg], false))
    }

    pub fn abs(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Abs, vec![arg], false))
    }

    pub fn ceil(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Ceil, vec![arg], false))
    }

    pub fn floor(arg: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Floor, vec![arg], false))
    }

    pub fn log(arg: Expression, base: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Log, vec![arg, base], false))
    }

    pub fn pow(arg: Expression, order: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Pow, vec![arg, order], false))
    }

    pub fn root(arg: Expression, order: Expression) -> Expression {
        Expression::Function(Function::new(FunctionType::Root, vec![arg, order], false))
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

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Negation(negation) => write!(f, "{}", negation),
            Expression::Number(numeral) => write!(f, "{}", numeral),
            Expression::Variable(var) => write!(f, "{}", var),
            Expression::Constant(constant) => write!(f, "{}", constant),
            Expression::Addition(addition) => write!(f, "{}", addition),
            Expression::Multiplication(multiplication) => write!(f, "{}", multiplication),
            Expression::Subtraction(substraction) => write!(f, "{}", substraction),
            Expression::Division(division) => write!(f, "{}", division),
            Expression::Exponentiation(exponentiation) => write!(f, "{}", exponentiation),
            Expression::Equality(equality) => write!(f, "{}", equality),
            Expression::Complex(complex) => write!(f, "{}", complex),
            Expression::Function(function) => write!(f, "{}", function),
            Expression::Derivative(derivative) => write!(f, "{}", derivative),
        }
    }
}

impl Expression {
    pub fn simplify(
        &mut self,
        explanation: &mut Option<Box<FormattingObserver>>,
    ) -> Result<Expression, SimplifyError> {
        match self {
            Expression::Addition(add) => add.simplify(explanation),
            Expression::Subtraction(sub) => sub.simplify(explanation),
            Expression::Multiplication(mul) => mul.simplify(explanation),
            Expression::Division(div) => div.simplify(explanation),
            Expression::Exponentiation(exp) => exp.simplify(explanation),
            Expression::Negation(neg) => neg.simplify(explanation),
            Expression::Complex(com) => com.simplify(explanation),
            Expression::Equality(equ) => equ.simplify(explanation),
            Expression::Function(fun) => fun.simplify(explanation),
            Expression::Number(num) => num.simplify(explanation),
            Expression::Variable(_) => Ok(self.clone()),
            Expression::Constant(con) => con.simplify(explanation),
            Expression::Derivative(der) => der.simplify(explanation),
        }
    }

    /// Returns `true` if the two `Expression` are equal and `false` otherwise
    pub fn is_equal(&self, other: &Expression) -> bool {
        match (self, other) {
            (Expression::Number(lhs), Expression::Number(rhs)) => lhs.is_equal(rhs),
            (Expression::Variable(lhs), Expression::Variable(rhs)) => lhs.is_equal(rhs),
            (Expression::Constant(lhs), Expression::Constant(rhs)) => lhs.is_equal(rhs),
            (Expression::Addition(lhs), Expression::Addition(rhs)) => lhs.is_equal(rhs),
            (Expression::Subtraction(lhs), Expression::Subtraction(rhs)) => lhs.is_equal(rhs),
            (Expression::Multiplication(lhs), Expression::Multiplication(rhs)) => lhs.is_equal(rhs),
            (Expression::Division(lhs), Expression::Division(rhs)) => lhs.is_equal(rhs),
            (Expression::Exponentiation(lhs), Expression::Exponentiation(rhs)) => lhs.is_equal(rhs),
            (Expression::Negation(lhs), Expression::Negation(rhs)) => lhs.is_equal(rhs),
            (Expression::Complex(lhs), Expression::Complex(rhs)) => lhs.is_equal(rhs),
            (Expression::Equality(lhs), Expression::Equality(rhs)) => lhs.is_equal(rhs),
            (Expression::Function(lhs), Expression::Function(rhs)) => lhs.is_equal(rhs),
            (Expression::Derivative(lhs), Expression::Derivative(rhs)) => lhs.is_equal(rhs),
            _ => false,
        }
    }

    /// Check wether the `Expression` can be printed as one continuous
    fn is_single(&self) -> bool {
        match self {
            Expression::Negation(negation) => negation.is_single(),
            Expression::Number(numeral) => numeral.is_single(),
            Expression::Variable(variable) => variable.is_single(),
            Expression::Constant(constant) => constant.is_single(),
            Expression::Addition(addition) => addition.is_single(),
            Expression::Multiplication(multiplication) => multiplication.is_single(),
            Expression::Subtraction(subtraction) => subtraction.is_single(),
            Expression::Division(division) => division.is_single(),
            Expression::Exponentiation(exponentiation) => exponentiation.is_single(),
            Expression::Equality(equality) => equality.is_single(),
            Expression::Complex(complex) => complex.is_single(),
            Expression::Function(function) => function.is_single(),
            Expression::Derivative(derivative) => derivative.is_single(),
        }
    }

    /// Returns `true` if the two vector are equal and `false` otherwise
    pub fn compare_expression_vectors(lhs: &[Expression], rhs: &[Expression]) -> bool {
        if rhs.len() != lhs.len() {
            return false;
        }

        let mut rhs = rhs.to_owned();

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

    /// Transform a `&[Expression]` representing an `Expression::Multiplication` and an Expression into a `Option<tuple>` representing
    /// the Expression in common, if each one is negative and the coefficient between them
    ///
    /// (a_negative, terms_negative, common_expr, coeff)
    fn reduce_add_mult<'b>(
        terms: &[Expression],
        a: &'b Expression,
    ) -> Option<(bool, bool, &'b Expression, u64)> {
        let mut coeff = 1;
        let mut terms_neg = false;

        let (expr_neg, expr): (bool, &Expression) = if let Expression::Negation(expr) = a {
            (true, &expr.term)
        } else {
            (false, a)
        };

        let equal = terms.iter().all(|term| {
            if term.is_equal(expr) {
                true
            } else {
                match term {
                    Expression::Negation(inner) => {
                        if let Expression::Number(numeral::Numeral::Integer(b)) = inner.term {
                            coeff *= b;
                            terms_neg = !terms_neg;
                            true
                        } else {
                            inner.term.is_equal(expr)
                        }
                    }
                    Expression::Number(numeral::Numeral::Integer(b)) => {
                        coeff *= b;
                        true
                    }
                    _ => false,
                }
            }
        });

        if equal {
            Some((terms_neg, expr_neg, expr, coeff))
        } else {
            None
        }
    }

    // /// Returns `true`  if the expression contains a `Expression::Complex` term.
    // /// Should be used on simplified expressions.
    // pub fn is_complex(&self) -> bool {
    //     match self {
    //         Expression::Complex(_) => true,
    //         Expression::Addition(terms) | Expression::Multiplication(terms) => {
    //             terms.iter().any(|term| term.is_complex())
    //         }
    //         Expression::Negation(term) => term.is_complex(),
    //         Expression::Division(a, b) => a.is_complex() || b.is_complex(),
    //         Expression::Exponentiation(a, b) => a.is_complex() || b.is_complex(),
    //         _ => false,
    //     }
    // }

    pub fn contains_var(&self, variable: &str) -> bool {
        match self {
            Expression::Negation(negation) => negation.contains_var(variable),
            Expression::Number(numeral) => numeral.contains_var(variable),
            Expression::Variable(var) => var.contains_var(variable),
            Expression::Constant(constant) => constant.contains_var(variable),
            Expression::Addition(addition) => addition.contains_var(variable),
            Expression::Multiplication(multiplication) => multiplication.contains_var(variable),
            Expression::Subtraction(substraction) => substraction.contains_var(variable),
            Expression::Division(division) => division.contains_var(variable),
            Expression::Exponentiation(exponentiation) => exponentiation.contains_var(variable),
            Expression::Equality(equality) => equality.contains_var(variable),
            Expression::Complex(complex) => complex.contains_var(variable),
            Expression::Function(function) => function.contains_var(variable),
            Expression::Derivative(derivative) => derivative.contains_var(variable),
        }
    }
}
