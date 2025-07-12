// // Solving equations for specific variables
// // Handling different types of equations (linear, quadratic, etc.)

// // Implementing numerical methods for solving equations
// // Implementing symbolic methods for solving equations
// // Implementing methods for solving systems of equations
// // Implementing methods for solving inequalities
// // Implementing methods for solving differential equations
// // Implementing methods for solving integral equations

use std::vec;

use crate::ast::{Expr, Expression, SimplifyError, numeral::Numeral};

#[derive(Debug, PartialEq)]
pub enum SolverError {
    /// Equation type not supported by the solver
    UnsupportedEquationType,
    /// The variable specified was not found in the equation
    VariableNotFound,
    /// The equation provided is not of type `Expression::Equation`
    InvalidEquation,
    /// The variable provided is not of type `Expression::Variable`
    InvalidVariable,
    /// The solver chosen cannot solve this type of equation
    InvalidSolver,
    /// The solver could not solve the equation
    CouldNotSolve,
    /// There was an error in a simplification step
    Simplifiyng(SimplifyError),
}

impl From<SimplifyError> for SolverError {
    fn from(error: SimplifyError) -> Self {
        SolverError::Simplifiyng(error)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Single(Expression),
    Multiple(Vec<Expression>),
    None,
    Infinite,
}

impl Solution {
    pub fn is_equal(&self, other: &Solution) -> bool {
        match (self, other) {
            (Solution::Single(lhs), Solution::Single(rhs)) => lhs.is_equal(rhs),
            (Solution::Multiple(lhs), Solution::Multiple(rhs)) => {
                Expression::compare_expression_vectors(&lhs, &rhs)
            }
            (Solution::None, Solution::None) => true,
            (Solution::Infinite, Solution::Infinite) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Single(expression) => write!(f, "Solution: {}", expression),
            Solution::Multiple(expressions) => {
                let mut result = String::from("Solutions:\n");
                expressions.iter().enumerate().for_each(|(i, expr)| {
                    result.push_str(&format!("Sol{}: {}", i, expr));
                });
                write!(f, "{}", result)
            }
            Solution::None => write!(f, "No solution"),
            Solution::Infinite => write!(f, "Infinity of solution"),
        }
    }
}

// // Add steps for solving equations
pub struct Solver;

// impl Solver {

//     /// Solves the given equation for the specified variable.
//     pub fn solve_for(equation: Equality, variable: Variable) -> Result<Solution, SolverError> {
//             if let Expression::Variable(_var) = variable {
//                 // Implement the logic to solve the equation for the variable
//                 // This is a placeholder implementation
//                 Ok(Solution::Single(Expression::Equality(lhs, rhs)))
//             } else {
//                 Err(SolverError::InvalidVariable)
//             }
//         }
// }

// Linear equations
impl Solver {
    /// Solves a linear equation for the specified variable.
    ///
    /// The variable must be `Expression::Variable` type.
    ///
    /// The equation must be `Expression::Equality` type.
    pub fn solve_linear(
        equation: Expression,
        variable: Expression,
    ) -> Result<Solution, SolverError> {
        if let Expression::Variable(var) = variable {
            if let Expression::Equality(mut equ) = equation {
                // Put all containing the var on the left
                match (
                    equ.left.contains_var(&var.name),
                    equ.right.contains_var(&var.name),
                ) {
                    (true, true) => {
                        equ.left =
                            Expression::subtraction(equ.left, equ.right).simplify(&mut None)?;
                        equ.right = Expression::integer(0);
                    }
                    (true, false) => {
                        // Already in the good from
                        equ.left = equ.left.simplify(&mut None)?;
                        equ.right = equ.right.simplify(&mut None)?;
                    }
                    (false, true) => {
                        let mut left = equ.left.clone();
                        equ.left = equ.right.simplify(&mut None)?;
                        equ.right = left.simplify(&mut None)?;
                    }
                    (false, false) => {
                        return Err(SolverError::VariableNotFound);
                    }
                };

                // Send the part not containing the variale of interest on the right
                loop {
                    println!("{} = {}", &equ.left, &equ.right);
                    let last_left = equ.left.clone();
                    match equ.left {
                        // x = Y
                        Expression::Variable(variable) if variable.is_equal(&var) => {
                            return Ok(Solution::Single(equ.right));
                        }
                        // 0 = Y
                        Expression::Number(Numeral::Integer(0))
                            if equ.right.is_equal(&Expression::integer(0)) =>
                        {
                            return Ok(Solution::Infinite);
                        }
                        Expression::Constant(_)
                        | Expression::Number(_)
                        | Expression::Variable(_) => {
                            // A = Y
                            return Ok(Solution::None);
                        }
                        Expression::Addition(add) => {
                            // Ax + B = Y => Ax = Y - B
                            let mut not_containing: Vec<Expression> = vec![];
                            let mut containing: Vec<Expression> = vec![];
                            add.terms.iter().for_each(|term| {
                                if term.contains_var(&var.name) {
                                    containing.push(term.clone());
                                } else {
                                    not_containing.push(term.clone());
                                }
                            });
                            equ.right = Expression::subtraction(
                                equ.right,
                                Expression::addition(not_containing),
                            )
                            .simplify(&mut None)?;
                            equ.left = Expression::addition(containing).simplify(&mut None)?;
                        }
                        Expression::Multiplication(mul) => {
                            // ABx = Y => x = Y/AB
                            let mut not_containing: Vec<Expression> = vec![];
                            let mut containing: Vec<Expression> = vec![];
                            mul.terms.iter().for_each(|term| {
                                if term.contains_var(&var.name) {
                                    containing.push(term.clone());
                                } else {
                                    not_containing.push(term.clone());
                                }
                            });
                            equ.right = Expression::division(
                                equ.right,
                                Expression::multiplication(not_containing),
                            )
                            .simplify(&mut None)?;
                            equ.left =
                                Expression::multiplication(containing).simplify(&mut None)?;
                        }
                        Expression::Subtraction(sub) => {
                            match (
                                sub.left.contains_var(&var.name),
                                sub.right.contains_var(&var.name),
                            ) {
                                (true, true) => {
                                    // Lx - Rx = Y => (L - R)x = Y
                                    return Err(SolverError::UnsupportedEquationType);
                                }
                                (true, false) => {
                                    // Lx - R = Y => Lx = Y + R
                                    equ.right = Expression::addition(vec![equ.right, sub.right])
                                        .simplify(&mut None)?;
                                    equ.left = sub.left;
                                }
                                (false, true) => {
                                    // L - Rx = Y => Rx = L - Y
                                    equ.right = Expression::subtraction(sub.left, equ.right)
                                        .simplify(&mut None)?;
                                    equ.left = sub.right;
                                }
                                (false, false) => {
                                    // L - R = Y => No Solution for x
                                    return Ok(Solution::None);
                                }
                            }
                        }
                        Expression::Negation(neg) => {
                            if neg.term.contains_var(&var.name) {
                                // -Ax = Y => Ax = -Y
                                equ.right = Expression::negation(equ.right).simplify(&mut None)?;
                                equ.left = neg.term
                            } else {
                                // -A = Y => No solution for x
                                return Ok(Solution::None);
                            }
                        }
                        Expression::Division(div) => {
                            match (
                                div.num.contains_var(&var.name),
                                div.den.contains_var(&var.name),
                            ) {
                                (true, true) => {
                                    // Nx/Dx = Y => N/D = Y
                                    return Err(SolverError::UnsupportedEquationType);
                                }
                                (true, false) => {
                                    // Nx/D = Y => Nx = YD
                                    equ.right =
                                        Expression::multiplication(vec![equ.right, div.den])
                                            .simplify(&mut None)?;
                                    equ.left = div.num;
                                }
                                (false, true) => {
                                    // N/Dx = Y => Dx = N/Y
                                    equ.right = Expression::division(div.num, equ.right)
                                        .simplify(&mut None)?;
                                    equ.left = div.den;
                                }
                                (false, false) => {
                                    // N/D = Y => No Solution for x
                                    return Ok(Solution::None);
                                }
                            }
                        }
                        _ => return Err(SolverError::InvalidSolver),
                    };
                    if equ.left.is_equal(&last_left) {
                        break;
                    }
                }

                if equ.left.is_equal(&Expression::variable(&var.name)) {
                    Ok(Solution::Single(equ.right))
                } else {
                    Err(SolverError::CouldNotSolve)
                }
            } else {
                Err(SolverError::InvalidEquation)
            }
        } else {
            Err(SolverError::InvalidVariable)
        }
    }
}

// Quadratic equations
impl Solver {
    /// Solves a quadratic equation for the specified variable.
    ///
    /// The variable must be `Expression::Variable` type.
    ///
    /// The equation must be `Expression::Equality` type.
    pub fn solve_quadratic(
        equation: Expression,
        variable: Expression,
    ) -> Result<Solution, SolverError> {
        if let Expression::Variable(var) = variable {
            if let Expression::Equality(mut equ) = equation {
                
                Err(SolverError::UnsupportedEquationType)
            } else {
                Err(SolverError::InvalidEquation)
            }
        } else {
            Err(SolverError::InvalidVariable)
        }
    }
}