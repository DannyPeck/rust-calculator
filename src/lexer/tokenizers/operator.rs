use std::str::Chars;
use std::iter::Peekable;
use lexer::{Token, ParseError, ParseResult, tokenizers::Tokenizer};

pub struct OperatorTokenizer;

impl Tokenizer for OperatorTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        match input_iter.peek() {
            None => Err(ParseError { error: String::from("end") }),
            Some(&character) => {
                let _ = input_iter.next();

                match character {
                    '+' => Ok(Token::Addition),
                    '-' => Ok(Token::Subtraction),
                    '/' => Ok(Token::Division),
                    '*' => Ok(Token::Multiplication),
                    _ => Err(ParseError { error: String::from("Not a supported operator")})
                }
            }
        }
    }
}