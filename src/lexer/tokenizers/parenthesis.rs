use lexer::{tokenizers::Tokenizer, ParseError, ParseResult, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct ParenthesisTokenizer;

impl Tokenizer for ParenthesisTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        if let Some(&character) = input_iter.peek() {
            let result = match character {
                '(' => Ok(Token::LeftParenthesis),
                ')' => Ok(Token::RightParenthesis),
                _ => Err(ParseError::from("Not a parenthesis")),
            }?;

            let _ = input_iter.next();

            Ok(result)
        } else {
            Err(ParseError::from("End of input"))
        }
    }
}
