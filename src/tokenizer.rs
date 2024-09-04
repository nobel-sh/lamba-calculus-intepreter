use crate::error::{InterpreterError, InterpreterResult};

/// Enum representing different kind of tokens.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Lambda,
    Dot,
    OpenParen,
    CloseParen,
    Identifier(String),
}

/// Tokenizes a string into a vector of tokens.
pub fn tokenize(input: &str) -> InterpreterResult<Vec<TokenKind>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            'λ' | '\\' => {
                tokens.push(TokenKind::Lambda);
                chars.next();
            }
            '.' => {
                tokens.push(TokenKind::Dot);
                chars.next();
            }
            '(' => {
                tokens.push(TokenKind::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(TokenKind::CloseParen);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next(); // Skip whitespace
            }
            _ => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !identifier.is_empty() {
                    tokens.push(TokenKind::Identifier(identifier));
                } else {
                    return Err(InterpreterError::TokenizerError(format!(
                        "Unexpected character '{}'",
                        c
                    )));
                }
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_variable() {
        assert_eq!(
            tokenize("x"),
            Ok(vec![TokenKind::Identifier("x".to_string())])
        );
    }

    #[test]
    fn test_tokenize_lambda() {
        assert_eq!(
            tokenize("λx.x"),
            Ok(vec![
                TokenKind::Lambda,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Dot,
                TokenKind::Identifier("x".to_string())
            ])
        );
        assert_eq!(
            tokenize("\\x.x"),
            Ok(vec![
                TokenKind::Lambda,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Dot,
                TokenKind::Identifier("x".to_string())
            ])
        );
    }

    #[test]
    fn test_tokenize_application() {
        assert_eq!(
            tokenize("(f x)"),
            Ok(vec![
                TokenKind::OpenParen,
                TokenKind::Identifier("f".to_string()),
                TokenKind::Identifier("x".to_string()),
                TokenKind::CloseParen
            ])
        );
    }

    #[test]
    fn test_tokenize_multiple_tokens() {
        assert_eq!(
            tokenize("λx.(y z)"),
            Ok(vec![
                TokenKind::Lambda,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Dot,
                TokenKind::OpenParen,
                TokenKind::Identifier("y".to_string()),
                TokenKind::Identifier("z".to_string()),
                TokenKind::CloseParen
            ])
        );
    }

    #[test]
    fn test_tokenize_whitespace() {
        assert_eq!(
            tokenize("λ x .  y z "),
            Ok(vec![
                TokenKind::Lambda,
                TokenKind::Identifier("x".to_string()),
                TokenKind::Dot,
                TokenKind::Identifier("y".to_string()),
                TokenKind::Identifier("z".to_string())
            ])
        );
    }

    #[test]
    fn test_tokenize_invalid_character() {
        assert_eq!(
            tokenize("λx@x"),
            Err(InterpreterError::TokenizerError(
                "Unexpected character '@'".to_string()
            ))
        );
    }
}
