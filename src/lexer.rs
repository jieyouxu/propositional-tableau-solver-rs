use logos::Logos;
use std::ops::Range;

/// The token type for Logos to generate a lexer from.
#[derive(Logos, Debug, Clone, PartialEq)]
enum LogosToken {
    /// Default EOF token required by Logos.
    #[end]
    End,
    /// Default error token required by Logos.
    #[error]
    Error,

    #[token = "("]
    LeftParen,
    #[token = ")"]
    RightParen,

    /// Negation can be either a dash `-` or tilde `~`.
    #[regex = "-|~"]
    Negation,

    /// Conjunction can be either an ampersand `&` or a caret `^`.
    #[regex = "&|\\^"]
    And,

    /// Disjunction.
    #[regex = "\\|"]
    Or,

    /// Implication can be either a thin right arrow `->` or a thick right
    /// arrow `=>`.
    #[regex = "(->)|(=>)"]
    Implication,

    /// Biimplication can be either a thin double-headed arrow `<->` or a thick
    /// double-headed arrow `<=>`.
    #[regex = "(<->)|(<=>)"]
    Biimplication,

    /// A propositional variable must be at least one character long, and must
    /// begin with a alpha character `[a-zA-Z]`, and optionally followed by any
    /// alphanumeric or underscore characters `[a-zA-Z0-9_]*`.
    #[regex = "[a-zA-Z][a-zA-Z0-9_]*"]
    Variable,
}

/// A token is a part of a well-formed propositional formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Token {
    Variable(String),
    LeftParen,
    RightParen,
    Negation,
    And,
    Or,
    Implication,
    Biimplication,
}

pub(crate) type Tokens = Vec<Token>;

/// Error information for when a lex fails.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LexError<'a> {
    /// The cause of the failed lex.
    cause: &'static str,
    /// The current span of input string that is being inspected which failed
    /// lex.
    current_slice: &'a str,
    /// The starting position of the part of the input string which failed to
    /// lex (inclusive).
    start: usize,
    /// The end position of the part of the input srting which failed to lex
    /// (exclusive).
    end: usize,
}

/// Lex the input string to tokens.
pub(crate) fn lex<'a>(input: &'a str) -> Result<Tokens, LexError<'a>> {
    let mut tokens = Vec::new();

    let mut lexer = LogosToken::lexer(input);

    loop {
        match lexer.token {
            LogosToken::End => {
                break;
            }
            LogosToken::Error => {
                let Range { start, end } = lexer.range();
                return Err(LexError {
                    cause: "failed to lex input",
                    current_slice: lexer.slice(),
                    start,
                    end,
                });
            }
            LogosToken::Variable => {
                tokens.push(Token::Variable(lexer.slice().to_owned()));
            }
            LogosToken::LeftParen => {
                tokens.push(Token::LeftParen);
            }
            LogosToken::RightParen => {
                tokens.push(Token::RightParen);
            }
            LogosToken::And => {
                tokens.push(Token::And);
            }
            LogosToken::Or => {
                tokens.push(Token::Or);
            }
            LogosToken::Implication => {
                tokens.push(Token::Implication);
            }
            LogosToken::Biimplication => {
                tokens.push(Token::Biimplication);
            }
            LogosToken::Negation => {
                tokens.push(Token::Negation);
            }
        }

        lexer.advance();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn empty_input() {
        let result = lex("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn negation() {
        assert_eq!(Token::Negation, *lex("-").unwrap().get(0).unwrap());
        assert_eq!(Token::Negation, *lex("~").unwrap().get(0).unwrap());
    }

    #[test]
    fn simple_variable() {
        assert_eq!(
            Token::Variable("a".to_string()),
            *lex("a").unwrap().get(0).unwrap()
        );
        assert_eq!(
            Token::Variable("A".to_string()),
            *lex("A").unwrap().get(0).unwrap()
        );
        assert!(lex("0").is_err());
        assert!(lex("_").is_err());
    }

    #[test]
    fn complex_variable() {
        assert_eq!(
            Token::Variable("aa".to_string()),
            *lex("aa").unwrap().get(0).unwrap()
        );

        assert_eq!(
            Token::Variable("a0".to_string()),
            *lex("a0").unwrap().get(0).unwrap()
        );

        assert_eq!(
            Token::Variable("a_0".to_string()),
            *lex("a_0").unwrap().get(0).unwrap()
        );

        assert_eq!(
            Token::Variable("a____".to_string()),
            *lex("a____").unwrap().get(0).unwrap()
        );
    }

    #[test]
    fn and() {
        assert_eq!(Token::And, *lex("^").unwrap().get(0).unwrap());
        assert_eq!(Token::And, *lex("&").unwrap().get(0).unwrap());
    }

    #[test]
    fn or() {
        assert_eq!(Token::Or, *lex("|").unwrap().get(0).unwrap());
    }

    #[test]
    fn implication() {
        assert_eq!(Token::Implication, *lex("->").unwrap().get(0).unwrap());
        assert_eq!(Token::Implication, *lex("=>").unwrap().get(0).unwrap());

        assert!(lex("=").is_err());
        assert!(lex(">").is_err());
    }

    #[test]
    fn biimplication() {
        assert_eq!(Token::Biimplication, *lex("<->").unwrap().get(0).unwrap());
        assert_eq!(Token::Biimplication, *lex("<=>").unwrap().get(0).unwrap());
    }

    #[test]
    fn parentheses() {
        assert_eq!(Token::LeftParen, *lex("(").unwrap().get(0).unwrap());
        assert_eq!(Token::RightParen, *lex(")").unwrap().get(0).unwrap());
    }

    #[test]
    fn complex_formula() {
        let tokens = lex("-(a&(b|c))->(d<=>f)").unwrap();
        assert_eq!(
            [
                Token::Negation,
                Token::LeftParen,
                Token::Variable("a".to_string()),
                Token::And,
                Token::LeftParen,
                Token::Variable("b".to_string()),
                Token::Or,
                Token::Variable("c".to_string()),
                Token::RightParen,
                Token::RightParen,
                Token::Implication,
                Token::LeftParen,
                Token::Variable("d".to_string()),
                Token::Biimplication,
                Token::Variable("f".to_string()),
                Token::RightParen,
            ]
            .to_vec(),
            tokens
        );
    }
}
