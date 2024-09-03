use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
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
