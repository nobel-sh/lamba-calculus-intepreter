use crate::error::{InterpreterError, InterpreterResult};
use crate::term::Term;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    bindings: HashMap<String, Term>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            parent: None,
            bindings: HashMap::new(),
        }
    }

    pub fn extend(&self) -> Self {
        Environment {
            parent: Some(Box::new(self.clone())),
            bindings: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> InterpreterResult<Term> {
        match self.bindings.get(name) {
            Some(term) => Ok(term.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => Err(InterpreterError::UnboundVariable(name.to_string())),
            },
        }
    }

    pub fn set(&mut self, name: String, value: Term) {
        self.bindings.insert(name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_new() {
        let env = Environment::new();
        assert!(env.get("x").is_err());
    }

    #[test]
    fn test_environment_set_get() {
        let mut env = Environment::new();
        let term = Term::Variable("y".to_string());
        env.set("x".to_string(), term.clone());
        assert_eq!(env.get("x"), Ok(term));
    }

    #[test]
    fn test_environment_extend() {
        let mut parent = Environment::new();
        parent.set("x".to_string(), Term::Variable("parent_x".to_string()));

        let mut child = parent.extend();
        child.set("y".to_string(), Term::Variable("child_y".to_string()));

        assert_eq!(child.get("x"), Ok(Term::Variable("parent_x".to_string())));
        assert_eq!(child.get("y"), Ok(Term::Variable("child_y".to_string())));
        assert!(child.get("z").is_err());
    }

    #[test]
    fn test_environment_shadowing() {
        let mut parent = Environment::new();
        parent.set("x".to_string(), Term::Variable("parent_x".to_string()));

        let mut child = parent.extend();
        child.set("x".to_string(), Term::Variable("child_x".to_string()));

        assert_eq!(child.get("x"), Ok(Term::Variable("child_x".to_string())));
    }
}
