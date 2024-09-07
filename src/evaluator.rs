use crate::environment::Environment;
use crate::error::{InterpreterError, InterpreterResult};
use crate::term::Term;

pub fn evaluate(term: &Term, env: &mut Environment) -> InterpreterResult<Term> {
    match term {
        Term::Variable(name) => Ok(env.get(name).unwrap_or_else(|| term.clone())),
        Term::Abstraction(param, body) => {
            let new_body = substitute(body, env);
            Ok(Term::Abstraction(param.clone(), Box::new(new_body)))
        }
        Term::Application(func, arg) => {
            let eval_func = evaluate(func, env)?;
            match eval_func {
                Term::Abstraction(param, body) => {
                    let eval_arg = evaluate(arg, env)?;
                    let mut new_env = env.extend();
                    new_env.set(param, eval_arg);
                    evaluate(&body, &mut new_env)
                }
                _ => Err(InterpreterError::TypeError(
                    "Expected function in application".to_string(),
                )),
            }
        }
    }
}

fn substitute(term: &Term, env: &Environment) -> Term {
    match term {
        Term::Variable(name) => env.get(name).unwrap_or_else(|| term.clone()),
        Term::Abstraction(param, body) => {
            let new_body = substitute(body, env);
            Term::Abstraction(param.clone(), Box::new(new_body))
        }
        Term::Application(func, arg) => {
            let new_func = substitute(func, env);
            let new_arg = substitute(arg, env);
            Term::Application(Box::new(new_func), Box::new(new_arg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_variable() {
        let mut env = Environment::new();
        env.set("x".to_string(), Term::Variable("y".to_string()));
        assert_eq!(
            evaluate(&Term::Variable("x".to_string()), &mut env),
            Ok(Term::Variable("y".to_string()))
        );
    }

    #[test]
    fn test_evaluate_abstraction() {
        let mut env = Environment::new();
        let abs = Term::Abstraction("x".to_string(), Box::new(Term::Variable("x".to_string())));
        assert_eq!(evaluate(&abs, &mut env), Ok(abs));
    }

    #[test]
    fn test_evaluate_application() {
        let mut env = Environment::new();
        let app = Term::Application(
            Box::new(Term::Abstraction(
                "x".to_string(),
                Box::new(Term::Variable("x".to_string())),
            )),
            Box::new(Term::Variable("y".to_string())),
        );
        assert_eq!(
            evaluate(&app, &mut env),
            Ok(Term::Variable("y".to_string()))
        );
    }

    #[test]
    fn test_evaluate_complex_expression() {
        let mut env = Environment::new();
        let expr = Term::Application(
            Box::new(Term::Abstraction(
                "x".to_string(),
                Box::new(Term::Abstraction(
                    "y".to_string(),
                    Box::new(Term::Application(
                        Box::new(Term::Variable("x".to_string())),
                        Box::new(Term::Variable("y".to_string())),
                    )),
                )),
            )),
            Box::new(Term::Variable("a".to_string())),
        );
        let expected = Term::Abstraction(
            "y".to_string(),
            Box::new(Term::Application(
                Box::new(Term::Variable("a".to_string())),
                Box::new(Term::Variable("y".to_string())),
            )),
        );
        assert_eq!(evaluate(&expr, &mut env), Ok(expected));
    }

    #[test]
    fn test_evaluate_error() {
        let mut env = Environment::new();
        let expr = Term::Application(
            Box::new(Term::Variable("x".to_string())),
            Box::new(Term::Variable("y".to_string())),
        );
        assert!(evaluate(&expr, &mut env).is_err());
    }
}
