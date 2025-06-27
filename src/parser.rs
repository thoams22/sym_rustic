use crate::ast::Expression::Function;
use crate::ast::{Expression, numeral};
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
                format!("Expected end of input but found {:?}", token),
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
                Token::Plus => Expression::Addition(vec![left_expr, right_expr]),
                Token::Minus => Expression::Subtraction(Box::new(left_expr), Box::new(right_expr)),
                Token::Multiply => Expression::Multiplication(vec![left_expr, right_expr]),
                Token::Divide => Expression::Division(Box::new(left_expr), Box::new(right_expr)),
                Token::Caret => {
                    Expression::Exponentiation(Box::new(left_expr), Box::new(right_expr))
                }
                Token::Equals => Expression::Equality(Box::new(left_expr), Box::new(right_expr)),
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        format!("{:?}", token),
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
                    Ok(Expression::Multiplication(vec![
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
                    Ok(Expression::Multiplication(vec![
                        expr,
                        self.parse_binary(None, 3)?,
                    ]))
                } else {
                    Ok(Expression::Negation(Box::new(expr)))
                }
            }
            Some(Token::Number(value)) => {
                let val = value.clone();
                self.advance();
                match val.parse::<u64>() {
                    Ok(number) => {
                        while let Some(Token::WhiteSpace) = self.current_token() {
                            self.advance();
                        }
                        // Handle implicit multiplication
                        if matches!(
                            self.current_token(),
                            Some(&Token::LeftParen) | Some(&Token::Literal(_))
                        ) {
                            Ok(Expression::Multiplication(vec![
                                Expression::Number(numeral::Numeral::Integer(number)),
                                self.parse_binary(None, 3)?,
                            ]))
                        } else {
                            Ok(Expression::Number(numeral::Numeral::Integer(number)))
                        }
                    }
                    Err(_) => Err(ParseError::InvalidNumberFormat(self.position)),
                }
            }
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
                    Ok(Expression::Multiplication(vec![
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
                        Ok(Expression::Multiplication(vec![
                            expr,
                            self.parse_binary(None, 3)?,
                        ]))
                    } else {
                        Ok(expr)
                    }
                } else {
                    Err(ParseError::UnexpectedToken(
                        format!("{:?}", self.current_token().unwrap()),
                        self.position,
                    ))
                }
            }
            Some(token) => Err(ParseError::UnexpectedToken(
                format!("{:?}", token),
                self.position,
            )),
            None => Err(ParseError::UnexpectedEndOfInput(self.position)),
        };

        while let Some(Token::WhiteSpace) = self.current_token() {
            self.advance();
        }

        parsed
    }

    fn parse_literal(&mut self, variable: String) -> Result<Expression, ParseError> {
        self.advance();
        // Variables
        match self.current_token() {
            Some(Token::Underscore) => {
                let var = self.parse_variable(variable)?;
                Ok(Expression::Variable(var))
            }
            Some(Token::LeftParen) => self.parse_functions(variable),
            Some(Token::Caret) | Some(Token::Divide) if variable == "d" => {
                let position = self.position;
                match self.parse_derivative() {
                    Ok(derivative) => Ok(derivative),
                    Err(_) => {
                        self.position = position;
                        Ok(Expression::Variable("d".to_string()))
                    },
                }
            }
            _ => match variable.as_str() {
                "tau" => Ok(Expression::Constant(crate::ast::constant::Constant::Tau)),
                "pi" => Ok(Expression::Constant(crate::ast::constant::Constant::Pi)),
                "e" => Ok(Expression::Constant(crate::ast::constant::Constant::E)),
                "i" => Ok(Expression::Complex(
                    Box::new(Expression::Number(numeral::Numeral::Integer(0))),
                    Box::new(Expression::Number(numeral::Numeral::Integer(1))),
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
                    return Ok(Expression::Exponentiation(
                        Box::new(Expression::Variable("d".to_string())),
                        Box::new(Expression::integer(order.unwrap() as u64)),
                    ));
                }
            } else {
                return Err(ParseError::DerivativeFailed);
            }
        } else {
            self.advance();
        }

        self.pass_whitespace();

        let var;
        if let Some(Token::Literal(literal)) = self.current_token().cloned() {
            self.advance();
            if literal == "d" {
                // d/d
                self.pass_whitespace();
                if let Some(Token::Literal(lit)) = self.current_token().cloned() {
                    self.advance();
                    var = self.parse_variable(lit)?;
                    // d/d var
                    if let Some(Token::Caret) = self.current_token() {
                        self.advance();
                        // d/d var^
                        if let Some(Token::Number(num_str)) = self.current_token().cloned() {
                            self.advance();
                            // d/d var^num
                            match (num_str.parse::<u32>(), order) {
                                (Ok(num), Some(ord)) => {
                                    if num != ord {
                                        return Err(ParseError::DerivativeFailed);
                                    }
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
                    }
                } else {
                    return Err(ParseError::DerivativeFailed);
                }
            } else {
                return Err(ParseError::DerivativeFailed);
            }
        } else {
            return Err(ParseError::DerivativeFailed)
        }

        self.pass_whitespace();


        let expression = if let Some(Token::LeftParen) = self.current_token() {
            self.advance();
            let expr = self.parse_binary(None, 0)?;
            if let Some(Token::RightParen) = self.current_token() {
                self.advance();
                while let Some(Token::WhiteSpace) = self.current_token() {
                    self.advance();
                }
                Ok(expr)
            } else {
                Err(ParseError::UnexpectedToken(
                    format!("{:?}", self.current_token().unwrap()),
                    self.position,
                ))
            }
        } else {
            return Err(ParseError::DerivativeFailed);
        }?;

        Ok(Expression::Derivative(
            Box::new(expression),
            var.to_string(),
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
                    "sin" => Ok(Function(crate::ast::function::Function::Sin, args)),
                    "cos" => Ok(Function(crate::ast::function::Function::Cos, args)),
                    "tan" => Ok(Function(crate::ast::function::Function::Tan, args)),
                    "asin" => Ok(Function(crate::ast::function::Function::Asin, args)),
                    "acos" => Ok(Function(crate::ast::function::Function::Acos, args)),
                    "atan" => Ok(Function(crate::ast::function::Function::Atan, args)),
                    "sinh" => Ok(Function(crate::ast::function::Function::Sinh, args)),
                    "cosh" => Ok(Function(crate::ast::function::Function::Cosh, args)),
                    "tanh" => Ok(Function(crate::ast::function::Function::Tanh, args)),
                    "asinh" => Ok(Function(crate::ast::function::Function::Asinh, args)),
                    "acosh" => Ok(Function(crate::ast::function::Function::Acosh, args)),
                    "atanh" => Ok(Function(crate::ast::function::Function::Atanh, args)),
                    "sqrt" => Ok(Function(crate::ast::function::Function::Sqrt, args)),
                    "exp" => Ok(Function(crate::ast::function::Function::Exp, args)),
                    "ln" => Ok(Function(crate::ast::function::Function::Ln, args)),
                    "log2" => Ok(Function(crate::ast::function::Function::Log2, args)),
                    "log10" => Ok(Function(crate::ast::function::Function::Log10, args)),
                    "abs" => Ok(Function(crate::ast::function::Function::Abs, args)),
                    "ceil" => Ok(Function(crate::ast::function::Function::Ceil, args)),
                    "floor" => Ok(Function(crate::ast::function::Function::Floor, args)),
                    _ => Err(ParseError::InvalidFunctionFormat(
                        variable,
                        1,
                        self.position,
                    )),
                },
                2 => match variable.as_str() {
                    "root" => Ok(Function(crate::ast::function::Function::Root, args)),

                    "log" => Ok(Function(crate::ast::function::Function::Log, args)),

                    "pow" => Ok(Function(crate::ast::function::Function::Pow, args)),
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
