#[cfg(test)]
mod test_linear {
    use sym_rustic::{
        ast::Expression,
        solver::{Solution, Solver, SolverError},
    };

    #[test]
    fn simple() {
        // x = 0
        let solution = Solver::solve_linear(
            Expression::equality(Expression::variable("x"), Expression::integer(0)),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(Solution::Single(Expression::integer(0)).is_equal(&solution));

        // x + 2 = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![Expression::variable("x"), Expression::integer(2)]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::negation(Expression::integer(2))).is_equal(&solution),
            "{}",
            solution
        );

        // x + y = 2
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![Expression::variable("x"), Expression::variable("y")]),
                Expression::integer(2),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::addition(vec![
                Expression::integer(2),
                Expression::negation(Expression::variable("y"))
            ]))
            .is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn subtraction() {
        // x - 5 = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::subtraction(Expression::variable("x"), Expression::integer(5)),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(5)).is_equal(&solution),
            "{}",
            solution
        );

        // 3 - x = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::subtraction(Expression::integer(3), Expression::variable("x")),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(3)).is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn multiplication() {
        // 2x = 8
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::multiplication(vec![Expression::integer(2), Expression::variable("x")]),
                Expression::integer(8),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(4)).is_equal(&solution),
            "{}",
            solution
        );

        // 3x = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::multiplication(vec![Expression::integer(3), Expression::variable("x")]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(0)).is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn division() {
        // x/2 = 4
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::division(Expression::variable("x"), Expression::integer(2)),
                Expression::integer(4),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(8)).is_equal(&solution),
            "{}",
            solution
        );

        // Not linear but still solves bcs no
        // 10/x = 2
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::division(Expression::integer(10), Expression::variable("x")),
                Expression::integer(2),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(5)).is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn complex_linear() {
        // 2x + 3 = 7
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::variable("x"),
                    ]),
                    Expression::integer(3),
                ]),
                Expression::integer(7),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(2)).is_equal(&solution),
            "{}",
            solution
        );

        // 5x - 4 = 2x + 8
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::subtraction(
                    Expression::multiplication(vec![
                        Expression::integer(5),
                        Expression::variable("x"),
                    ]),
                    Expression::integer(4),
                ),
                Expression::addition(vec![
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::variable("x"),
                    ]),
                    Expression::integer(8),
                ]),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(4)).is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn rational_coefficients() {
        // x/2 + 1/3 = 5/6
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![
                    Expression::division(Expression::variable("x"), Expression::integer(2)),
                    Expression::rational(1, 3),
                ]),
                Expression::rational(5, 6),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(1)).is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn negation() {
        // -x = 5
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::negation(Expression::variable("x")),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::negation(Expression::integer(5))).is_equal(&solution),
            "{}",
            solution
        );

        // -2x + 4 = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![
                    Expression::negation(Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::variable("x"),
                    ])),
                    Expression::integer(4),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Single(Expression::integer(2)).is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn no_solution() {
        // 0x = 5 (no solution)
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::multiplication(vec![Expression::integer(0), Expression::variable("x")]),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(Solution::None.is_equal(&solution), "{}", solution);
    }

    #[test]
    fn infinite_solutions() {
        // 0x = 0 (infinite solutions)
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::multiplication(vec![Expression::integer(0), Expression::variable("x")]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(Solution::Infinite.is_equal(&solution), "{}", solution);
    }

    #[test]
    fn errors() {
        // Not linear
        // 10/x + 2 = 5
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![
                    Expression::division(Expression::integer(10), Expression::variable("x")),
                    Expression::integer(2),
                ]),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::UnsupportedEquationType, result);

        // 3y = 5 for x
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::multiplication(vec![Expression::integer(3), Expression::variable("y")]),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::VariableNotFound, result);

        // 3y = 5 for 3
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::multiplication(vec![Expression::integer(3), Expression::variable("y")]),
                Expression::integer(5),
            ),
            Expression::integer(3),
        )
        .unwrap_err();
        assert_eq!(SolverError::InvalidVariable, result);

        // 3 = 5 for x
        let result = Solver::solve_linear(
            Expression::equality(Expression::integer(3), Expression::integer(5)),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::VariableNotFound, result);

        // 3 + 5
        let result = Solver::solve_linear(
            Expression::addition(vec![Expression::integer(3), Expression::integer(5)]),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::InvalidEquation, result);

        // ln(x) = 3
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::ln(Expression::variable("x")),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::InvalidSolver, result);

        // x^2 + 2x = 0
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::multiplication(vec![
                        Expression::variable("x"),
                        Expression::integer(2),
                    ]),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::CouldNotSolve, result);
    }
}

#[cfg(test)]
mod test_quadratic {
    use sym_rustic::{
        ast::Expression,
        solver::{Solution, Solver, SolverError},
    };

    #[test]
    fn simple_quadratic() {
        // x^2 = 4
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                Expression::integer(4),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::integer(2),
                Expression::negation(Expression::integer(2))
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // x^2 - 4 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::subtraction(
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::integer(4),
                ),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::integer(2),
                Expression::negation(Expression::integer(2))
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // x^2 - 6 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::subtraction(
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::integer(6),
                ),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::sqrt(Expression::integer(6)),
                Expression::negation(Expression::sqrt(Expression::integer(6)))
            ])
            .is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn complex_quadratic() {
        // x^2 + 3x + 2 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::multiplication(vec![
                        Expression::integer(3),
                        Expression::variable("x"),
                    ]),
                    Expression::integer(2),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::negation(Expression::integer(2)),
                Expression::negation(Expression::integer(1))
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // 2x^2 - 4x - 6 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::subtraction(
                    Expression::addition(vec![
                        Expression::multiplication(vec![
                            Expression::integer(2),
                            Expression::exponentiation(
                                Expression::variable("x"),
                                Expression::integer(2),
                            ),
                        ]),
                        Expression::multiplication(vec![
                            Expression::negation(Expression::integer(4)),
                            Expression::variable("x"),
                        ]),
                    ]),
                    Expression::integer(6),
                ),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::integer(3),
                Expression::negation(Expression::integer(1))
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // x^2 - x - 1 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::negation(Expression::variable("x")),
                    Expression::negation(Expression::integer(1)),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::division(
                    Expression::addition(vec![
                        Expression::integer(1),
                        Expression::sqrt(Expression::integer(5))
                    ]),
                    Expression::integer(2)
                ),
                Expression::division(
                    Expression::addition(vec![
                        Expression::integer(1),
                        Expression::negation(Expression::sqrt(Expression::integer(5)))
                    ]),
                    Expression::integer(2)
                )
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // ax^2 + bx + c = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::multiplication(vec![
                        Expression::variable("a"),
                        Expression::exponentiation(
                            Expression::variable("x"),
                            Expression::integer(2),
                        ),
                    ]),
                    Expression::multiplication(vec![
                        Expression::variable("b"),
                        Expression::variable("x"),
                    ]),
                    Expression::variable("c"),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::division(
                    Expression::addition(vec![
                        Expression::negation(Expression::variable("b")),
                        Expression::negation(Expression::sqrt(Expression::addition(vec![
                            Expression::exponentiation(
                                Expression::variable("b"),
                                Expression::integer(2)
                            ),
                            Expression::multiplication(vec![
                                Expression::negation(Expression::integer(4)),
                                Expression::variable("a"),
                                Expression::variable("c")
                            ])
                        ])))
                    ]),
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::variable("a")
                    ])
                ),
                Expression::division(
                    Expression::addition(vec![
                        Expression::negation(Expression::variable("b")),
                        Expression::sqrt(Expression::addition(vec![
                            Expression::exponentiation(
                                Expression::variable("b"),
                                Expression::integer(2)
                            ),
                            Expression::multiplication(vec![
                                Expression::negation(Expression::integer(4)),
                                Expression::variable("a"),
                                Expression::variable("c")
                            ])
                        ]))
                    ]),
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::variable("a")
                    ])
                )
            ])
            .is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn no_real_solution() {
        // x^2 + 1 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::integer(1),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::complex(Expression::integer(0), Expression::integer(1)),
                Expression::complex(
                    Expression::integer(0),
                    Expression::negation(Expression::integer(1))
                )
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // x^2 + 2x + 5 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::variable("x"),
                    ]),
                    Expression::integer(5),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::complex(
                    Expression::negation(Expression::integer(1)),
                    Expression::integer(2)
                ),
                Expression::complex(
                    Expression::negation(Expression::integer(1)),
                    Expression::negation(Expression::integer(2))
                )
            ])
            .is_equal(&solution),
            "{}",
            solution
        );

        // x^2 - 4x + 13 = 0
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                    Expression::multiplication(vec![
                        Expression::negation(Expression::integer(4)),
                        Expression::variable("x"),
                    ]),
                    Expression::integer(13),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(
            Solution::Multiple(vec![
                Expression::complex(Expression::integer(2), Expression::integer(3)),
                Expression::complex(
                    Expression::integer(2),
                    Expression::negation(Expression::integer(3))
                )
            ])
            .is_equal(&solution),
            "{}",
            solution
        );
    }

    #[test]
    fn infinite_solutions() {
        // 0x^2 = 0 (infinite solutions)
        let solution = Solver::solve_quadratic(
            Expression::equality(
                Expression::multiplication(vec![
                    Expression::integer(0),
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(2)),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert!(Solution::Infinite.is_equal(&solution), "{}", solution);
    }

    #[test]
    fn errors() {
        // Not quadratic
        // x^3 + 2 = 0
        let result = Solver::solve_quadratic(
            Expression::equality(
                Expression::addition(vec![
                    Expression::exponentiation(Expression::variable("x"), Expression::integer(3)),
                    Expression::integer(2),
                ]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::CouldNotSolve, result);

        // 3y^2 = 5 for x
        let result = Solver::solve_quadratic(
            Expression::equality(
                Expression::multiplication(vec![
                    Expression::integer(3),
                    Expression::exponentiation(Expression::variable("y"), Expression::integer(2)),
                ]),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::VariableNotFound, result);

        // 3 = 5 for x
        let result = Solver::solve_quadratic(
            Expression::equality(Expression::integer(3), Expression::integer(5)),
            Expression::variable("x"),
        )
        .unwrap_err();
        assert_eq!(SolverError::VariableNotFound, result);
    }
}
