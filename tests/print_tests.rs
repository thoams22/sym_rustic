#[cfg(test)]
mod print_tests {
    use std::vec;

    use sym_rustic::ast::{
        Expression,
        constant::Constant,
        function::{self, Function},
    };

    #[test]
    fn test_print_tree_simple_expressions() {
        // Test simple expressions
        let var = Expression::Variable("x".to_string());
        assert_eq!(var.print_tree(0), "x");

        let num = Expression::integer(42);
        assert_eq!(num.print_tree(0), "42");

        let neg = Expression::Negation(Box::new(Expression::integer(5)));
        assert_eq!(neg.print_tree(0), "Negation:\n  - 5");
    }

    #[test]
    fn test_print_tree_addition() {
        // Test addition with multiple terms
        let add = Expression::Addition(vec![
            Expression::Variable("x".to_string()),
            Expression::integer(5),
            Expression::Negation(Box::new(Expression::Variable("y".to_string()))),
        ]);

        let expected = "Addition:\n  + x\n  + 5\n  + Negation:\n    - y";
        assert_eq!(add.print_tree(0), expected);

        // Test empty addition
        let empty_add = Expression::Addition(vec![]);
        assert_eq!(empty_add.print_tree(0), "0");

        // Test single term addition
        let single_add = Expression::Addition(vec![Expression::integer(10)]);
        assert_eq!(single_add.print_tree(0), "10");
    }

    #[test]
    fn test_print_tree_multiplication() {
        // Test multiplication with multiple terms
        let mult = Expression::Multiplication(vec![
            Expression::Variable("x".to_string()),
            Expression::integer(2),
            Expression::Variable("y".to_string()),
        ]);

        let expected = "Multiplication:\n  * x\n  * 2\n  * y";
        assert_eq!(mult.print_tree(0), expected);

        // Test empty multiplication
        let empty_mult = Expression::Multiplication(vec![]);
        assert_eq!(empty_mult.print_tree(0), "1");

        // Test single term multiplication
        let single_mult = Expression::Multiplication(vec![Expression::integer(10)]);
        assert_eq!(single_mult.print_tree(0), "10");
    }

    #[test]
    fn test_print_tree_binary_operations() {
        // Test subtraction
        let sub = Expression::Subtraction(
            Box::new(Expression::Variable("x".to_string())),
            Box::new(Expression::integer(5)),
        );

        let expected = "Subtraction:\n  x\n  - 5";
        assert_eq!(sub.print_tree(0), expected);

        // Test division
        let div = Expression::Division(
            Box::new(Expression::Variable("x".to_string())),
            Box::new(Expression::integer(2)),
        );

        let expected = "Division:\n  x\n  / 2";
        assert_eq!(div.print_tree(0), expected);

        // Test exponentiation
        let exp = Expression::Exponentiation(
            Box::new(Expression::Variable("x".to_string())),
            Box::new(Expression::integer(2)),
        );

        let expected = "Exponentiation:\n  x\n  ^ 2";
        assert_eq!(exp.print_tree(0), expected);
    }

    #[test]
    fn test_print_tree_complex_expressions() {
        // Test complex expressions with nested operations
        let complex = Expression::Complex(
            Box::new(Expression::Variable("a".to_string())),
            Box::new(Expression::Variable("b".to_string())),
        );

        let expected = "Complex:\n  a\n  i b";
        assert_eq!(complex.print_tree(0), expected);

        // Test equality
        let eq = Expression::Equality(
            Box::new(Expression::Variable("x".to_string())),
            Box::new(Expression::integer(5)),
        );

        let expected = "Equality:\n  x\n  = 5";
        assert_eq!(eq.print_tree(0), expected);
    }

    #[test]
    fn test_print_tree_nested_expressions() {
        // Test deeply nested expressions
        let nested = Expression::Addition(vec![
            Expression::Multiplication(vec![
                Expression::Variable("x".to_string()),
                Expression::integer(2),
            ]),
            Expression::Division(
                Box::new(Expression::Variable("y".to_string())),
                Box::new(Expression::integer(3)),
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("z".to_string())),
                Box::new(Expression::integer(2)),
            ),
        ]);

        let expected = "Addition:\n  + Multiplication:\n    * x\n    * 2\n  + Division:\n    y\n    / 3\n  + Exponentiation:\n    z\n    ^ 2";
        assert_eq!(nested.print_tree(0), expected);
    }

    #[test]
    fn test_print_tree_function() {
        // Test function with multiple arguments
        let func = Expression::Function(
            function::Function::Sqrt,
            vec![Expression::Addition(vec![
                Expression::Variable("x".to_string()),
                Expression::integer(1),
            ])],
        );

        let expected = "sqrt(Addition:\n  + x\n  + 1)";
        assert_eq!(func.print_tree(0), expected);
    }

    #[test]
    fn test_print_tree_with_indentation() {
        // Test that indentation works correctly
        let expr = Expression::Addition(vec![
            Expression::Multiplication(vec![
                Expression::Variable("x".to_string()),
                Expression::integer(2),
            ]),
            Expression::integer(5),
        ]);

        let expected = "Addition:\n    + Multiplication:\n      * x\n      * 2\n    + 5";
        assert_eq!(expr.print_tree(2), expected);
    }

    #[test]
    fn test_get_processed() {
        let expr = Expression::Exponentiation(
            Box::new(Expression::Addition(vec![
                Expression::Variable("a".to_string()),
                Expression::Variable("b".to_string()),
            ])),
            Box::new(Expression::integer(2)),
        );
        let expected = "       2\n(a + b) ";
        assert_eq!(expr.get_processed(), expected);

        let expr2 = Expression::Division(
            Box::new(Expression::Variable("a".to_string())),
            Box::new(Expression::Addition(vec![
                Expression::Variable("a".to_string()),
                Expression::Variable("b".to_string()),
            ])),
        );
        let expected2 = "  a  \n-----\na + b";
        assert_eq!(expr2.get_processed(), expected2);

        let expr3 = Expression::Exponentiation(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(2)),
            )),
            Box::new(Expression::integer(6)),
        );
        let expected3 = "    6\n/ 2\\ \n\\a / ";
        assert_eq!(expr3.get_processed(), expected3);

        let expr4 = Expression::Multiplication(vec![
            Expression::integer(3),
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(2)),
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(4)),
            ),
        ]);
        let expected4 = "     2    4\n3 * a  * b ";
        assert_eq!(expr4.get_processed(), expected4);

        let expr5 = Expression::Function(
            Function::Sqrt,
            vec![Expression::Addition(vec![
                Expression::Exponentiation(
                    Box::new(Expression::Variable("a".to_string())),
                    Box::new(Expression::Addition(vec![
                        Expression::Variable("x".to_string()),
                        Expression::Variable("y".to_string()),
                    ])),
                ),
                Expression::Variable("b".to_string()),
            ])],
        );
        let expected5 = "    / x + y    \\\nsqrt\\a      + b/";
        assert_eq!(expr5.get_processed(), expected5);

        let expr6 = Expression::Addition(vec![
            Expression::Multiplication(vec![
                Expression::integer(4),
                Expression::Exponentiation(
                    Box::new(Expression::Variable("x".to_string())),
                    Box::new(Expression::integer(3)),
                ),
            ]),
            Expression::Multiplication(vec![
                Expression::Function(Function::Ln, vec![Expression::integer(2)]),
                Expression::Exponentiation(
                    Box::new(Expression::Variable("x".to_string())),
                    Box::new(Expression::integer(2)),
                ),
            ]),
            Expression::Division(
                Box::new(Expression::integer(3)),
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Multiplication(vec![
                        Expression::integer(2),
                        Expression::Exponentiation(
                            Box::new(Expression::Variable("x".to_string())),
                            Box::new(Expression::Exponentiation(
                                Box::new(Expression::integer(3)),
                                Box::new(Expression::Variable("y".to_string())),
                            )),
                        ),
                    ])),
                    Box::new(Expression::integer(4)),
                )),
            ),
        ]);

        // expr6.print_console();
        // 4*x^3 + ln(2)*x^2 + 3/(2*x^3^y)^4
        let expected6 = "     3            2       3     \n4 * x  + ln(2) * x  + ----------\n                               4\n                      /      y\\ \n                      |     3 | \n                      \\2 * x  / ";
        assert_eq!(expr6.get_processed(), expected6);

        let expr7 = Expression::Multiplication(vec![
            Expression::integer(2),
            Expression::Division(
                Box::new(Expression::integer(4)),
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::integer(4)),
                    Box::new(Expression::integer(4)),
                )),
            ),
            Expression::Division(
                Box::new(Expression::integer(6)),
                Box::new(Expression::integer(4)),
            ),
        ]);

        // expr7.print_console();

        let expected7 = "    4    6\n2 * -- * -\n     4   4\n    4     ";
        assert_eq!(expr7.get_processed(), expected7);

        let expr8 = Expression::Exponentiation(
            Box::new(Expression::Division(
                Box::new(Expression::Addition(vec![
                    Expression::integer(6),
                    Expression::integer(10),
                ])),
                Box::new(Expression::integer(4)),
            )),
            Box::new(Expression::integer(4)),
        );
        // expr8.print_console();
        let expected8 = "        4\n/6 + 10\\ \n|------| \n\\  4   / ";
        assert_eq!(expr8.get_processed(), expected8);

        let expr9 = Expression::rational(143, 12);

        let expected9 = "143\n---\n12 ";
        assert_eq!(expr9.get_processed(), expected9);

        let expr10 = Expression::Derivative(
            Box::new(Expression::Constant(Constant::Tau)),
            "x".to_string(),
            1,
        );
        assert_eq!(expr10.get_processed(), "d      \n--- tau\nd x    ");

        let expr11 = Expression::Derivative(
            Box::new(Expression::Division(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::Addition(vec![
                    Expression::Variable("a".to_string()),
                    Expression::Variable("b".to_string()),
                ])),
            )),
            "a_1".to_string(),
            2,
        );
        assert_eq!(
            expr11.get_processed(),
            "  2         \n d       a  \n------ -----\n     2 a + b\nd a_1       "
        );

        let expr12 = Expression::Derivative(
            Box::new(Expression::Division(
                Box::new(Expression::integer(3)),
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Multiplication(vec![
                        Expression::integer(2),
                        Expression::Exponentiation(
                            Box::new(Expression::Variable("x".to_string())),
                            Box::new(Expression::Exponentiation(
                                Box::new(Expression::integer(3)),
                                Box::new(Expression::Variable("y".to_string())),
                            )),
                        ),
                    ])),
                    Box::new(Expression::integer(4)),
                )),
            )),
            "a_1".to_string(),
            10,
        );
        //expr12.print_console();
        assert_eq!(
            expr12.get_processed(),
            "  10              \n d          3     \n------- ----------\n     10          4\nd a_1   /      y\\ \n        |     3 | \n        \\2 * x  / "
        );

        let expr13 = Expression::Addition(vec![
            Expression::Division(
                Box::new(Expression::integer(3)),
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Multiplication(vec![
                        Expression::integer(2),
                        Expression::Exponentiation(
                            Box::new(Expression::Variable("x".to_string())),
                            Box::new(Expression::Exponentiation(
                                Box::new(Expression::integer(3)),
                                Box::new(Expression::Variable("y".to_string())),
                            )),
                        ),
                    ])),
                    Box::new(Expression::integer(4)),
                )),
            ),
            Expression::Exponentiation(
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Exponentiation(
                        Box::new(Expression::Exponentiation(
                            Box::new(Expression::Exponentiation(
                                Box::new(Expression::integer(2)),
                                Box::new(Expression::integer(2)),
                            )),
                            Box::new(Expression::integer(2)),
                        )),
                        Box::new(Expression::integer(2)),
                    )),
                    Box::new(Expression::integer(2)),
                )),
                Box::new(Expression::integer(2)),
            ),
        ]);
        // expr13.print_console();
        assert_eq!(
            expr13.get_processed(),
            "                          2\n             /          2\\ \n             |/       2\\ | \n             ||/    2\\ | | \n    3        |||/ 2\\ | | | \n---------- + \\\\\\\\2 / / / / \n         4                 \n/      y\\                  \n|     3 |                  \n\\2 * x  /                  "
        );
    }
}
