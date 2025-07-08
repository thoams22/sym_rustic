#[cfg(test)]
mod test_linear {
    use sym_rustic::{
        ast::Expression,
        solver::{Solution, Solver},
    };

    #[test]
    fn simple() {
        // x = 0
        let solution = Solver::solve_linear(
            Expression::equality(Expression::variable("x"), Expression::integer(0)),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(0)), solution);

        // x + 2 = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::Addition(vec![Expression::variable("x"), Expression::integer(2)]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(
            Solution::Single(Expression::negation(Expression::integer(0))),
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
        assert_eq!(Solution::Single(Expression::integer(5)), solution);

        // 3 - x = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::subtraction(Expression::integer(3), Expression::variable("x")),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(3)), solution);
    }

    #[test]
    fn multiplication() {
        // 2x = 8
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::Multiplication(vec![Expression::integer(2), Expression::variable("x")]),
                Expression::integer(8),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(4)), solution);

        // 3x = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::Multiplication(vec![Expression::integer(3), Expression::variable("x")]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(0)), solution);
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
        assert_eq!(Solution::Single(Expression::integer(8)), solution);

        // 10/x = 2
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::division(Expression::integer(10), Expression::variable("x")),
                Expression::integer(2),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(5)), solution);
    }

    #[test]
    fn complex_linear() {
        // 2x + 3 = 7
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::Addition(vec![
                    Expression::Multiplication(vec![Expression::integer(2), Expression::variable("x")]),
                    Expression::integer(3),
                ]),
                Expression::integer(7),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(2)), solution);

        // 5x - 4 = 2x + 8
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::subtraction(
                    Expression::Multiplication(vec![Expression::integer(5), Expression::variable("x")]),
                    Expression::integer(4),
                ),
                Expression::Addition(vec![
                    Expression::Multiplication(vec![Expression::integer(2), Expression::variable("x")]),
                    Expression::integer(8),
                ]),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(4)), solution);
    }

    #[test]
    fn rational_coefficients() {
        // x/2 + 1/3 = 5/6
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::Addition(vec![
                    Expression::division(Expression::variable("x"), Expression::integer(2)),
                    Expression::rational(1, 3),
                ]),
                Expression::rational(5, 6),
            ),
            Expression::variable("x"),
        )
        .unwrap();
        assert_eq!(Solution::Single(Expression::integer(1)), solution);
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
        assert_eq!(Solution::Single(Expression::negation(Expression::integer(5))), solution);

        // -2x + 4 = 0
        let solution = Solver::solve_linear(
            Expression::equality(
                Expression::Addition(vec![
                    Expression::negation(Expression::Multiplication(vec![
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
        assert_eq!(Solution::Single(Expression::integer(2)), solution);
    }

    #[test]
    fn no_solution() {
        // 0x = 5 (no solution)
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::Multiplication(vec![Expression::integer(0), Expression::variable("x")]),
                Expression::integer(5),
            ),
            Expression::variable("x"),
        ).unwrap();
        assert_eq!(result, Solution::None);
    }

    #[test]
    fn infinite_solutions() {
        // 0x = 0 (infinite solutions)
        let result = Solver::solve_linear(
            Expression::equality(
                Expression::Multiplication(vec![Expression::integer(0), Expression::variable("x")]),
                Expression::integer(0),
            ),
            Expression::variable("x"),
        ).unwrap();
        assert_eq!(result, Solution::Infinite);
    }
}
