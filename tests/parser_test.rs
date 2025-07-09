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
mod tests_bad_lex {
    use crate::lex;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::{ParseError, Parser};

    #[test]
    fn test_bad_lex_1() {
        let tokens: Vec<Token> = lex("é");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(
            expr,
            ParseError::UnexpectedToken("Unsupported: é".to_string(), 0)
        );
    }
}

#[cfg(test)]
mod tests_number {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::{ParseError, Parser};

    #[test]
    fn test_number_simple() {
        let tokens: Vec<Token> = lex("42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::integer(42));
    }

    #[test]
    fn test_number_plus() {
        let tokens: Vec<Token> = lex("+ 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::integer(42));
    }

    #[test]
    fn test_number_minus() {
        let tokens: Vec<Token> = lex("- 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::negation(Expression::integer(42))
        );
    }

    #[test]
    fn test_number_decimal() {
        let tokens: Vec<Token> = lex("43.55");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::rational(4355, 100));
    }

    #[test]
    fn test_number_decimal_negative() {
        let tokens: Vec<Token> = lex("-43.55");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::negation(Expression::rational(4355, 100))
        );
    }

    #[test]
    fn test_number_decimal_no_decimal() {
        let tokens: Vec<Token> = lex("43.");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(expr, ParseError::UnexpectedEndOfInput(2));
    }

    #[test]
    fn test_number_decimal_no_leading() {
        let tokens: Vec<Token> = lex(".55");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::rational(55, 100));
    }

    #[test]
    fn test_number_decimal_leading_zero() {
        let tokens: Vec<Token> = lex("0.55");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::rational(55, 100));
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
            Expression::addition(vec![Expression::integer(42), Expression::integer(42)])
        );
    }

    #[test]
    fn test_addition_2() {
        let tokens: Vec<Token> = lex("38 + 40 + 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::addition(vec![
                Expression::addition(vec![Expression::integer(38), Expression::integer(40),]),
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
            Expression::addition(vec![
                Expression::addition(vec![Expression::integer(40), Expression::integer(42)]),
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
            Expression::addition(vec![
                Expression::integer(38),
                Expression::addition(vec![Expression::integer(40), Expression::integer(42)])
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
    fn test_multiplication_simple() {
        let tokens: Vec<Token> = lex("42 * 42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![Expression::integer(42), Expression::integer(42)])
        );
    }

    #[test]
    fn test_multiplication_simple_2() {
        let tokens: Vec<Token> = lex("a * b * c");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::multiplication(vec![
                    Expression::variable("a"),
                    Expression::variable("b"),
                ]),
                Expression::variable("c")
            ])
        );
    }

    #[test]
    fn test_multiplication_parenthesis() {
        let tokens: Vec<Token> = lex("38 * (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::integer(38),
                Expression::multiplication(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }

    #[test]
    fn test_multiplication_parenthesis_negation() {
        let tokens: Vec<Token> = lex("(40 * 42) * -38");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::multiplication(vec![Expression::integer(40), Expression::integer(42)]),
                Expression::negation(Expression::integer(38))
            ])
        );
    }

    #[test]
    fn test_multiplication_parenthesis_implicit_multiplication_number() {
        let tokens: Vec<Token> = lex("(40 * 42) 38");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::multiplication(vec![Expression::integer(40), Expression::integer(42)]),
                Expression::integer(38)
            ])
        );
    }

    #[test]
    fn test_multiplication_parenthesis_implicit_multiplication_number_2() {
        let tokens: Vec<Token> = lex("8 (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::integer(8),
                Expression::multiplication(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }

    #[test]
    fn test_multiplication_parenthesis_implicit_multiplication_variable() {
        let tokens: Vec<Token> = lex("a (40 * 42)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::variable("a"),
                Expression::multiplication(vec![Expression::integer(40), Expression::integer(42)])
            ])
        );
    }

    #[test]
    fn test_multiplication_implicit_multiplication_parenthesis() {
        let tokens: Vec<Token> = lex(" (a + b)(a - b)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::addition(vec![
                    Expression::variable("a"),
                    Expression::variable("b")
                ]),
                Expression::subtraction(
                    Expression::variable("a"),
                    Expression::variable("b")
                )
            ])
        );
    }

    #[test]
    fn test_multiplication_division() {
        let tokens: Vec<Token> = lex(" a * b / c * a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::division(
                    Expression::multiplication(vec![
                        Expression::variable("a"),
                        Expression::variable("b"),
                    ]),
                    Expression::variable("c")
                ),
                Expression::variable("a")
            ])
        );
    }

    #[test]
    fn test_multiplication_division_2() {
        let tokens: Vec<Token> = lex("a / b * c / a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::multiplication(vec![
                    Expression::division(
                        Expression::variable("a"),
                        Expression::variable("b"),
                    ),
                    Expression::variable("c")
                ]),
                Expression::variable("a")
            )
        );
    }

    #[test]
    fn test_multiplication_implcit_multiplication_variable() {
        let tokens: Vec<Token> = lex("a b c");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::variable("a"),
                Expression::multiplication(vec![
                    Expression::variable("b"),
                    Expression::variable("c")
                ])
            ])
        );
    }

    #[test]
    fn test_multiplication_implcit_multiplication_variable_2() {
        let tokens: Vec<Token> = lex("a (b c)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::variable("a"),
                Expression::multiplication(vec![
                    Expression::variable("b"),
                    Expression::variable("c")
                ])
            ])
        );
    }

    #[test]
    fn test_multiplication_implcit_multiplication() {
        let tokens: Vec<Token> = lex("3 a (b c)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::integer(3),
                Expression::multiplication(vec![
                    Expression::variable("a"),
                    Expression::multiplication(vec![
                        Expression::variable("b"),
                        Expression::variable("c")
                    ])
                ])
            ])
        );
    }
}

#[cfg(test)]
mod tests_division {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::lexer::Token;
    use sym_rustic::parser::{ParseError, Parser};

    #[test]
    fn test_division_simple() {
        let tokens: Vec<Token> = lex("42 / 2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::integer(42),
                Expression::integer(2)
            )
        );
    }

    #[test]
    fn test_division_no_spaces() {
        let tokens: Vec<Token> = lex("42/2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::integer(42),
                Expression::integer(2)
            )
        );
    }

    #[test]
    fn test_division_chained() {
        let tokens: Vec<Token> = lex("a / b / c");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::division(
                    Expression::variable("a"),
                    Expression::variable("b")
                ),
                Expression::variable("c")
            )
        );
    }

    #[test]
    fn test_division_chained_2() {
        let tokens: Vec<Token> = lex("a / b / c / d");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::division(
                    Expression::division(
                        Expression::variable("a"),
                        Expression::variable("b")
                    ),
                    Expression::variable("c")
                ),
                Expression::variable("d")
            )
        );
    }

    #[test]
    fn test_division_parentheses() {
        let tokens: Vec<Token> = lex("a / (b / c)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::variable("a"),
                Expression::division(
                    Expression::variable("b"),
                    Expression::variable("c")
                )
            )
        );
    }

    #[test]
    fn test_division_negative_numerator() {
        let tokens: Vec<Token> = lex("-42 / 2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::negation(Expression::integer(42)),
                Expression::integer(2)
            )
        );
    }

    #[test]
    fn test_division_negative_denominator() {
        let tokens: Vec<Token> = lex("42 / -2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::division(
                Expression::integer(42),
                Expression::negation(Expression::integer(2))
            )
        );
    }

    #[test]
    fn test_division_multiplication_denominator() {
        let tokens: Vec<Token> = lex("1 / 3 * 4");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::division(
                    Expression::integer(1),
                    Expression::integer(3),
                ),
                Expression::integer(4),
            ])
        );
    }

    #[test]
    fn test_division_missing_denominator() {
        let tokens: Vec<Token> = lex("42 /");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(ParseError::UnexpectedEndOfInput(3), expr);
    }

    #[test]
    fn test_division_leading_division_operator() {
        let tokens: Vec<Token> = lex("/42");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(ParseError::UnexpectedToken("/".to_string(), 0), expr);
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
            Expression::exponentiation(
                Expression::integer(42),
                Expression::integer(42)
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
            Expression::exponentiation(
                Expression::integer(38),
                Expression::multiplication(vec![
                    Expression::integer(40),
                    Expression::integer(42)
                ])
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
            Expression::exponentiation(
                Expression::multiplication(vec![
                    Expression::integer(40),
                    Expression::integer(42)
                ]),
                Expression::integer(38)
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
            Expression::exponentiation(
                Expression::integer(42),
                Expression::exponentiation(
                    Expression::integer(42),
                    Expression::integer(42)
                )
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
            Expression::exponentiation(
                Expression::integer(42),
                Expression::exponentiation(
                    Expression::integer(42),
                    Expression::integer(42)
                )
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
            Expression::exponentiation(
                Expression::exponentiation(
                    Expression::integer(42),
                    Expression::integer(42)
                ),
                Expression::integer(42)
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
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::division(
                    Expression::integer(1),
                    Expression::integer(2)
                )
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
            Expression::division(
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(1)
                ),
                Expression::integer(2)
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
    use sym_rustic::parser::{ParseError, Parser};

    #[test]
    fn test_literal_1() {
        let tokens: Vec<Token> = lex("a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("a"))
    }

    #[test]
    fn test_literal_2() {
        let tokens = lex("abc");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("abc"))
    }

    #[test]
    fn test_literal_3() {
        let tokens: Vec<Token> = lex("a_1");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("a_1"))
    }

    #[test]
    fn test_literal_4() {
        let tokens: Vec<Token> = lex("a_a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("a_a"))
    }

    #[test]
    fn test_literal_5() {
        let tokens: Vec<Token> = lex("a__a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();
        assert_eq!(expr, ParseError::InvalidVariableFormat(2))
    }

    #[test]
    fn test_literal_6() {
        let tokens: Vec<Token> = lex("a_a_a_b");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("a_a_a_b"))
    }

    #[test]
    fn test_literal_7() {
        let tokens: Vec<Token> = lex("a_123_12");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("a_123_12"))
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
        assert_eq!(expr, Expression::variable("E"))
    }

    #[test]
    fn test_literal_12() {
        let tokens: Vec<Token> = lex("e_3");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("e_3"))
    }

    #[test]
    fn test_literal_13() {
        let tokens: Vec<Token> = lex("pi_a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("pi_a"))
    }

    #[test]
    fn test_literal_14() {
        let tokens: Vec<Token> = lex("tau_628");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(expr, Expression::variable("tau_628"))
    }
}

#[cfg(test)]
mod tests_functions {
    use crate::lex;
    use sym_rustic::ast::Expression;
    use sym_rustic::parser::{ParseError, Parser};

    #[test]
    fn test_function_1() {
        let tokens = lex("sin(3 + 8)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::sin(
                Expression::addition(vec![
                    Expression::integer(3),
                    Expression::integer(8)
                ])
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
            Expression::multiplication(vec![
                Expression::variable("sin"),
                Expression::addition(vec![Expression::integer(3), Expression::integer(8)])
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
            Expression::log(
                
                    Expression::integer(10),
                    Expression::variable("a")
                
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
        assert_eq!(expr, ParseError::UnexpectedToken(",".to_string(), 5))
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
            Expression::addition(vec![
                Expression::variable("a"),
                Expression::multiplication(vec![
                    Expression::variable("b"),
                    Expression::complex(
                        Expression::integer(0),
                        Expression::integer(1),
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
            Expression::multiplication(vec![
                Expression::variable("b"),
                Expression::exponentiation(
                    Expression::complex(
                        Expression::integer(0),
                        Expression::integer(1),
                    ),
                    Expression::integer(5)
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
            Expression::exponentiation(
                Expression::multiplication(vec![
                    Expression::variable("b"),
                    Expression::complex(
                        Expression::integer(0),
                        Expression::integer(1),
                    )
                ]),
                Expression::integer(5)
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
            Expression::equality(
                Expression::variable("a"),
                Expression::variable("b")
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
            Expression::equality(
                Expression::equality(
                    Expression::variable("a"),
                    Expression::variable("b")
                ),
                Expression::variable("c")
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
            Expression::negation(Expression::variable("a"))
        );
    }

    #[test]
    fn test_negation_2() {
        let tokens = lex("(-a)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::negation(Expression::variable("a"))
        );
    }

    #[test]
    fn test_negation_3() {
        let tokens = lex("(-a)^2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::exponentiation(
                Expression::negation(Expression::variable(
                    "a"
                )),
                Expression::integer(2)
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
            Expression::negation(Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(2)
            ))
        );
    }

    #[test]
    fn test_negation_5() {
        let tokens = lex("---a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::negation(Expression::negation(
                Expression::negation(Expression::variable("a"))
            ))
        );
    }

    #[test]
    fn test_negation_6() {
        let tokens = lex("a--b");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::subtraction(
                Expression::variable("a"),
                Expression::negation(Expression::variable(
                    "b"
                )))
        );
    }

    #[test]
    fn test_negation_7() {
        let tokens = lex("-(a + b)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::negation(Expression::addition(vec![
                Expression::variable("a"),
                Expression::variable("b")
            ]))
        );
    }

    #[test]
    fn test_negation_8() {
        let tokens = lex("-(a + b)^2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::negation(Expression::exponentiation(
                Expression::addition(vec![
                    Expression::variable("a"),
                    Expression::variable("b")
                ]),
                Expression::integer(2)
            ))
        );
    }

    #[test]
    fn test_negation_9() {
        let tokens = lex("--(a*b)^2 * 8");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();
        assert_eq!(
            expr,
            Expression::multiplication(vec![
                Expression::negation(Expression::negation(
                    Expression::exponentiation(
                        Expression::multiplication(vec![
                            Expression::variable("a"),
                            Expression::variable("b")
                        ]),
                        Expression::integer(2)
                    )
                )),
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
            Expression::negation(Expression::negation(
                Expression::exponentiation(
                    Expression::addition(vec![
                        Expression::variable("a"),
                        Expression::variable("b")
                    ]),
                    Expression::integer(2)
                )
            ))
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
        let tokens = lex("d/dx (x^2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::derivative(
            Expression::exponentiation(
                Expression::variable("x"),
                Expression::integer(2)
            ),
            "x",
            1
        )))
    }

    #[test]
    fn test_derivative_2() {
        let tokens = lex("d^1/dx^1 (x^2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::derivative(
            Expression::exponentiation(
                Expression::variable("x"),
                Expression::integer(2)
            ),
            "x",
            1
        )))
    }

    #[test]
    fn test_derivative_3() {
        let tokens = lex("d^2/dx^2 (x^2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::derivative(
            Expression::exponentiation(
                Expression::variable("x"),
                Expression::integer(2)
            ),
            "x",
            2
        )))
    }

    #[test]
    fn test_derivative_4() {
        let tokens = lex("d^4/dx_1^4 (x^2 + 2)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::derivative(
            Expression::addition(vec![
                Expression::exponentiation(
                    Expression::variable("x"),
                    Expression::integer(2)
                ),
                Expression::integer(2)
            ]),
            "x_1",
            4
        )))
    }

    #[test]
    fn test_derivative_5() {
        let tokens = lex(&format!("d^{} + 2", Into::<u64>::into(u32::MAX) + 2));
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::addition(vec![
            Expression::exponentiation(
                Expression::variable("d"),
                Expression::integer(Into::<u64>::into(u32::MAX) + 2)
            ),
            Expression::integer(2)
        ])))
    }

    #[test]
    fn test_derivative_6() {
        let tokens = lex("d^a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::exponentiation(
            Expression::variable("d"),
            Expression::variable("a")
        )))
    }

    #[test]
    fn test_derivative_7() {
        let tokens = lex("d^3/2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::division(
            Expression::exponentiation(
                Expression::variable("d"),
                Expression::integer(3)
            ),
            Expression::integer(2)
        )))
    }
    #[test]
    fn test_derivative_8() {
        let tokens = lex("d^2/a");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::division(
            Expression::exponentiation(
                Expression::variable("d"),
                Expression::integer(2)
            ),
            Expression::variable("a")
        )))
    }
    #[test]
    fn test_derivative_9() {
        let tokens = lex("d^2/d 1");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::division(
            Expression::exponentiation(
                Expression::variable("d"),
                Expression::integer(2)
            ),
            Expression::multiplication(vec![
                Expression::variable("d"),
                Expression::integer(1)
            ]),
        )))
    }

    #[test]
    fn test_derivative_10() {
        let tokens = lex("d^2/d + 1");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::addition(vec![
            Expression::division(
                Expression::exponentiation(
                    Expression::variable("d"),
                    Expression::integer(2)
                ),
                Expression::variable("d")
            ),
            Expression::integer(1)
        ])))
    }

    #[test]
    fn test_derivative_11() {
        let tokens = lex("d^2/d a * 2");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::multiplication(vec![
            Expression::division(
                Expression::exponentiation(
                    Expression::variable("d"),
                    Expression::integer(2)
                ),
                Expression::multiplication(vec![
                    Expression::variable("d"),
                    Expression::variable("a"),
                ])
            ),
            Expression::integer(2)
        ])))
    }

    #[test]
    fn test_derivative_12() {
        let tokens = lex("d^2/d x^3");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::division(
            Expression::exponentiation(
                Expression::variable("d"),
                Expression::integer(2)
            ),
            Expression::multiplication(vec![
                Expression::variable("d"),
                Expression::exponentiation(
                    Expression::variable("x"),
                    Expression::integer(3)
                )
            ]),
        )))
    }

    #[test]
    fn test_derivative_13() {
        let tokens = lex("d^2/dx^2 + 3");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::derivative(Expression::integer(3), "x", 2)))
    }

    #[test]
    fn test_derivative_14() {
        let tokens = lex("d^2/dx^2 (3)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap();

        assert!(expr.is_equal(&Expression::derivative(
            Expression::integer(3),
            "x",
            2
        )))
    }

    #[test]
    fn test_derivative_15() {
        let tokens = lex("d^2/dx^2 (*)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expression().unwrap_err();

        println!("{:?}", expr);

        assert_eq!(expr, ParseError::UnexpectedToken("*".to_string(), 9))
    }
}
