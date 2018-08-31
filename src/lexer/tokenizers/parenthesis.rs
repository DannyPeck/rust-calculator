use lexer::{tokenizers::Tokenizer, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct ParenthesisTokenizer;

impl Tokenizer for ParenthesisTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> Option<Token> {
        match input_iter.peek() {
            Some(&character) => {
                let token = match character {
                    '(' => Some(Token::LeftParenthesis),
                    ')' => Some(Token::RightParenthesis),
                    _ => None,
                }?;

                input_iter.next();

                Some(token)
            },
            None => None
        }
    }
}
