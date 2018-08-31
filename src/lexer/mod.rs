mod tokenizers;

use std::fmt;
use std::str::Chars;
use std::iter::Peekable;
use lexer::tokenizers::{Tokenizer, number::NumberTokenizer, operator::OperatorTokenizer, whitespace::WhitespaceTokenizer};

#[derive(Debug)]
pub struct ParseError {
    error: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}

#[derive(Debug)]
pub enum Token {
    Number(i32),
    LeftParenthesis,
    RightParenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Whitespace
}

type ParseResult = Result<Token, ParseError>;

pub fn process(input_iter: &mut Peekable<Chars>) -> ParseResult {
    let tokenizers: Vec<Box<Tokenizer>> = vec![Box::new(NumberTokenizer), Box::new(OperatorTokenizer), Box::new(WhitespaceTokenizer)];
    for tokenizer in &tokenizers {
        if let Ok(token) = tokenizer.process(input_iter) {
            println!("token: {:?}", &token);
            return Ok(token)
        }
    }

    Err(ParseError { error: String::from("Invalid token") })
}
