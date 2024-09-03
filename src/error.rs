use thiserror::Error;

/// Enum representing errors that can occur in the lambda calculus interpreter.
#[derive(Error, Debug)]
pub enum InterpreterError {
    /// Error for an unbound variable. This occurs when a variable is used but
    /// not defined in the current scope.
    #[error("Unbound variable: {0}")]
    UnboundVariable(String),

    /// Error for type mismatches. This occurs when the types of expressions do
    /// not align correctly during evaluation.
    #[error("Type error: {0}")]
    TypeError(String),

    /// Error during parsing. This occurs when the input string cannot be parsed
    /// into a valid lambda expression.
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Type alias for results returned by the interpreter.
pub type InterpreterResult<T> = Result<T, InterpreterError>;
