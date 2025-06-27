use sym_rustic::lexer::{Lexer, Token};

fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests_number {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_number_1() {
        let tokens: Vec<Token> = lex("42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::integer(42));
    }

    #[test]
    fn test_number_2() {
        let tokens: Vec<Token> = lex("+ 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::integer(42));
    }

    #[test]
    fn test_number_3() {
        let tokens: Vec<Token> = lex("- 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::integer(42)))
        );
    }
}

#[cfg(test)]
mod tests_addition {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::Parser;
    #[test]
    fn test_addition_1() {
        let tokens: Vec<Token> = lex("42 + 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Addition(vec![Expression::integer(42), Expression::integer(42)])
        );
    }

    #[test]
    fn test_addition_2() {
        let tokens: Vec<Token> = lex("38 + 40 + 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Addition(vec![
                Expression::Addition(vec![Expression::integer(38), Expression::integer(40),]),
                Expression::integer(42)
            ])
        );
    }

    #[test]
    fn test_addition_3() {
        let tokens: Vec<Token> = lex("(40 + 42) + 38");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Addition(vec![
                Expression::Addition(vec![Expression::integer(40), Expression::integer(42)]),
                Expression::integer(38)
            ])
        );
    }

    #[test]
    fn test_addition_5() {
        let tokens: Vec<Token> = lex("38 + (40 + 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Addition(vec![
                Expression::integer(38),
                Expression::Addition(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }
}

#[cfg(test)]
mod tests_multiplication {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_multiplication_1() {
        let tokens: Vec<Token> = lex("42 * 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![Expression::integer(42), Expression::integer(42)])
        );
    }

    #[test]
    fn test_multiplication_2() {
        let tokens: Vec<Token> = lex("38 * (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::integer(38),
                Expression::Multiplication(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }

    #[test]
    fn test_multiplication_3() {
        let tokens: Vec<Token> = lex("(40 * 42) * -38");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Multiplication(vec![Expression::integer(40), Expression::integer(42)]),
                Expression::Negation(Box::new(Expression::integer(38)))
            ])
        );
    }

    #[test]
    fn test_multiplication_4() {
        let tokens: Vec<Token> = lex("(40 * 42) 38");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Multiplication(vec![Expression::integer(40), Expression::integer(42)]),
                Expression::integer(38)
            ])
        );
    }

    #[test]
    fn test_multiplication_5() {
        let tokens: Vec<Token> = lex("a (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Variable("a".to_string()),
                Expression::Multiplication(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }

    #[test]
    fn test_multiplication_6() {
        let tokens: Vec<Token> = lex("8 (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::integer(8),
                Expression::Multiplication(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }

    #[test]
    fn test_multiplication_7() {
        let tokens: Vec<Token> = lex(" (a + b)(a - b)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Addition(vec![
                    Expression::Variable("a".to_string()),
                    Expression::Variable("b".to_string())
                ]),
                Expression::Subtraction(
                    Box::new(Expression::Variable("a".to_string())),
                    Box::new(Expression::Variable("b".to_string()))
                )
            ])
        );
    }

    #[test]
    fn test_multiplication_8() {
        let tokens: Vec<Token> = lex(" a * b / c * a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Division(
                    Box::new(Expression::Multiplication(vec![
                        Expression::Variable("a".to_string()),
                        Expression::Variable("b".to_string()),
                    ])),
                    Box::new(Expression::Variable("c".to_string()))
                ),
                Expression::Variable("a".to_string())
            ])
        );
    }

    #[test]
    fn test_multiplication_9() {
        let tokens: Vec<Token> = lex("a b c");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Variable("a".to_string()),
                Expression::Multiplication(vec![
                    Expression::Variable("b".to_string()),
                    Expression::Variable("c".to_string())
                ])
            ])
        );
    }

    #[test]
    fn test_multiplication_10() {
        let tokens: Vec<Token> = lex("a (b c)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Variable("a".to_string()),
                Expression::Multiplication(vec![
                    Expression::Variable("b".to_string()),
                    Expression::Variable("c".to_string())
                ])
            ])
        );
    }

    #[test]
    fn test_multiplication_11() {
        let tokens: Vec<Token> = lex("3 a (b c)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::integer(3),
                Expression::Multiplication(vec![
                    Expression::Variable("a".to_string()),
                    Expression::Multiplication(vec![
                        Expression::Variable("b".to_string()),
                        Expression::Variable("c".to_string())
                    ])
                ])
            ])
        );
    }
}
#[cfg(test)]
mod tests_exponentiations {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_exponentiation_1() {
        let tokens: Vec<Token> = lex("42 ^ 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::integer(42)),
                Box::new(Expression::integer(42))
            )
        )
    }

    #[test]
    fn test_exponentiation_2() {
        let tokens: Vec<Token> = lex("38 ^ (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::integer(38)),
                Box::new(Expression::Multiplication(vec![
                    Expression::integer(40),
                    Expression::integer(42)
                ]))
            )
        );
    }

    #[test]
    fn test_exponentiation_3() {
        let tokens: Vec<Token> = lex("(40 * 42) ^ 38");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::Multiplication(vec![
                    Expression::integer(40),
                    Expression::integer(42)
                ])),
                Box::new(Expression::integer(38))
            )
        );
    }
    #[test]
    fn test_exponentiation_4() {
        let tokens: Vec<Token> = lex("42 ^ 42 ^ 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::integer(42)),
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::integer(42)),
                    Box::new(Expression::integer(42))
                ))
            )
        )
    }
    #[test]
    fn test_exponentiation_5() {
        let tokens: Vec<Token> = lex("42 ^ (42 ^ 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::integer(42)),
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::integer(42)),
                    Box::new(Expression::integer(42))
                ))
            )
        )
    }
    #[test]
    fn test_exponentiation_6() {
        let tokens: Vec<Token> = lex("(42 ^ 42) ^ 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::integer(42)),
                    Box::new(Expression::integer(42))
                )),
                Box::new(Expression::integer(42),)
            )
        )
    }

    #[test]
    fn test_exponentiation_7() {
        let tokens = lex("a ^ (1/2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::Division(
                    Box::new(Expression::integer(1)),
                    Box::new(Expression::integer(2))
                ))
            )
        )
    }

    #[test]
    fn test_exponentiation_8() {
        let tokens = lex("a^1/2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Division(
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Variable("a".to_string())),
                    Box::new(Expression::integer(1))
                )),
                Box::new(Expression::integer(2))
            )
        )
    }
}

#[cfg(test)]
mod tests_literal {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::ast::constant::Constant;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::ParseError::InvalidVariableFormat;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_literal_1() {
        let tokens: Vec<Token> = lex("a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("a".to_string()))
    }

    #[test]
    fn test_literal_2() {
        let tokens = lex("abc");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("abc".to_string()))
    }

    #[test]
    fn test_literal_3() {
        let tokens: Vec<Token> = lex("a_1");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("a_1".to_string()))
    }

    #[test]
    fn test_literal_4() {
        let tokens: Vec<Token> = lex("a_a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("a_a".to_string()))
    }

    #[test]
    fn test_literal_5() {
        let tokens: Vec<Token> = lex("a__a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(expr, InvalidVariableFormat(2))
    }

    #[test]
    fn test_literal_6() {
        let tokens: Vec<Token> = lex("a_a_a_b");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("a_a_a_b".to_string()))
    }

    #[test]
    fn test_literal_7() {
        let tokens: Vec<Token> = lex("a_123_12");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("a_123_12".to_string()))
    }

    #[test]
    fn test_literal_8() {
        let tokens: Vec<Token> = lex("e");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Constant(Constant::E))
    }

    #[test]
    fn test_literal_9() {
        let tokens: Vec<Token> = lex("pi");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Constant(Constant::Pi))
    }

    #[test]
    fn test_literal_10() {
        let tokens: Vec<Token> = lex("tau");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Constant(Constant::Tau))
    }

    #[test]
    fn test_literal_11() {
        let tokens: Vec<Token> = lex("E");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("E".to_string()))
    }

    #[test]
    fn test_literal_12() {
        let tokens: Vec<Token> = lex("e_3");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("e_3".to_string()))
    }

    #[test]
    fn test_literal_13() {
        let tokens: Vec<Token> = lex("pi_a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("pi_a".to_string()))
    }

    #[test]
    fn test_literal_14() {
        let tokens: Vec<Token> = lex("tau_628");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::Variable("tau_628".to_string()))
    }
}

#[cfg(test)]
mod tests_functions {
    use crate::lex;
    use sym_rustic::ast::Expression::Function;
    use sym_rustic::ast::{Expression, function};
    use sym_rustic::parser::{ParseError, Parser};

    #[test]
    fn test_function_1() {
        let tokens = lex("sin(3 + 8)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Function(
                function::Function::Sin,
                vec![Expression::Addition(vec![
                    Expression::integer(3),
                    Expression::integer(8)
                ])]
            )
        )
    }

    #[test]
    fn test_function_2() {
        let tokens = lex("sin (3 + 8)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Variable("sin".to_string()),
                Expression::Addition(vec![Expression::integer(3), Expression::integer(8)])
            ])
        )
    }

    #[test]
    fn test_function_3() {
        let tokens = lex("log(10, a)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Function(
                function::Function::Log,
                vec![
                    Expression::integer(10),
                    Expression::Variable("a".to_string())
                ]
            )
        )
    }

    #[test]
    fn test_function_4() {
        let tokens = lex("log(10)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(
            expr,
            ParseError::InvalidFunctionFormat("log".to_string(), 1, 4)
        )
    }

    #[test]
    fn test_function_5() {
        let tokens = lex("log ( 10, a)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(expr, ParseError::UnexpectedToken("Comma".to_string(), 5))
    }
}

#[cfg(test)]
mod tests_complex {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_complex_1() {
        let tokens = lex("a + b*i");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Addition(vec![
                Expression::Variable("a".to_string()),
                Expression::Multiplication(vec![
                    Expression::Variable("b".to_string()),
                    Expression::Complex(
                        Box::new(Expression::integer(0)),
                        Box::new(Expression::integer(1)),
                    )
                ])
            ])
        )
    }

    #[test]
    fn test_complex_2() {
        let tokens = lex("b*i^5");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Variable("b".to_string()),
                Expression::Exponentiation(
                    Box::new(Expression::Complex(
                        Box::new(Expression::integer(0)),
                        Box::new(Expression::integer(1)),
                    )),
                    Box::new(Expression::integer(5))
                )
            ])
        )
    }

    #[test]
    fn test_complex_3() {
        let tokens = lex("(b i)^5");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::Multiplication(vec![
                    Expression::Variable("b".to_string()),
                    Expression::Complex(
                        Box::new(Expression::integer(0)),
                        Box::new(Expression::integer(1)),
                    )
                ])),
                Box::new(Expression::integer(5))
            )
        )
    }
}

#[cfg(test)]
mod tests_equlity {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_equlity_1() {
        let tokens = lex("a = b");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Equality(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::Variable("b".to_string()))
            )
        );
    }

    #[test]
    fn test_equlity_2() {
        let tokens = lex("a = b = c");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Equality(
                Box::new(Expression::Equality(
                    Box::new(Expression::Variable("a".to_string())),
                    Box::new(Expression::Variable("b".to_string()))
                )),
                Box::new(Expression::Variable("c".to_string()))
            )
        );
    }
}

#[cfg(test)]
mod tests_negation {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::parser::Parser;

    #[test]
    fn test_negation_1() {
        let tokens = lex("-a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Variable("a".to_string())))
        );
    }

    #[test]
    fn test_negation_2() {
        let tokens = lex("(-a)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Variable("a".to_string())))
        );
    }

    #[test]
    fn test_negation_3() {
        let tokens = lex("(-a)^2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Exponentiation(
                Box::new(Expression::Negation(Box::new(Expression::Variable(
                    "a".to_string()
                )))),
                Box::new(Expression::integer(2))
            )
        );
    }

    #[test]
    fn test_negation_4() {
        let tokens = lex("-a^2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::integer(2))
            )))
        );
    }

    #[test]
    fn test_negation_5() {
        let tokens = lex("---a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Negation(Box::new(
                Expression::Negation(Box::new(Expression::Variable("a".to_string())))
            ))))
        );
    }

    #[test]
    fn test_negation_6() {
        let tokens = lex("a--b");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Subtraction(
                Box::new(Expression::Variable("a".to_string())),
                Box::new(Expression::Negation(Box::new(Expression::Variable(
                    "b".to_string()
                ))))
            )
        );
    }

    #[test]
    fn test_negation_7() {
        let tokens = lex("-(a + b)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Addition(vec![
                Expression::Variable("a".to_string()),
                Expression::Variable("b".to_string())
            ])))
        );
    }

    #[test]
    fn test_negation_8() {
        let tokens = lex("-(a + b)^2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Exponentiation(
                Box::new(Expression::Addition(vec![
                    Expression::Variable("a".to_string()),
                    Expression::Variable("b".to_string())
                ])),
                Box::new(Expression::integer(2))
            )))
        );
    }

    #[test]
    fn test_negation_9() {
        let tokens = lex("--(a*b)^2 * 8");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Multiplication(vec![
                Expression::Negation(Box::new(Expression::Negation(Box::new(
                    Expression::Exponentiation(
                        Box::new(Expression::Multiplication(vec![
                            Expression::Variable("a".to_string()),
                            Expression::Variable("b".to_string())
                        ])),
                        Box::new(Expression::integer(2))
                    )
                )))),
                Expression::integer(8)
            ])
        );
    }

    #[test]
    fn test_negation_10() {
        let tokens = lex("--(a + b)^2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::Negation(Box::new(Expression::Negation(Box::new(
                Expression::Exponentiation(
                    Box::new(Expression::Addition(vec![
                        Expression::Variable("a".to_string()),
                        Expression::Variable("b".to_string())
                    ])),
                    Box::new(Expression::integer(2))
                )
            ))))
        );
    }
}

#[cfg(test)]
mod tests_derivative {

    use std::u32;

    use sym_rustic::{
        ast::Expression,
        parser::{ParseError, Parser},
    };

    use crate::lex;

    #[test]
    fn test_derivative_1() {
        let tokens = lex("d/d x (x^2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Derivative(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("x".to_string())),
                Box::new(Expression::integer(2))
            )),
            "x".to_string(),
            1
        )))
    }

    #[test]
    fn test_derivative_2() {
        let tokens = lex("d^1/d x^1 (x^2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Derivative(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("x".to_string())),
                Box::new(Expression::integer(2))
            )),
            "x".to_string(),
            1
        )))
    }

    #[test]
    fn test_derivative_3() {
        let tokens = lex("d^2/d x^2 (x^2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Derivative(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("x".to_string())),
                Box::new(Expression::integer(2))
            )),
            "x".to_string(),
            2
        )))
    }

    #[test]
    fn test_derivative_4() {
        let tokens = lex("d^4/d x_1^4 (x^2 + 2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Derivative(
            Box::new(Expression::Addition(vec![
                Expression::Exponentiation(
                    Box::new(Expression::Variable("x".to_string())),
                    Box::new(Expression::integer(2))
                ),
                Expression::integer(2)
            ])),
            "x_1".to_string(),
            4
        )))
    }

    #[test]
    fn test_derivative_5() {
        let tokens = lex(&format!("d^{} + 2", Into::<u64>::into(u32::MAX) + 2));
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Addition(vec![
            Expression::Exponentiation(
                Box::new(Expression::Variable("d".to_string())),
                Box::new(Expression::integer(Into::<u64>::into(u32::MAX) + 2))
            ),
            Expression::integer(2)
        ])))
    }

    #[test]
    fn test_derivative_6() {
        let tokens = lex("d^a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Exponentiation(
            Box::new(Expression::Variable("d".to_string())),
            Box::new(Expression::Variable("a".to_string()))
        )))
    }

    #[test]
    fn test_derivative_7() {
        let tokens = lex("d^3/2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Division(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("d".to_string())),
                Box::new(Expression::integer(3))
            )),
            Box::new(Expression::integer(2))
        )))
    }
    #[test]
    fn test_derivative_8() {
        let tokens = lex("d^2/a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Division(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("d".to_string())),
                Box::new(Expression::integer(2))
            )),
            Box::new(Expression::Variable("a".to_string()))
        )))
    }
    #[test]
    fn test_derivative_9() {
        let tokens = lex("d^2/d 1");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Division(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("d".to_string())),
                Box::new(Expression::integer(2))
            )),
            Box::new(Expression::Multiplication(vec![
                Expression::Variable("d".to_string()),
                Expression::integer(1)
            ])),
        )))
    }

    #[test]
    fn test_derivative_10() {
        let tokens = lex("d^2/d + 1");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Addition(vec![
            Expression::Division(
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Variable("d".to_string())),
                    Box::new(Expression::integer(2))
                )),
                Box::new(Expression::Variable("d".to_string()))
            ),
            Expression::integer(1)
        ])))
    }

    #[test]
    fn test_derivative_11() {
        let tokens = lex("d^2/d a * 2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::Division(
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Variable("d".to_string())),
                    Box::new(Expression::integer(2))
                )),
                Box::new(Expression::Multiplication(vec![
                    Expression::Variable("d".to_string()),
                    Expression::Variable("a".to_string()),
                ]))
            ),
            Expression::integer(2)
        ])))
    }

    #[test]
    fn test_derivative_12() {
        let tokens = lex("d^2/d x^3");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::Division(
            Box::new(Expression::Exponentiation(
                Box::new(Expression::Variable("d".to_string())),
                Box::new(Expression::integer(2))
            )),
            Box::new(Expression::Multiplication(vec![
                Expression::Variable("d".to_string()),
                Expression::Exponentiation(
                    Box::new(Expression::Variable("x".to_string())),
                    Box::new(Expression::integer(3))
                )
            ])),
        )))
    }

    #[test]
    fn test_derivative_13() {
        let tokens = lex("d^2/d x^2 + 3");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        
        assert!(expr.is_equal(&Expression::Addition(vec![
            Expression::Division(
                Box::new(Expression::Exponentiation(
                    Box::new(Expression::Variable("d".to_string())),
                    Box::new(Expression::integer(2))
                )),
                Box::new(Expression::Multiplication(vec![
                    Expression::Variable("d".to_string()),
                    Expression::Exponentiation(
                        Box::new(Expression::Variable("x".to_string())),
                        Box::new(Expression::integer(2))
                    )
                ]))
            ),
            Expression::integer(3)
        ]),))
    }

    #[test]
    fn test_derivative_14() {
        let tokens = lex("d^2/d x^2 (3)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        
        assert!(expr.is_equal(&Expression::Derivative(
            Box::new(Expression::integer(3)), "x".to_string(), 2)))
        }

    #[test]
    fn test_derivative_15() {
        let tokens = lex("d^2/d x^2 (*)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        
        println!("{:?}", expr);

        assert_eq!(
            expr,
            ParseError::UnexpectedToken("Multiply".to_string(), 11)
        )
    }

    #[test]
    fn test_derivative_16() {
        let tokens = lex("d^2/d x^2 4");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();

        assert_eq!(expr, ParseError::UnexpectedToken("Expected end of input but found Number(\"4\")".to_string(), 10))
    }
}
