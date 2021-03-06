use super::Token;
use lexer::tokenizers::Tokenizer;
use std::iter::Peekable;
use std::str::Chars;

pub struct OperatorTokenizer;

impl Tokenizer for OperatorTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> Option<Token> {
        let token = match input_iter.peek()? {
            '+' => Some(Token::Addition),
            '-' => Some(Token::Subtraction),
            '/' => Some(Token::Division),
            '*' => Some(Token::Multiplication),
            _ => None,
        }?;

        input_iter.next();

        Some(token)
    }
}
