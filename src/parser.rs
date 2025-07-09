use crate::ast::Expression;
use crate::lexer::Token;
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(String, usize),
    UnexpectedEndOfInput(usize),
    InvalidNumberFormat(usize),
    InvalidVariableFormat(usize),
    // Name of fct, args_number, position
    InvalidFunctionFormat(String, usize, usize),
    DerivativeFailed,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // #[allow(dead_code)]
    // fn previous_token(&self) -> Option<&Token> {
    //     self.tokens.get(self.position - 1)
    // }

    // #[allow(dead_code)]
    // fn next_token(&self) -> Option<&Token> {
    //     self.tokens.get(self.position + 1)
    // }

    // fn peek_token(&self, offset: usize) -> Option<&Token> {
    //     self.tokens.get(self.position + offset)
    // }

    fn pass_whitespace(&mut self) {
        while let Some(Token::WhiteSpace) = self.current_token() {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_binary(None, 0)?;

        if let Some(token) = self.current_token() {
            Err(ParseError::UnexpectedToken(
                format!("Expected end of input but found {}", token),
                self.position,
            ))
        } else {
            Ok(expr)
        }
    }

    fn parse_binary(
        &mut self,
        previous_left: Option<Expression>,
        past_precedence: u8,
    ) -> Result<Expression, ParseError> {
        let mut left_expr = if let Some(expr) = previous_left {
            expr
        } else {
            self.parse_primary()?
        };

        while let Some(token) = self.current_token().cloned() {
            let precedence = match token {
                Token::Equals => 1,
                Token::Plus | Token::Minus => 2,
                Token::Multiply | Token::Divide => 3,
                Token::Caret => 5,
                _ => break,
            };

            if precedence < past_precedence || (precedence != 5 && precedence == past_precedence) {
                break;
            }

            self.advance();

            let right_expr = self.parse_binary(None, precedence)?;

            left_expr = match token {
                Token::Plus => Expression::addition(vec![left_expr, right_expr]),
                Token::Minus => Expression::subtraction(left_expr, right_expr),
                Token::Multiply => Expression::multiplication(vec![left_expr, right_expr]),
                Token::Divide => Expression::division(left_expr, right_expr),
                Token::Caret => {
                    Expression::exponentiation(left_expr, right_expr)
                }
                Token::Equals => Expression::equality(left_expr, right_expr),
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        format!("{}", token),
                        self.position,
                    ));
                }
            };
        }

        Ok(left_expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        while let Some(Token::WhiteSpace) = self.current_token() {
            self.advance();
        }
        let parsed = match self.current_token() {
            Some(Token::Plus) => {
                self.advance();
                let expr = self.parse_primary()?;
                while let Some(Token::WhiteSpace) = self.current_token() {
                    self.advance();
                }
                // Handle implicit multiplication
                if matches!(
                    self.current_token(),
                    Some(&Token::LeftParen) | Some(&Token::Literal(_))
                ) {
                    Ok(Expression::multiplication(vec![
                        expr,
                        self.parse_binary(None, 3)?,
                    ]))
                } else {
                    Ok(expr)
                }
            }
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_binary(None, 4)?;
                while let Some(Token::WhiteSpace) = self.current_token() {
                    self.advance();
                }
                // Handle implicit multiplication
                if matches!(
                    self.current_token(),
                    Some(&Token::LeftParen) | Some(&Token::Literal(_))
                ) {
                    Ok(Expression::multiplication(vec![
                        expr,
                        self.parse_binary(None, 3)?,
                    ]))
                } else {
                    Ok(Expression::negation(expr))
                }
            }
            Some(Token::Number(value)) => self.parse_number(value.clone()),
            Some(Token::Literal(value)) => {
                let expr = self.parse_literal(value.clone())?;
                while let Some(Token::WhiteSpace) = self.current_token() {
                    self.advance();
                }
                // Handle implicit multiplication
                if matches!(
                    self.current_token(),
                    Some(&Token::LeftParen) | Some(&Token::Literal(_)) | Some(&Token::Number(_))
                ) {
                    Ok(Expression::multiplication(vec![
                        expr,
                        self.parse_binary(None, 3)?,
                    ]))
                } else {
                    Ok(expr)
                }
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_binary(None, 0)?;
                if let Some(Token::RightParen) = self.current_token() {
                    self.advance();
                    while let Some(Token::WhiteSpace) = self.current_token() {
                        self.advance();
                    }
                    // Handle implicit multiplication
                    if matches!(
                        self.current_token(),
                        Some(&Token::LeftParen)
                            | Some(&Token::Literal(_))
                            | Some(&Token::Number(_))
                    ) {
                        Ok(Expression::multiplication(vec![
                            expr,
                            self.parse_binary(None, 3)?,
                        ]))
                    } else {
                        Ok(expr)
                    }
                } else {
                    Err(ParseError::UnexpectedToken(
                        format!("{}", self.current_token().unwrap()),
                        self.position,
                    ))
                }
            }
            Some(Token::Dot) => {
                // Handle decimal number with no leading zero like .55
                self.advance();
                match self.current_token() {
                    Some(Token::Number(value)) => match value.parse::<u64>() {
                        Ok(denominator) => {
                            self.advance();
                            Ok(Expression::rational(
                                denominator,
                                10_u64.pow(denominator.to_string().len() as u32),
                            ))
                        }
                        Err(_) => Err(ParseError::InvalidNumberFormat(self.position)),
                    },
                    Some(token) => {
                        Err(ParseError::UnexpectedToken(
                            format!("{}", token),
                            self.position,
                        ))
                    }
                    None => Err(ParseError::UnexpectedEndOfInput(self.position)),
                }
            }
            Some(token) => Err(ParseError::UnexpectedToken(
                format!("{}", token),
                self.position,
            )),
            None => Err(ParseError::UnexpectedEndOfInput(self.position)),
        };

        while let Some(Token::WhiteSpace) = self.current_token() {
            self.advance();
        }

        parsed
    }

    fn parse_number(&mut self, variable: String) -> Result<Expression, ParseError> {
        self.advance();
        match variable.parse::<u64>() {
            Ok(numerator) => {
                // Handle decimal point
                let number = if let Some(Token::Dot) = self.current_token() {
                    self.advance();
                    match self.current_token() {
                        Some(Token::Number(value)) => match value.parse::<u64>() {
                            Ok(denominator) => {
                                self.advance();
                                let power = 10_u64.pow(denominator.to_string().len() as u32);
                                Expression::rational(numerator * power + denominator, power)
                            }
                            Err(_) => return Err(ParseError::InvalidNumberFormat(self.position)),
                        },
                        Some(token) => {
                            return Err(ParseError::UnexpectedToken(
                                format!("{}", token),
                                self.position,
                            ));
                        }
                        None => return Err(ParseError::UnexpectedEndOfInput(self.position)),
                    }
                } else {
                    Expression::integer(numerator)
                };

                while let Some(Token::WhiteSpace) = self.current_token() {
                    self.advance();
                }
                // Handle implicit multiplication
                if matches!(
                    self.current_token(),
                    Some(&Token::LeftParen) | Some(&Token::Literal(_))
                ) {
                    Ok(Expression::multiplication(vec![
                        number,
                        self.parse_binary(None, 3)?,
                    ]))
                } else {
                    Ok(number)
                }
            }
            Err(_) => Err(ParseError::InvalidNumberFormat(self.position)),
        }
    }

    fn parse_literal(&mut self, variable: String) -> Result<Expression, ParseError> {
        self.advance();
        // Variables
        match self.current_token() {
            Some(Token::Underscore) => {
                let var = self.parse_variable(variable)?;
                Ok(Expression::Variable(var))
            }
            Some(Token::Caret) | Some(Token::Divide) if variable == "d" => {
                let position = self.position;
                match self.parse_derivative() {
                    Ok(derivative) => Ok(derivative),
                    Err(_) => {
                        self.position = position;
                        Ok(Expression::Variable("d".to_string()))
                    }
                }
            }
            Some(Token::LeftParen) => self.parse_functions(variable),
            _ => match variable.as_str() {
                "tau" => Ok(Expression::tau()),
                "pi" => Ok(Expression::pi()),
                "e" => Ok(Expression::e()),
                "i" => Ok(Expression::complex(
                    Expression::integer(0),
                    Expression::integer(1),
                )),
                _ => Ok(Expression::Variable(variable)),
            },
        }
    }

    fn parse_variable(&mut self, mut variable: String) -> Result<String, ParseError> {
        while let Some(Token::Underscore) = self.current_token() {
            self.advance();
            variable += "_";
            if let Some(token) = self.current_token() {
                match token {
                    Token::Number(value) | Token::Literal(value) => {
                        variable += value;
                        self.advance();
                    }
                    _ => return Err(ParseError::InvalidVariableFormat(self.position)),
                }
            } else {
                return Err(ParseError::UnexpectedEndOfInput(self.position));
            }
        }
        Ok(variable)
    }

    fn parse_derivative(&mut self) -> Result<Expression, ParseError> {
        let mut order: Option<u32> = None;

        if let Some(Token::Caret) = self.current_token() {
            self.advance();
            // d^
            if let Some(Token::Number(num_str)) = self.current_token().cloned() {
                // d^num
                match num_str.parse::<u32>() {
                    Ok(num) => {
                        order = Some(num);
                    }
                    // Do not throw error bcs trying for u32 but Expression::integer are in u64
                    Err(_) => {
                        return Err(ParseError::DerivativeFailed);
                    }
                }

                self.advance();
                if let Some(Token::Divide) = self.current_token() {
                    self.advance();
                } else {
                    return Ok(Expression::exponentiation(
                        Expression::variable("d"),
                        Expression::integer(order.unwrap() as u64),
                    ));
                }
            } else {
                return Err(ParseError::DerivativeFailed);
            }
        } else {
            self.advance();
        }

        self.pass_whitespace();

        let var = if let Some(Token::Literal(literal)) = self.current_token().cloned() {
            self.advance();
            if let Some(var) = literal.clone().strip_prefix("d") {
                // d/dvar
                // self.pass_whitespace();
                // if let Some(Token::Literal(lit)) = self.current_token().cloned() {
                //     self.advance();
                //     var = self.parse_variable(lit)?;

                let variable = self.parse_variable(var.to_owned())?;
                // d/dvar
                if let Some(Token::Caret) = self.current_token() {
                    self.advance();
                        // d/dvar^
                        if let Some(Token::Number(num_str)) = self.current_token().cloned() {
                            self.advance();
                            // d/dvar^num
                            match (num_str.parse::<u32>(), order) {
                                (Ok(num), Some(ord)) => {
                                    if num != ord {
                                        return Err(ParseError::DerivativeFailed);
                                    }
                                    variable
                                }
                                (Ok(_), None) => {
                                    return Err(ParseError::DerivativeFailed);
                                }
                                (Err(_), _) => return Err(ParseError::DerivativeFailed),
                            }
                        } else {
                            return Err(ParseError::DerivativeFailed);
                        }
                    } else if order.is_some() {
                        return Err(ParseError::DerivativeFailed);
                    } else {
                        variable
                    }
                // } else {
                //     return Err(ParseError::DerivativeFailed);
                // }
            } else {
                return Err(ParseError::DerivativeFailed);
            }
        } else {
            return Err(ParseError::DerivativeFailed);
        }; 

        self.pass_whitespace();

        // let expression = if let Some(Token::LeftParen) = self.current_token() {
        //     self.advance();
            let expr = self.parse_binary(None, 0)?;
        //     if let Some(Token::RightParen) = self.current_token() {
        //         self.advance();
        //         while let Some(Token::WhiteSpace) = self.current_token() {
        //             self.advance();
        //         }
        //         Ok(expr)
        //     } else {
        //         Err(ParseError::UnexpectedToken(
        //             format!("{}", self.current_token().unwrap()),
        //             self.position,
        //         ))
        //     }
        // } else {
        //     return Err(ParseError::DerivativeFailed);
        // }?;

        Ok(Expression::derivative(
            expr,
            &var,
            order.unwrap_or(1),
        ))
    }

    fn parse_functions(&mut self, variable: String) -> Result<Expression, ParseError> {
        let mut args = Vec::new();
        self.advance();
        args.push(self.parse_binary(None, 0)?);
        while let Some(Token::Comma) = self.current_token() {
            self.advance();
            args.push(self.parse_binary(None, 0)?);
        }
        if let Some(Token::RightParen) = self.current_token() {
            self.advance();
            match args.len() {
                1 => match variable.as_str() {
                    "sin" => Ok(Expression::sin( args[0].clone())),
                    "cos" => Ok(Expression::cos( args[0].clone())),
                    "tan" => Ok(Expression::tan(args[0].clone())),
                    "asin" => Ok(Expression::asin(args[0].clone())),
                    "acos" => Ok(Expression::acos(args[0].clone())),
                    "atan" => Ok(Expression::atan(args[0].clone())),
                    "sinh" => Ok(Expression::sinh(args[0].clone())),
                    "cosh" => Ok(Expression::cosh(args[0].clone())),
                    "tanh" => Ok(Expression::tanh(args[0].clone())),
                    "asinh" => Ok(Expression::asinh(args[0].clone())),
                    "acosh" => Ok(Expression::acosh(args[0].clone())),
                    "atanh" => Ok(Expression::atanh(args[0].clone())),
                    "sqrt" => Ok(Expression::sqrt(args[0].clone())),
                    "exp" => Ok(Expression::exp(args[0].clone())),
                    "ln" => Ok(Expression::ln(args[0].clone())),
                    "log2" => Ok(Expression::log2(args[0].clone())),
                    "log10" => Ok(Expression::log10(args[0].clone())),
                    "abs" => Ok(Expression::abs(args[0].clone())),
                    "ceil" => Ok(Expression::ceil(args[0].clone())),
                    "floor" => Ok(Expression::floor(args[0].clone())),
                    _ => Err(ParseError::InvalidFunctionFormat(
                        variable,
                        1,
                        self.position,
                    )),
                },
                2 => match variable.as_str() {
                    "root" => Ok(Expression::root(args[0].clone(), args[1].clone())),

                    "log" => Ok(Expression::log(args[0].clone(), args[1].clone())),

                    "pow" => Ok(Expression::pow(args[0].clone(), args[1].clone())),
                    _ => Err(ParseError::InvalidFunctionFormat(
                        variable,
                        2,
                        self.position,
                    )),
                },
                x => Err(ParseError::InvalidFunctionFormat(
                    variable,
                    x,
                    self.position,
                )),
            }
        } else {
            Err(ParseError::UnexpectedEndOfInput(self.position))
        }
    }
}
