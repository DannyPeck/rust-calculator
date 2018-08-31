use std::fmt;

mod lexer;

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

pub fn evaluate(input: &String) -> Result<i32, EvalError> {
    let mut input_iter = input.trim().chars().peekable();

    while let Some(_) = input_iter.peek() {
        let token = lexer::process(&mut input_iter).map_err(|_| EvalError::from("Invalid token"))?;
        println!("token: {:?}", &token);
    }

    Ok(0)
}
