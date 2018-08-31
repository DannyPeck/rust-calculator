use std::str::Chars;
use std::iter::Peekable;
use lexer::{Token, ParseError, ParseResult, tokenizers::Tokenizer};

pub struct OperatorTokenizer;

impl Tokenizer for OperatorTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        if let Some(&character) = input_iter.peek() {
           let result = match character {
                '+' => Ok(Token::Addition),
                '-' => Ok(Token::Subtraction),
                '/' => Ok(Token::Division),
                '*' => Ok(Token::Multiplication),
                _ => Err(ParseError::from("Not a supported operator"))
            }?;

            let _ = input_iter.next();

            Ok(result)
        } else {
            Err(ParseError::from("End of input"))
        }
    }
}