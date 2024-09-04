use std::fmt;

/// Enum representing a lambda calculus term, which can be a variable, an
/// abstraction, or an application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// A variable term, represented by its name.
    Variable(String),

    /// An abstraction term, which binds a variable to a body term. This
    /// basically represents a function with one parameter.
    Abstraction(String, Box<Term>),

    /// An application term, representing the application of one term
    /// (function) to another term (argument).
    Application(Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Variable(name) => write!(f, "{}", name),
            Term::Abstraction(param, body) => write!(f, "λ{}.{}", param, body),
            Term::Application(func, arg) => write!(f, "({} {})", func, arg),
        }
    }
}
