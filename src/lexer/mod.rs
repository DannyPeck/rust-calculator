mod tokenizers;

use lexer::tokenizers::{
    number::NumberTokenizer, operator::OperatorTokenizer, parenthesis::ParenthesisTokenizer,
    whitespace::WhitespaceTokenizer, Tokenizer,
};
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

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

#[derive(Debug)]
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

type ParseResult = Result<Token, ParseError>;

pub fn process(input_iter: &mut Peekable<Chars>) -> ParseResult {
    let tokenizers: Vec<Box<Tokenizer>> = vec![
        Box::new(NumberTokenizer),
        Box::new(ParenthesisTokenizer),
        Box::new(OperatorTokenizer),
        Box::new(WhitespaceTokenizer),
    ];

    for tokenizer in &tokenizers {
        if let Ok(token) = tokenizer.process(input_iter) {
            return Ok(token);
        }
    }

    Err(ParseError::from("Invalid token"))
}
