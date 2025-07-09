#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sym_rustic::{
        ast::Expression,
        utils::{factorial, gcd, lcm, multinomial_expansion, prime_factors},
    };

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);

        assert_eq!(gcd(12, 24), 12);

        assert_eq!(gcd(12, 13), 1);

        assert_eq!(gcd(12, 14), 2);

        assert_eq!(gcd(12, 15), 3);

        assert_eq!(gcd(12, 0), 12);

        assert_eq!(gcd(0, 10), 10);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);

        assert_eq!(lcm(12, 24), 24);

        assert_eq!(lcm(12, 13), 156);

        assert_eq!(lcm(12, 14), 84);

        assert_eq!(lcm(12, 15), 60);

        assert_eq!(lcm(12, 0), 0);

        assert_eq!(lcm(0, 10), 0);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), Some(HashMap::from([(2, 2), (3, 1)])));

        assert_eq!(prime_factors(13), Some(HashMap::from([(13, 1)])));

        assert_eq!(prime_factors(14), Some(HashMap::from([(2, 1), (7, 1)])));

        assert_eq!(prime_factors(15), Some(HashMap::from([(3, 1), (5, 1)])));

        assert_eq!(prime_factors(0), None);

        assert_eq!(prime_factors(1), None);

        assert_eq!(prime_factors(100), Some(HashMap::from([(2, 2), (5, 2)])));

        assert_eq!(prime_factors(1000), Some(HashMap::from([(2, 3), (5, 3)])));

        assert_eq!(prime_factors(10000), Some(HashMap::from([(2, 4), (5, 4)])));

        assert_eq!(prime_factors(100000), Some(HashMap::from([(2, 5), (5, 5)])));

        assert_eq!(
            prime_factors(1000000),
            Some(HashMap::from([(2, 6), (5, 6)]))
        );

        assert_eq!(
            prime_factors(10000000),
            Some(HashMap::from([(2, 7), (5, 7)]))
        );
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);

        assert_eq!(factorial(1), 1);

        assert_eq!(factorial(2), 2);

        assert_eq!(factorial(3), 6);

        assert_eq!(factorial(4), 24);

        assert_eq!(factorial(5), 120);

        assert_eq!(factorial(6), 720);

        assert_eq!(factorial(7), 5040);
    }

    #[test]
    fn test_multinomial_expansion() {
        assert!(
            multinomial_expansion(
                &vec![
                    Expression::variable("a"),
                    Expression::variable("b"),
                ],
                2
            ).simplify(&mut None).unwrap().is_equal(&Expression::addition(vec![
                Expression::multiplication(vec![
                    Expression::integer(2),
                    Expression::variable("a"),
                    Expression::variable("b"),
                ]),
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(2)
                ),
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(2)
                ),
            ]))
        );

        assert!(
            multinomial_expansion(
                &(vec![
                    Expression::variable("a"),
                    Expression::variable("b"),
                ]),
                3   
            ).simplify(&mut None).unwrap().is_equal(&Expression::addition(vec![
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(3)
                ),
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(3)
                ),
                Expression::multiplication(vec![
                    Expression::integer(3),
                    Expression::exponentiation(
                        Expression::variable("a"),
                        Expression::integer(2)
                    ),
                    Expression::variable("b"),
                ]),
                Expression::multiplication(vec![
                    Expression::integer(3),
                    Expression::variable("a"),
                    Expression::exponentiation(
                        Expression::variable("b"),
                        Expression::integer(2)
                    ),
                ]),
            ]))
        );

        println!("{}", multinomial_expansion(
            &vec![
                Expression::variable("a"),
                Expression::variable("b"),
            ],
            4
        ).simplify(&mut None).unwrap().calculate_tree(0));

        print!("{}", 
        Expression::addition(vec![
            Expression::exponentiation(
                Expression::variable("a"),
                Expression::integer(4)
            ),
            Expression::exponentiation(
                Expression::variable("b"),
                Expression::integer(4)
            ),
            Expression::multiplication(vec![
                Expression::integer(6),
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(2)
                ),
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(2)
                ),
            ]),
            Expression::multiplication(vec![
                Expression::integer(4),
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(3)
                ),
                Expression::variable("b"),
            ]),
            Expression::multiplication(vec![
                Expression::integer(4),
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(3)
                ),
                Expression::variable("a"),
            ]),
        ]).calculate_tree(0));

        // multinomial_expansion(
        //     &vec![
        //         Expression::variable("a"),
        //         Expression::variable("b"),
        //     ],
        //     4
        // ).simplify(&mut None).unwrap().print_console();

        assert!(
            multinomial_expansion(
                &vec![
                    Expression::variable("a"),
                    Expression::variable("b"),
                ],
                4
            ).simplify(&mut None).unwrap().is_equal(&Expression::addition(vec![
                Expression::exponentiation(
                    Expression::variable("a"),
                    Expression::integer(4)
                ),
                Expression::exponentiation(
                    Expression::variable("b"),
                    Expression::integer(4)
                ),
                Expression::multiplication(vec![
                    Expression::integer(6),
                    Expression::exponentiation(
                        Expression::variable("a"),
                        Expression::integer(2)
                    ),
                    Expression::exponentiation(
                        Expression::variable("b"),
                        Expression::integer(2)
                    ),
                ]),
                Expression::multiplication(vec![
                    Expression::integer(4),
                    Expression::exponentiation(
                        Expression::variable("a"),
                        Expression::integer(3)
                    ),
                    Expression::variable("b"),
                ]),
                Expression::multiplication(vec![
                    Expression::integer(4),
                    Expression::exponentiation(
                        Expression::variable("b"),
                        Expression::integer(3)
                    ),
                    Expression::variable("a"),
                ]),
            ]))
        );
    }
}
