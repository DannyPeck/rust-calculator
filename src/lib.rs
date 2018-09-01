mod lexer;
mod parser;

use parser::Parser;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Number(i32),
    LeftParenthesis,
    RightParenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Whitespace,
}

#[derive(Debug)]
pub struct EvalError {
    error: String,
}

impl EvalError {
    pub fn from(error: &str) -> EvalError {
        EvalError {
            error: String::from(error),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}

type EvalResult = Result<i32, EvalError>;

pub fn evaluate(input: &String) -> EvalResult {
    let mut parser = Parser::new();
    let mut input_iter = input.trim().chars().peekable();

    while input_iter.peek().is_some() {
        if let Some(token) = lexer::process(&mut input_iter) {
            if !parser.process(token) {
                return Err(EvalError::from("Parser failed"));
            }
        } else if input_iter.peek().is_some() {
            return Err(EvalError::from("Didn't finish tokenizing"));
        }
    }

    Ok(0)
}
