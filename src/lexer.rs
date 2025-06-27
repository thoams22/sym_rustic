#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Caret,
    Equals,
    Comma,
    Quote,
    SemiColon,
    SingleQuote,
    LeftParen,
    RightParen,
    Literal(String),
    Underscore,
    WhiteSpace,
    Error,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
            current_char: input.chars().next(),
        };
        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    fn number(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(self.input[start..self.position].to_string())
    }

    fn identifier(&mut self) -> Token {
        let start = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        Token::Literal(self.input[start..self.position].to_string())
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.current_char {
            Some(c) if c.is_ascii_digit() => Some(self.number()),
            Some(c) if c.is_alphabetic() => Some(self.identifier()),
            Some(' ') => {
                self.advance();
                Some(Token::WhiteSpace)
            }
            Some('+') => {
                self.advance();
                Some(Token::Plus)
            }
            Some('-') => {
                self.advance();
                Some(Token::Minus)
            }
            Some('*') => {
                self.advance();
                Some(Token::Multiply)
            }
            Some('/') => {
                self.advance();
                Some(Token::Divide)
            }
            Some('^') => {
                self.advance();
                Some(Token::Caret)
            }
            Some('(') => {
                self.advance();
                Some(Token::LeftParen)
            }
            Some(')') => {
                self.advance();
                Some(Token::RightParen)
            }
            Some('_') => {
                self.advance();
                Some(Token::Underscore)
            }
            Some('=') => {
                self.advance();
                Some(Token::Equals)
            }
            Some(',') => {
                self.advance();
                Some(Token::Comma)
            }
            Some('"') => {
                self.advance();
                Some(Token::Quote)
            }
            Some(';') => {
                self.advance();
                Some(Token::SemiColon)
            }
            Some('\'') => {
                self.advance();
                Some(Token::SingleQuote)
            }
            Some(_) => {
                self.advance();
                Some(Token::Error)
            }
            None => None,
        }
    }
}