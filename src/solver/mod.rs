// Solving equations for specific variables
// Handling different types of equations (linear, quadratic, etc.)

// Implementing numerical methods for solving equations
// Implementing symbolic methods for solving equations
// Implementing methods for solving systems of equations
// Implementing methods for solving inequalities
// Implementing methods for solving differential equations
// Implementing methods for solving integral equations

use crate::ast::{Expression, SimplifyError};

#[derive(Debug)]
pub enum SolverError {
    InvalidEquation,
    UnsupportedEquationType,
    VariableNotFound,
    InvalidVariable,
    InvalidSolver,
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

// Add steps for solving equations
pub struct Solver;

impl Solver {
    /// Solves the given equation for the specified variable.
    ///
    /// The variable must be `Expression::Variable` type.
    ///
    /// The equation must be `Expression::Equality` type.
    pub fn solve_for(equation: Expression, variable: Expression) -> Result<Solution, SolverError> {
        if let Expression::Equality(lhs, rhs) = equation {
            if let Expression::Variable(_var) = variable {
                // Implement the logic to solve the equation for the variable
                // This is a placeholder implementation
                Ok(Solution::Single(Expression::Equality(lhs, rhs)))
            } else {
                Err(SolverError::InvalidVariable)
            }
        } else {
            Err(SolverError::InvalidEquation)
        }
    }
}

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
            if equation.contains_var(&var) {
                if let Expression::Equality(lhs, rhs) = equation {
                    // Put all on the left
                    let equality = if rhs.is_equal(&Expression::integer(0)) {
                        *lhs
                    } else {
                        Expression::Subtraction(lhs, rhs).simplify(&mut None)?
                    };

                    let mut last_equality = equality.clone();
                    // Send the part not containing the variale of interest on the right
                    while equality.is_equal(&last_equality) {
                        last_equality = equality;
                        match equality {
                            Expression::Constant(_)
                            | Expression::Number(_)
                            | Expression::Variable(_) => Solution::Single(Expression::integer(0)),
                            Expression::Addition(expressions) => todo!(),
                            Expression::Multiplication(expressions) => todo!(),
                            Expression::Negation(expression) => todo!(),
                            Expression::Subtraction(lhs, rhs) => todo!(),
                            Expression::Division(lhs, rhs) => todo!(),
                            _ => return Err(SolverError::InvalidSolver),
                        };
                    }

                    Ok(Solution::Infinite)
                } else {
                    Err(SolverError::InvalidEquation)
                }
            } else {
                Err(SolverError::InvalidVariable)
            }
        } else {
            Err(SolverError::VariableNotFound)
        }
    }
}
