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
mod tests_bad_token {
    use crate::lex;
    use sym_rustic::lexer::Token;

    #[test]
    fn test_bad_token_1() {
        let tokens: Vec<Token> = lex("é");

        assert_eq!(tokens, vec![Token::Unsupported("é".to_string())])
    }

    #[test]
    fn test_bad_token_2() {
        let tokens: Vec<Token> = lex("老");

        assert_eq!(tokens, vec![Token::Unsupported("老".to_string())])
    }
}

#[cfg(test)]
mod tests_good_token {
    use crate::lex;
    use sym_rustic::lexer::Token;

    #[test]
    fn test_good_token_1() {
        let tokens: Vec<Token> = lex("aae");

        assert_eq!(tokens, vec![Token::Literal("aae".to_string())])
    }

    #[test]
    fn test_good_token_2() {
        let tokens: Vec<Token> = lex("18");

        assert_eq!(tokens, vec![Token::Number("18".to_string())])
    }

    #[test]
    fn test_good_token_3() {
        let tokens: Vec<Token> = lex("18.9");

        assert_eq!(
            tokens,
            vec![
                Token::Number("18".to_string()),
                Token::Dot,
                Token::Number("9".to_string())
            ]
        )
    }
}