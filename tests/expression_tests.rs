#[cfg(test)]
mod tests {
    use std::vec;
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
        let expr1 = Expression::variable("x");
        let expr2 = Expression::variable("x");
        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::variable("y");
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
        let expr1 = Expression::addition(vec![Expression::integer(2), Expression::integer(3)]);
        let expr2 = Expression::addition(vec![Expression::integer(2), Expression::integer(3)]);
        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::addition(vec![Expression::integer(3), Expression::integer(2)]);
        assert!(expr1.is_equal(&expr3));

        let expr4 = Expression::addition(vec![Expression::integer(2), Expression::integer(4)]);
        assert!(!expr1.is_equal(&expr4));

        let expr5 = Expression::addition(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4),
        ]);

        let expr6 = Expression::addition(vec![
            Expression::integer(3),
            Expression::integer(2),
            Expression::integer(4),
        ]);
        assert!(expr5.is_equal(&expr6));

        let expr7 = Expression::addition(vec![
            Expression::integer(4),
            Expression::integer(2),
            Expression::integer(3),
        ]);
        assert!(expr5.is_equal(&expr7));

        let expr8 = Expression::addition(vec![
            Expression::integer(4),
            Expression::integer(3),
            Expression::integer(2),
        ]);
        assert!(expr5.is_equal(&expr8));
    }

    #[test]
    fn test_is_equal_multiplication() {
        let expr1 =
            Expression::multiplication(vec![Expression::integer(2), Expression::integer(3)]);
        let expr2 =
            Expression::multiplication(vec![Expression::integer(2), Expression::integer(3)]);
        assert!(expr1.is_equal(&expr2));

        let expr3 =
            Expression::multiplication(vec![Expression::integer(3), Expression::integer(2)]);
        assert!(expr1.is_equal(&expr3));

        let expr4 =
            Expression::multiplication(vec![Expression::integer(2), Expression::integer(4)]);
        assert!(!expr1.is_equal(&expr4));

        let expr5 = Expression::multiplication(vec![
            Expression::variable("x"),
            Expression::negation(Expression::integer(2)),
            Expression::sin(Expression::exponentiation(
                Expression::variable("x"),
                Expression::integer(2))
            )
        ]);

        let expr6 = Expression::multiplication(vec![
            Expression::sin(Expression::exponentiation(Expression::variable("x"), Expression::integer(2))),
            Expression::variable("x"),
            Expression::negation(Expression::integer(2))
        ]);

        assert!(expr5.is_equal(&expr6));

        let expr7 = Expression::multiplication(vec![
            Expression::sin(Expression::exponentiation(Expression::variable("x"), Expression::integer(2))),
            Expression::variable("x"),
            Expression::negation(Expression::integer(4))
        ]);

        assert!(!expr5.is_equal(&expr7));
    
        let expr8 =
        Expression::multiplication(vec![Expression::variable("a"), Expression::variable("b"), Expression::integer(2)]);
        let expr9 = 
        Expression::multiplication(vec![Expression::variable("a"), Expression::variable("b")]);

        assert!(!expr8.is_equal(&expr9));    
    }

    #[test]
    fn test_is_equal_subtraction() {
        let expr1 = Expression::subtraction(Expression::integer(2), Expression::integer(3));
        let expr2 = Expression::subtraction(Expression::integer(2), Expression::integer(3));
        assert!(expr1.is_equal(&expr2));
    }

    #[test]
    fn test_is_equal_division() {
        let expr1 = Expression::division(Expression::integer(2), Expression::integer(3));
        let expr2 = Expression::division(Expression::integer(2), Expression::integer(3));
        assert!(expr1.is_equal(&expr2));
    }

    #[test]
    fn test_is_equal_exponentiation() {
        let expr1 = Expression::exponentiation(Expression::integer(2), Expression::integer(3));
        let expr2 = Expression::exponentiation(Expression::integer(2), Expression::integer(3));
        assert!(expr1.is_equal(&expr2));
    }
    

    #[test]
    fn test_is_equal_equality() {
        let expr1 = Expression::equality(Expression::integer(2), Expression::integer(3));
        let expr2 = Expression::equality(Expression::integer(2), Expression::integer(3));
        assert!(expr1.is_equal(&expr2));
    }
    

    #[test]
    fn test_is_equal_complex() {
        let expr1 = Expression::complex(Expression::integer(2), Expression::integer(3));
        let expr2 = Expression::complex(Expression::integer(2), Expression::integer(3));
        assert!(expr1.is_equal(&expr2));
    }
    

    #[test]
    fn test_is_equal_negation() {
        let expr1 = Expression::negation(Expression::integer(2));
        let expr2 = Expression::negation(Expression::integer(2));
        assert!(expr1.is_equal(&expr2));
    }

    #[test]
    fn test_is_equal_function() {
        let expr1 = Expression::sin(Expression::integer(2));
        let expr2 = Expression::sin(Expression::integer(2));
        assert!(expr1.is_equal(&expr2));
    }   

    #[test]
    fn test_is_equal_long_input() {
        let expr1 = Expression::addition(vec![
            Expression::multiplication(vec![Expression::integer(2), Expression::integer(3)]),
            Expression::multiplication(vec![Expression::integer(4), Expression::integer(5)]),
        ]);

        assert!(expr1.is_equal(&expr1));

        let expr2 = Expression::addition(vec![
            Expression::multiplication(vec![Expression::integer(3), Expression::integer(2)]),
            Expression::multiplication(vec![Expression::integer(5), Expression::integer(4)]),
        ]);

        assert!(expr1.is_equal(&expr2));

        let expr3 = Expression::addition(vec![
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(3)
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(3)
            ),
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(2)
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(2)
            ),
        ]);

        assert!(expr3.is_equal(&expr3));

        let expr4 = Expression::addition(vec![
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(3)
            ),
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(2)
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(3)
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(2)
            ),
        ]);
        
        assert!(expr3.is_equal(&expr4));

        let expr5 = Expression::multiplication(vec![
            Expression::integer(4),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(3)
            ),
            Expression::variable("a"),
        ]);

        assert!(expr5.is_equal(&expr5));

        let expr6 = Expression::multiplication(vec![
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(3)
            ),
            Expression::integer(4),
            Expression::variable("a"),
        ]);

        assert!(expr5.is_equal(&expr6));

        let expr7 = Expression::addition(vec![
            Expression::integer(4),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(3)
            ),
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(3)
            ),
            Expression::variable("b"),
        ]);

        assert!(expr7.is_equal(&expr7));

        let expr8 = Expression::addition(vec![
            Expression::integer(4),
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(3)
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(3)
            ),
            Expression::variable("b"),
        ]);

        assert!(expr7.is_equal(&expr8));

        let expr9 = Expression::addition(vec![
            Expression::multiplication(vec![
                Expression::integer(3),
                Expression::variable("b"),
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(2)
                ),
            ]),
            Expression::multiplication(vec![
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(2)
                ),
                Expression::integer(3),
                Expression::variable("a")
            ]),
            Expression::multiplication(vec![
                Expression::variable("a"),
                Expression::integer(2),
                Expression::variable("b")
            ]),
        ]);

        assert!(expr9.is_equal(&expr9));

        let expr10 = Expression::addition(vec![
            Expression::multiplication(vec![
                Expression::integer(3),
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(2)
                ),
                Expression::variable("a")
            ]),
            Expression::multiplication(vec![
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(2)
                ),
                Expression::integer(3),
                Expression::variable("b")
            ]),
            Expression::multiplication(vec![
                Expression::integer(2),
                Expression::variable("a"),
                Expression::variable("b")
            ]),
        ]);

        assert!(expr9.is_equal(&expr10));
    }
}
