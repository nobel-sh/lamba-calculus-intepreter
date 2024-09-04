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
///   abstraction ::= λ identifier . term
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
