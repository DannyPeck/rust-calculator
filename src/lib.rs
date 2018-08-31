mod lexer;

use lexer::Token;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    error: String,
}

impl ParseError {
    pub fn from(error: &str) -> ParseError {
        ParseError {
            error: String::from(error),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}

type ParseResult = Result<Token, ParseError>;

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
    let mut input_iter = input.trim().chars().peekable();

    while let Some(_) = input_iter.peek() {
        if let Some(token) = lexer::process(&mut input_iter) {
            println!("token: {:?}", &token);
        }
    }

    Ok(0)
}
