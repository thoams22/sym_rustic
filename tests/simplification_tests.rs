use sym_rustic::{ast::{Expression, SimplifyError}, lexer::{Lexer, Token}, parser::Parser};

fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    tokens
}

fn parse(input: Vec<Token>) -> Expression {
    let mut parser = Parser::new(&input);
    parser.parse_expression().unwrap()
}

fn simplify(
    mut expr: Expression,
    explanation: &mut Option<Vec<String>>,
) -> Result<Expression, SimplifyError> {
    expr.simplify(explanation)
}

#[cfg(test)]
mod tests_additions {
    use crate::{lex, parse, simplify};
    use sym_rustic::ast::Expression;
    use sym_rustic::ast::function::Function;

    #[test]
    fn test_addition_1() {
        let expr = simplify(parse(lex("42 + 42")), &mut None).unwrap();
        assert_eq!(expr, Expression::integer(84));
    }

    #[test]
    fn test_addition_2() {
        let expr = simplify(parse(lex("a + a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::integer(2),
            Expression::variable("a")
        ])));
    }

    #[test]
    fn test_addition_3() {
        let expr = simplify(parse(lex("a + a + a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::integer(3),
            Expression::variable("a" )
        ])));
    }

    #[test]
    fn test_addition_4() {
        let expr = simplify(parse(lex("a + 2a + 4a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::integer(7),
            Expression::variable("a" )
        ])));
    }

    #[test]
    fn test_addition_5() {
        let expr = simplify(parse(lex("a - a + a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::variable("a" )));
    }

    #[test]
    fn test_addition_6() {
        let expr = simplify(parse(lex("a + a - a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::variable("a" )));
    }

    #[test]
    fn test_addition_7() {
        let expr = simplify(parse(lex("2a - a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::variable("a" )));
    }

    #[test]
    fn test_addition_8() {
        let expr = simplify(parse(lex("a - 2a + a")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::integer(0)));
    }

    #[test]
    fn test_addition_9() {
        let expr = simplify(parse(lex("-2a + a + a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::integer(0)));
    }

    #[test]
    fn test_addition_10() {
        let expr = simplify(parse(lex("a + a*i")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::complex(
            Expression::variable("a" ),
            Expression::variable("a" )
        )));
    }

    #[test]
    fn test_addition_11() {
        let expr = simplify(parse(lex("cos(x) + cos(x)")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::integer(2),
            Expression::Function(Function::Cos, vec![Expression::variable("x" )]),
        ])));
    }

    #[test]
    fn test_addition_12() {
        let expr = simplify(parse(lex("cos(x) - cos(x)")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::integer(0),));
    }

    #[test]
    fn test_addition_13() {
        let expr = simplify(parse(lex("a + b i + c + d i")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::complex(
            Expression::Addition(vec![
                Expression::variable("a" ),
                Expression::variable("c" )
            ]),
            Expression::Addition(vec![
                Expression::variable("b" ),
                Expression::variable("d" )
            ]))
        ))
    }

    #[test]
    fn test_addition_14() {
        let expr = simplify(parse(lex("cos(x) + cos(y)")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Addition(vec![
            Expression::cos(Expression::variable("x" )),
            Expression::cos(Expression::variable("y" )),
        ])));
    }

    #[test]
    fn test_addition_15() {
        let expr = simplify(parse(lex("1 - 2")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::negation(Expression::integer(1))));
    }

    #[test]
    fn test_addition_16() {
        let expr = simplify(parse(lex("- 1 - 2 + 7")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::integer(4)));
    }

    #[test]
    fn test_addition_17() {
        let expr = simplify(parse(lex("2 - 4")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::negation(Expression::integer(2))));
    }


}

mod tests_multiplication {
    use crate::{lex, parse, simplify};
    use sym_rustic::ast::Expression;

    #[test]
    fn test_multiplication_1() {
        let expr = simplify(parse(lex("42 * 42")), &mut None).unwrap();
        assert_eq!(expr, Expression::integer(1764));
    }

    #[test]
    fn test_multiplication_2() {
        let expr = simplify(parse(lex("a * a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::exponentiation(
            Expression::variable("a" ),
            Expression::integer(2))
        ));
    }

    #[test]
    fn test_multiplication_3() {
        let expr = simplify(parse(lex("a * a * a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::exponentiation(
            Expression::variable("a" ),
            Expression::integer(3))
        ));
    }

    #[test]
    fn test_multiplication_4() {
        let expr = simplify(parse(lex("a * 2a * 4a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::integer(8),
            Expression::exponentiation(
                Expression::variable("a" ),
                Expression::integer(3))
            
        ])));
    }

    #[test]
    fn test_multiplication_5() {
        let mut log: Option<Vec<String>> = Some(Vec::new());

        let expr = simplify(parse(lex("a * a / a")), &mut log).unwrap();
        if let Some(logged) = log {
            for line in logged {
                println!("{}", line);
            }
        }
        assert!(expr.is_equal(&Expression::variable("a" )));
    }

    #[test]
    fn test_multiplication_6() {
        let expr = simplify(parse(lex("a * a / a * a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::exponentiation(
            Expression::variable("a" ),
            Expression::integer(2))
        ));
    }

    #[test]
    fn test_multiplication_7() {
        let expr = simplify(parse(lex("a * a / a / a")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::integer(1)));
    }

    #[test]
    fn test_multiplication_8() {
        let expr = simplify(parse(lex("(a * a) / (a * a)")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::integer(1)));
    }

    #[test]
    fn test_multiplication_9() {
        let expr = simplify(parse(lex("cos(x) * cos(x)")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::exponentiation(
            Expression::cos(
                Expression::variable("x" )
            ),
            Expression::integer(2))
        ));
    }

    #[test]
    fn test_multiplication_11() {
        let expr = simplify(parse(lex("cos(x) * cos(y)")), &mut None).unwrap();
        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::cos(Expression::variable("x" )),
            Expression::cos(Expression::variable("y" )),
        ])));
    }

    #[test]
    fn test_multiplication_12() {
        let expr = simplify(parse(lex("a * (a + b i)")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::complex(
            Expression::exponentiation(
                Expression::variable("a" ),
                Expression::integer(2))
            ,
            Expression::Multiplication(vec![
                Expression::variable("a" ),
                Expression::variable("b" )
            ])
        )));
    }

    #[test]
    fn test_multiplication_14() {
        let expr = simplify(parse(lex("(a + b i) * (c + d i)")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::complex(
            Expression::Addition(vec![
                Expression::Multiplication(vec![
                    Expression::variable("a" ),
                    Expression::variable("c" )
                ]),
                Expression::negation(Expression::Multiplication(vec![
                    Expression::variable("b" ),
                    Expression::variable("d" )
                ]))
            ]),
            Expression::Addition(vec![
                Expression::Multiplication(vec![
                    Expression::variable("a" ),
                    Expression::variable("d" )
                ]),
                Expression::Multiplication(vec![
                    Expression::variable("b" ),
                    Expression::variable("c" )
                ])
            ]))
        ));
    }

    #[test]
    fn test_multiplication_15() {
        let expr = simplify(parse(lex("(a + b i) * (a - b i)")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::Addition(vec![
            Expression::exponentiation(
                Expression::variable("a" ),
                Expression::integer(2)
            ),
            Expression::exponentiation(
                Expression::variable("b" ),
                Expression::integer(2)
            )
        ])));
    }

    #[test]
    fn test_multiplication_16() {
        let expr = simplify(parse(lex("a (b + c)")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::Addition(vec![
            Expression::Multiplication(vec![
                Expression::variable("a" ),
                Expression::variable("b" )
            ]),
            Expression::Multiplication(vec![
                Expression::variable("a" ),
                Expression::variable("c" )
            ])
        ])));
    }

    #[test]
    fn test_multiplication_17() {
        let expr = simplify(parse(lex(" c/ (a + b i)")), &mut None).unwrap();

        assert!(expr.is_equal(&
            Expression::complex(
                Expression::Multiplication(vec![
                    Expression::variable("c" ),
                    Expression::variable("a" ),
                    Expression::exponentiation(Expression::Addition(vec![
                        Expression::exponentiation(
                            Expression::variable("a" ),
                            Expression::integer(2)
                        ),
                        Expression::exponentiation(
                            Expression::variable("b" ),
                            Expression::integer(2)
                        )
                    ]), 
                Expression::negation(Expression::integer(1)))
                ]),
                Expression::Multiplication(vec![
                    Expression::variable("c" ),
                    Expression::variable("b" ),
                    Expression::exponentiation(Expression::Addition(vec![
                        Expression::exponentiation(
                            Expression::variable("a" ),
                            Expression::integer(2)
                        ),
                        Expression::exponentiation(
                            Expression::variable("b" ),
                            Expression::integer(2)
                        )
                    ]), 
                Expression::negation(Expression::integer(1)))
                ]),
            ),
            
        ));
    }

    #[test]
    fn test_multiplication_18() {
        let expr = simplify(parse(lex(" b/ sqrt(3) ")), &mut None).unwrap();

        assert!(expr.is_equal(&
            Expression::Multiplication(vec![
                Expression::variable("b" ),
                Expression::exponentiation(Expression::sqrt(Expression::integer(3)), Expression::negation(Expression::integer(1)))
            ]),
        ));
    }

    #[test]
    fn test_multiplication_19() {
        let expr = simplify(parse(lex("(a + b)/(a)")), &mut None).unwrap();

        assert!(expr.is_equal(&
            Expression::Addition(vec![
                Expression::Multiplication(vec![
                    Expression::exponentiation(Expression::variable("a" ), Expression::negation(Expression::integer(1))),
                    Expression::variable("b" )]
                ),
                Expression::integer(1)
            ]),
        ));
    }

    #[test]
    fn test_multiplication_20() {
        let expr = simplify(parse(lex("(a * b) / a")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::variable("b" )));
    }

    #[test]
    fn test_multiplication_21() {
        let expr = simplify(parse(lex("a * b / a")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::variable("b" )));
    }

    #[test]
    fn test_multiplication_22() {
        let expr = simplify(parse(lex("(a * b) / (a * c)")), &mut None).unwrap();

        assert!(expr.is_equal(&Expression::Multiplication(vec![
            Expression::variable("b" ),
            Expression::exponentiation(Expression::variable("c" ), Expression::negation(Expression::integer(1)))
        ]))
        )
    }
}
