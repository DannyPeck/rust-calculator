use std::str::Chars;
use std::iter::Peekable;
use lexer::Token;
use lexer::ParseError;
use lexer::ParseResult;
use lexer::tokenizers::Tokenizer;

pub struct NumberTokenizer;

impl Tokenizer for NumberTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        let mut characters = String::new();

        while let Some(&character) = input_iter.peek() {
            if character.is_digit(10) {
                let _ = input_iter.next();
                characters.push(character);
            } else {
                break;
            }
        }

        let number = characters.parse::<i32>().map_err(|_| { ParseError::from("ParseIntError") })?;
        Ok(Token::Number(number))
    }
}