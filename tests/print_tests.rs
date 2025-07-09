#[cfg(test)]
mod print_tests {
    use std::vec;

    use sym_rustic::{ast::{
        constant::Constant, Expression
    }, prints::PrettyPrints};

    #[test]
    fn test_calculate_tree_simple_expressions() {
        // Test simple expressions
        let var = Expression::variable("x");
        assert_eq!(var.calculate_tree(0), "x");

        let num = Expression::integer(42);
        assert_eq!(num.calculate_tree(0), "42");

        let neg = Expression::negation(Expression::integer(5));
        assert_eq!(neg.calculate_tree(0), "Negation:\n  - 5");
    }

    #[test]
    fn test_calculate_tree_addition() {
        // Test addition with multiple terms
        let add = Expression::addition(vec![
            Expression::variable("x"),
            Expression::integer(5),
            Expression::negation(Expression::variable("y")),
        ]);

        let expected = "Addition:\n  + x\n  + 5\n  + Negation:\n    - y";
        assert_eq!(add.calculate_tree(0), expected);

        // Test empty addition
        let empty_add = Expression::addition(vec![]);
        assert_eq!(empty_add.calculate_tree(0), "0");

        // Test single term addition
        let single_add = Expression::addition(vec![Expression::integer(10)]);
        assert_eq!(single_add.calculate_tree(0), "10");
    }

    #[test]
    fn test_calculate_tree_multiplication() {
        // Test multiplication with multiple terms
        let mult = Expression::multiplication(vec![
            Expression::variable("x"),
            Expression::integer(2),
            Expression::variable("y"),
        ]);

        let expected = "Multiplication:\n  * x\n  * 2\n  * y";
        assert_eq!(mult.calculate_tree(0), expected);

        // Test empty multiplication
        let empty_mult = Expression::multiplication(vec![]);
        assert_eq!(empty_mult.calculate_tree(0), "1");

        // Test single term multiplication
        let single_mult = Expression::multiplication(vec![Expression::integer(10)]);
        assert_eq!(single_mult.calculate_tree(0), "10");
    }

    #[test]
    fn test_calculate_tree_binary_operations() {
        // Test subtraction
        let sub = Expression::subtraction(
            Expression::variable("x"),
            Expression::integer(5),
        );

        let expected = "Subtraction:\n  x\n  - 5";
        assert_eq!(sub.calculate_tree(0), expected);

        // Test division
        let div = Expression::division(
            Expression::variable("x"),
            Expression::integer(2),
        );

        let expected = "Division:\n  x\n  / 2";
        assert_eq!(div.calculate_tree(0), expected);

        // Test exponentiation
        let exp = Expression::exponentiation(
            Expression::variable("x"),
            Expression::integer(2),
        );

        let expected = "Exponentiation:\n  x\n  ^ 2";
        assert_eq!(exp.calculate_tree(0), expected);
    }

    #[test]
    fn test_calculate_tree_complex_expressions() {
        // Test complex expressions with nested operations
        let complex = Expression::complex(
            Expression::variable("a"),
            Expression::variable("b"),
        );

        let expected = "Complex:\n  a\n  i b";
        assert_eq!(complex.calculate_tree(0), expected);

        // Test equality
        let eq = Expression::equality(
            Expression::variable("x"),
            Expression::integer(5),
        );

        let expected = "Equality:\n  x\n  = 5";
        assert_eq!(eq.calculate_tree(0), expected);
    }

    #[test]
    fn test_calculate_tree_nested_expressions() {
        // Test deeply nested expressions
        let nested = Expression::addition(vec![
            Expression::multiplication(vec![
                Expression::variable("x"),
                Expression::integer(2),
            ]),
            Expression::division(
                Expression::variable("y"),
                Expression::integer(3),
            ),
            Expression::exponentiation(
                Expression::variable("z"),
                Expression::integer(2),
            ),
        ]);

        let expected = "Addition:\n  + Multiplication:\n    * x\n    * 2\n  + Division:\n    y\n    / 3\n  + Exponentiation:\n    z\n    ^ 2";
        assert_eq!(nested.calculate_tree(0), expected);
    }

    #[test]
    fn test_calculate_tree_function() {
        // Test function with multiple arguments
        let func = Expression::sqrt(
            Expression::addition(vec![
                Expression::variable("x"),
                Expression::integer(1),
            ]),
        );

        let expected = "sqrt(Addition:\n  + x\n  + 1)";
        assert_eq!(func.calculate_tree(0), expected);
    }

    #[test]
    fn test_calculate_tree_with_indentation() {
        // Test that indentation works correctly
        let expr = Expression::addition(vec![
            Expression::multiplication(vec![
                Expression::variable("x"),
                Expression::integer(2),
            ]),
            Expression::integer(5),
        ]);

        let expected = "Addition:\n    + Multiplication:\n      * x\n      * 2\n    + 5";
        assert_eq!(expr.calculate_tree(2), expected);
    }

    #[test]
    fn test_get_processed() {
        let expr = Expression::exponentiation(
            Expression::addition(vec![
                Expression::variable("a"),
                Expression::variable("b"),
            ]),
            Expression::integer(2),
        );
        let expected = "       2\n(a + b) ";
        assert_eq!(expr.get_processed(), expected);

        let expr2 = Expression::division(
            Expression::variable("a"),
            Expression::addition(vec![
                Expression::variable("a"),
                Expression::variable("b"),
            ]),
        );
        let expected2 = "  a  \n-----\na + b";
        assert_eq!(expr2.get_processed(), expected2);

        let expr3 = Expression::exponentiation(
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(2),
            ),
            Expression::integer(6),
        );
        let expected3 = "    6\n/ 2\\ \n\\a / ";
        assert_eq!(expr3.get_processed(), expected3);

        let expr4 = Expression::multiplication(vec![
            Expression::integer(3),
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(2),
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(4),
            ),
        ]);
        let expected4 = "     2    4\n3 * a  * b ";
        assert_eq!(expr4.get_processed(), expected4);

        let expr5 = Expression::sqrt(
            Expression::addition(vec![
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::addition(vec![
                        Expression::variable("x"),
                        Expression::variable("y"),
                    ]),
                ),
                Expression::variable("b"),
            ]),
        );
        let expected5 = "    / x + y    \\\nsqrt\\a      + b/";
        assert_eq!(expr5.get_processed(), expected5);

        let expr6 = Expression::addition(vec![
            Expression::multiplication(vec![
                Expression::integer(4),
                Expression::exponentiation(
                    Expression::variable("x"),
                    Expression::integer(3),
                ),
            ]),
            Expression::multiplication(vec![
                Expression::ln(Expression::integer(2)),
                Expression::exponentiation(
                    Expression::variable("x"),
                    Expression::integer(2),
                ),
            ]),
            Expression::division(
                Expression::integer(3),
                Expression::exponentiation(
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::exponentiation(
                            Expression::variable("x"),
                            Expression::exponentiation(
                                Expression::integer(3),
                                Expression::variable("y"),
                            ),
                        ),
                    ]),
                    Expression::integer(4),
                ),
            ),
        ]);

        // expr6.print_console();
        // 4*x^3 + ln(2)*x^2 + 3/(2*x^3^y)^4
        let expected6 = "     3            2       3     \n4 * x  + ln(2) * x  + ----------\n                               4\n                      /      y\\ \n                      |     3 | \n                      \\2 * x  / ";
        assert_eq!(expr6.get_processed(), expected6);

        let expr7 = Expression::multiplication(vec![
            Expression::integer(2),
            Expression::division(
                Expression::integer(4),
                Expression::exponentiation(
                    Expression::integer(4),
                    Expression::integer(4),
                ),
            ),
            Expression::division(
                Expression::integer(6),
                Expression::integer(4),
            ),
        ]);

        // expr7.print_console();

        let expected7 = "    4    6\n2 * -- * -\n     4   4\n    4     ";
        assert_eq!(expr7.get_processed(), expected7);

        let expr8 = Expression::exponentiation(
            Expression::division(
                Expression::addition(vec![
                    Expression::integer(6),
                    Expression::integer(10),
                ]),
                Expression::integer(4),
            ),
            Expression::integer(4),
        );
        // expr8.print_console();
        let expected8 = "        4\n/6 + 10\\ \n|------| \n\\  4   / ";
        assert_eq!(expr8.get_processed(), expected8);

        let expr9 = Expression::rational(143, 12);

        let expected9 = "143\n---\n12 ";
        assert_eq!(expr9.get_processed(), expected9);

        let expr10 = Expression::derivative(
            Expression::Constant(Constant::Tau),
            "x",
            1,
        );
        assert_eq!(expr10.get_processed(), "d      \n--- tau\nd x    ");

        let expr11 = Expression::derivative(
            Expression::division(
                Expression::variable("a"),
                Expression::addition(vec![
                    Expression::variable("a"),
                    Expression::variable("b"),
                ]),
            ),
            "a_1",
            2,
        );
        assert_eq!(
            expr11.get_processed(),
            "  2         \n d       a  \n------ -----\n     2 a + b\nd a_1       "
        );

        let expr12 = Expression::derivative(
            Expression::division(
                Expression::integer(3),
                Expression::exponentiation(
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::exponentiation(
                            Expression::variable("x"),
                            Expression::exponentiation(
                                Expression::integer(3),
                                Expression::variable("y"),
                            ),
                        ),
                    ]),
                    Expression::integer(4),
                ),
            ),
            "a_1",
            10,
        );
        //expr12.print_console();
        assert_eq!(
            expr12.get_processed(),
            "  10              \n d          3     \n------- ----------\n     10          4\nd a_1   /      y\\ \n        |     3 | \n        \\2 * x  / "
        );

        let expr13 = Expression::addition(vec![
            Expression::division(
                Expression::integer(3),
                Expression::exponentiation(
                    Expression::multiplication(vec![
                        Expression::integer(2),
                        Expression::exponentiation(
                            Expression::variable("x"),
                            Expression::exponentiation(
                                Expression::integer(3),
                                Expression::variable("y"),
                            ),
                        ),
                    ]),
                    Expression::integer(4),
                ),
            ),
            Expression::exponentiation(
                Expression::exponentiation(
                    Expression::exponentiation(
                        Expression::exponentiation(
                            Expression::exponentiation(
                                Expression::integer(2),
                                Expression::integer(2),
                            ),
                            Expression::integer(2),
                        ),
                        Expression::integer(2),
                    ),
                    Expression::integer(2),
                ),
                Expression::integer(2),
            ),
        ]);
        // expr13.print_console();
        assert_eq!(
            expr13.get_processed(),
            "                          2\n             /          2\\ \n             |/       2\\ | \n             ||/    2\\ | | \n    3        |||/ 2\\ | | | \n---------- + \\\\\\\\2 / / / / \n         4                 \n/      y\\                  \n|     3 |                  \n\\2 * x  /                  "
        );
    }
}
