use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(String),
    Literal(String),

    Plus,
    Minus,
    Multiply,
    Divide,
    Caret,
    Equals,

    Percent,
    Dot,
    Comma,
    Quote,
    SingleQuote,
    SemiColon,

    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,

    Underscore,
    WhiteSpace,

    Unsupported(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(a) => write!(f, "{a}"),
            Token::Literal(a) => write!(f, "{a}"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Caret => write!(f, "^"),
            Token::Equals => write!(f, "="),
            Token::Percent => write!(f, "%"),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Quote => write!(f, "\""),
            Token::SingleQuote => write!(f, "'"),
            Token::SemiColon => write!(f, ";"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftCurly => write!(f, "{{"),
            Token::RightCurly => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Underscore => write!(f, "_"),
            Token::WhiteSpace => write!(f, "WhiteSpace"),
            Token::Unsupported(a) => write!(f, "Unsupported: {a}"),
        }
    }
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
            if c.is_ascii_alphanumeric() {
                self.advance();
            }
            else {
                break;
            }
        }
        Token::Literal(self.input[start..self.position].to_string())
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.current_char {
            Some(c) if c.is_ascii_digit() => Some(self.number()),
            Some(c) if c.is_ascii_alphabetic() => Some(self.identifier()),
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
            Some('%') => {
                self.advance();
                Some(Token::Percent)
            }
            Some('.') => {
                self.advance();
                Some(Token::Dot)
            }
            Some('{') => {
                self.advance();
                Some(Token::LeftCurly)
            }
            Some('}') => {
                self.advance();
                Some(Token::RightCurly)
            }
            Some('[') => {
                self.advance();
                Some(Token::LeftBracket)
            }
            Some(']') => {
                self.advance();
                Some(Token::RightBracket)
            }
            Some(a) => {
                self.advance();
                Some(Token::Unsupported(a.to_string()))
            }
            None => None,
        }
    }
}
