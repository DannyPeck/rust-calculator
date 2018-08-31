use lexer::{tokenizers::Tokenizer, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct OperatorTokenizer;

impl Tokenizer for OperatorTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> Option<Token> {
        match input_iter.peek() {
            Some(&character) => {
                let token = match character {
                    '+' => Some(Token::Addition),
                    '-' => Some(Token::Subtraction),
                    '/' => Some(Token::Division),
                    '*' => Some(Token::Multiplication),
                    _ => None,
                }?;

                input_iter.next();

                Some(token)
            },
            None => None
        }
    }
}
