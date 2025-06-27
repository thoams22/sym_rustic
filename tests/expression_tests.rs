#[cfg(test)]
mod tests {
    use std::vec;

    use sym_rustic::ast::function::Function;
    use sym_rustic::ast::Expression;
    use sym_rustic::ast::constant::Constant;

    #[test]
    fn test_is_equal_numbers() {
        let expr1 = Expression::integer(5);
        let expr2 = Expression::integer(5);
        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::integer(3);
        assert!(!expr1.is_equal(&expr3));

        let expr4 = Expression::rational(1, 2);
        let expr5 = Expression::rational(1, 2);
        assert!(expr4.is_equal(&expr5));

        let expr6 = Expression::rational(1, 3);
        assert!(!expr4.is_equal(&expr6));
        
    }

    #[test]
    fn test_is_equal_variables() {
        let expr1 = Expression::Variable("x".to_string());
        let expr2 = Expression::Variable("x".to_string());
        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::Variable("y".to_string());
        assert!(!expr1.is_equal(&expr3));
    }

    #[test]
    fn test_is_equal_constants() {
        let expr1 = Expression::Constant(Constant::Pi);
        let expr2 = Expression::Constant(Constant::Pi);
        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::Constant(Constant::E);
        assert!(!expr1.is_equal(&expr3));
    }

    #[test]
    fn test_is_equal_addition() {
        let expr1 = Expression::Addition(vec![Expression::integer(2), Expression::integer(3)]);
        let expr2 = Expression::Addition(vec![Expression::integer(2), Expression::integer(3)]);
        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::Addition(vec![Expression::integer(3), Expression::integer(2)]);
        assert!(expr1.is_equal(&expr3));

        let expr4 = Expression::Addition(vec![Expression::integer(2), Expression::integer(4)]);
        assert!(!expr1.is_equal(&expr4));

        let expr5 = Expression::Addition(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4),
        ]);

        let expr6 = Expression::Addition(vec![
            Expression::integer(3),
            Expression::integer(2),
            Expression::integer(4),
        ]);
        assert!(expr5.is_equal(&expr6));

        let expr7 = Expression::Addition(vec![
            Expression::integer(4),
            Expression::integer(2),
            Expression::integer(3),
        ]);
        assert!(expr5.is_equal(&expr7));

        let expr8 = Expression::Addition(vec![
            Expression::integer(4),
            Expression::integer(3),
            Expression::integer(2),
        ]);
        assert!(expr5.is_equal(&expr8));
    }

    #[test]
    fn test_is_equal_multiplication() {
        let expr1 =
            Expression::Multiplication(vec![Expression::integer(2), Expression::integer(3)]);
        let expr2 =
            Expression::Multiplication(vec![Expression::integer(2), Expression::integer(3)]);
        assert!(expr1.is_equal(&expr2));

        let expr3 =
            Expression::Multiplication(vec![Expression::integer(3), Expression::integer(2)]);
        assert!(expr1.is_equal(&expr3));

        let expr4 =
            Expression::Multiplication(vec![Expression::integer(2), Expression::integer(4)]);
        assert!(!expr1.is_equal(&expr4));

        let expr5 = Expression::Multiplication(vec![
            Expression::Variable("x".to_string()),
            Expression::Negation(Box::new(Expression::integer(2))),
            Expression::sin(Expression::Exponentiation(
                Box::new(Expression::Variable("x".to_string())),
                Box::new(Expression::integer(2))
            ))
        ]);

        let expr6 = Expression::Multiplication(vec![
            Expression::sin(Expression::exponentiation(Expression::variable("x"), Expression::integer(2))),
            Expression::variable("x"),
            Expression::negation(Expression::integer(2))
        ]);

        assert!(expr5.is_equal(&expr6));
    }

    #[test]
    fn test_is_equal_subtraction() {
        let expr1 = Expression::Subtraction(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        let expr2 = Expression::Subtraction(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        assert!(expr1.is_equal(&expr2));
    }

    #[test]
    fn test_is_equal_division() {
        let expr1 = Expression::Division(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        let expr2 = Expression::Division(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        assert!(expr1.is_equal(&expr2));
    }

    #[test]
    fn test_is_equal_exponentiation() {
        let expr1 = Expression::Exponentiation(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        let expr2 = Expression::Exponentiation(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        assert!(expr1.is_equal(&expr2));
    }
    

    #[test]
    fn test_is_equal_equality() {
        let expr1 = Expression::Equality(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        let expr2 = Expression::Equality(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        assert!(expr1.is_equal(&expr2));
    }
    

    #[test]
    fn test_is_equal_complex() {
        let expr1 = Expression::Complex(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        let expr2 = Expression::Complex(Box::new(Expression::integer(2)), Box::new(Expression::integer(3)));
        assert!(expr1.is_equal(&expr2));
    }
    

    #[test]
    fn test_is_equal_negation() {
        let expr1 = Expression::Negation(Box::new(Expression::integer(2)));
        let expr2 = Expression::Negation(Box::new(Expression::integer(2)));
        assert!(expr1.is_equal(&expr2));
    }

    #[test]
    fn test_is_equal_function() {
        let expr1 = Expression::Function(Function::Sin, vec![Expression::integer(2)]);
        let expr2 = Expression::Function(Function::Sin, vec![Expression::integer(2)]);
        assert!(expr1.is_equal(&expr2));
    }   

    #[test]
    fn test_is_equal_long_input() {
        let expr1 = Expression::Addition(vec![
            Expression::Multiplication(vec![Expression::integer(2), Expression::integer(3)]),
            Expression::Multiplication(vec![Expression::integer(4), Expression::integer(5)]),
        ]);

        assert!(expr1.is_equal(&expr1));

        let expr2 = Expression::Addition(vec![
            Expression::Multiplication(vec![Expression::integer(3), Expression::integer(2)]),
            Expression::Multiplication(vec![Expression::integer(5), Expression::integer(4)]),
        ]);

        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::Addition(vec![
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(2))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(2))
            ),
        ]);

        assert!(expr3.is_equal(&expr3));

        let expr4 = Expression::Addition(vec![
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(2))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(2))
            ),
        ]);
        
        assert!(expr3.is_equal(&expr4));

        let expr5 = Expression::Multiplication(vec![
            Expression::integer(4),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Variable("a".to_string()),
        ]);

        assert!(expr5.is_equal(&expr5));

        let expr6 = Expression::Multiplication(vec![
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::integer(4),
            Expression::Variable("a".to_string()),
        ]);

        assert!(expr5.is_equal(&expr6));

        let expr7 = Expression::Addition(vec![
            Expression::integer(4),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Variable("b".to_string()),
        ]);

        assert!(expr7.is_equal(&expr7));

        let expr8 = Expression::Addition(vec![
            Expression::integer(4),
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Exponentiation(
                Box::new(Expression::Variable("b".to_string())),
                Box::new(Expression::integer(3))
            ),
            Expression::Variable("b".to_string()),
        ]);

        assert!(expr7.is_equal(&expr8));

        let expr9 = Expression::Addition(vec![
            Expression::Multiplication(vec![
                Expression::integer(3),
                Expression::Variable("b".to_string()),
                Expression::Exponentiation(
                    Box::new(Expression::Variable("a".to_string())),
                    Box::new(Expression::integer(2))
                ),
            ]),
            Expression::Multiplication(vec![
                Expression::Exponentiation(
                    Box::new(Expression::Variable("b".to_string())),
                    Box::new(Expression::integer(2))
                ),
                Expression::integer(3),
                Expression::Variable("a".to_string())
            ]),
            Expression::Multiplication(vec![
                Expression::Variable("a".to_string()),
                Expression::integer(2),
                Expression::Variable("b".to_string())
            ]),
        ]);

        assert!(expr9.is_equal(&expr9));

        let expr10 = Expression::Addition(vec![
            Expression::Multiplication(vec![
                Expression::integer(3),
                Expression::Exponentiation(
                    Box::new(Expression::Variable("b".to_string())),
                    Box::new(Expression::integer(2))
                ),
                Expression::Variable("a".to_string())
            ]),
            Expression::Multiplication(vec![
                Expression::Exponentiation(
                    Box::new(Expression::Variable("a".to_string())),
                    Box::new(Expression::integer(2))
                ),
                Expression::integer(3),
                Expression::Variable("b".to_string())
            ]),
            Expression::Multiplication(vec![
                Expression::integer(2),
                Expression::Variable("a".to_string()),
                Expression::Variable("b".to_string())
            ]),
        ]);

        assert!(expr9.is_equal(&expr10));
    }
}
