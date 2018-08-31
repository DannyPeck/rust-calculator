use std::str::Chars;
use std::iter::Peekable;
use lexer::{Token, ParseError, ParseResult, tokenizers::Tokenizer};

pub struct OperatorTokenizer;

impl Tokenizer for OperatorTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        if let Some(&character) = input_iter.peek() {
            let _ = input_iter.next();

            match character {
                '+' => Ok(Token::Addition),
                '-' => Ok(Token::Subtraction),
                '/' => Ok(Token::Division),
                '*' => Ok(Token::Multiplication),
                _ => Err(ParseError::from("Not a supported operator"))
            }
        } else {
            Err(ParseError::from("End of input"))
        }
    }
}