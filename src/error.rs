use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Unbound variable: {0}")]
    UnboundVariable(String),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;
