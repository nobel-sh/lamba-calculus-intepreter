use crate::error::{InterpreterError, InterpreterResult};
use crate::term::Term;
use crate::tokenizer::{tokenize, TokenKind};

/// Parses a string into a `Term` representing a lambda calculus expression.
///
/// Grammar:
///   parse     ::= term EOF
///   EOF       ::= (end of input)
pub fn parse(input: &str) -> InterpreterResult<Term> {
    let tokens = tokenize(input)?;
    let (term, remaining) = parse_term(&tokens)?;
    if !remaining.is_empty() {
        return Err(InterpreterError::ParseError(
            "Unexpected tokens at end of input".to_string(),
        ));
    }
    Ok(term)
}

/// Parses the next term from the token stream.
///
/// Grammar:
///   term      ::= abstraction
///              | application
///              | variable
///   abstraction ::= lambda identifier . term
///   lambda ::= λ | \
///   application ::= ( term term )
///   variable  ::= identifier
fn parse_term(tokens: &[TokenKind]) -> Result<(Term, &[TokenKind]), InterpreterError> {
    if let Some(token) = tokens.first() {
        match token {
            TokenKind::Lambda => parse_abstraction(&tokens[1..]),
            TokenKind::OpenParen => parse_application(&tokens[1..]),
            TokenKind::Identifier(name) => Ok((Term::Variable(name.clone()), &tokens[1..])),
            _ => Err(InterpreterError::ParseError(format!(
                "Unexpected token: {:?}",
                token
            ))),
        }
    } else {
        Err(InterpreterError::ParseError(
            "Unexpected end of input".to_string(),
        ))
    }
}

/// Parses an abstraction term.
///
/// Grammar:
///   abstraction ::= λ identifier . term
fn parse_abstraction(tokens: &[TokenKind]) -> Result<(Term, &[TokenKind]), InterpreterError> {
    if let Some(TokenKind::Identifier(param)) = tokens.first() {
        if let Some(TokenKind::Dot) = tokens.get(1) {
            let (body, remaining) = parse_term(&tokens[2..])?;
            Ok((Term::Abstraction(param.clone(), Box::new(body)), remaining))
        } else {
            Err(InterpreterError::ParseError(
                "Expected '.' after parameter in abstraction".to_string(),
            ))
        }
    } else {
        Err(InterpreterError::ParseError(
            "Expected parameter in abstraction".to_string(),
        ))
    }
}

/// Parses an application term.
///
/// Grammar:
///   application ::= ( term term )
fn parse_application(tokens: &[TokenKind]) -> Result<(Term, &[TokenKind]), InterpreterError> {
    let (func, remaining) = parse_term(tokens)?;
    let (arg, remaining) = parse_term(remaining)?;
    if let Some(TokenKind::CloseParen) = remaining.first() {
        Ok((
            Term::Application(Box::new(func), Box::new(arg)),
            &remaining[1..],
        ))
    } else {
        Err(InterpreterError::ParseError(
            "Expected closing parenthesis".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variable() {
        assert_eq!(parse("x"), Ok(Term::Variable("x".to_string())));
    }

    #[test]
    fn test_parse_abstraction() {
        assert_eq!(
            parse("λx.x"),
            Ok(Term::Abstraction(
                "x".to_string(),
                Box::new(Term::Variable("x".to_string()))
            ))
        );
    }

    #[test]
    fn test_parse_application() {
        assert_eq!(
            parse("(f x)"),
            Ok(Term::Application(
                Box::new(Term::Variable("f".to_string())),
                Box::new(Term::Variable("x".to_string()))
            ))
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        assert_eq!(
            parse("(λx.λy.(x y) a)"),
            Ok(Term::Application(
                Box::new(Term::Abstraction(
                    "x".to_string(),
                    Box::new(Term::Abstraction(
                        "y".to_string(),
                        Box::new(Term::Application(
                            Box::new(Term::Variable("x".to_string())),
                            Box::new(Term::Variable("y".to_string()))
                        ))
                    ))
                )),
                Box::new(Term::Variable("a".to_string()))
            ))
        );
    }

    #[test]
    fn test_parse_error_missing_dot() {
        assert!(parse("λx x").is_err());
    }

    #[test]
    fn test_parse_error_unexpected_end() {
        assert!(parse("λx").is_err());
    }

    #[test]
    fn test_parse_error_unexpected_parenthesis() {
        assert!(parse("(x").is_err());
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        assert!(parse("λx.x)").is_err());
    }
}
