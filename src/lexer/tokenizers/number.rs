use lexer::tokenizers::Tokenizer;
use lexer::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct NumberTokenizer;

impl Tokenizer for NumberTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> Option<Token> {
        let mut characters = String::new();

        while let Some(&character) = input_iter.peek() {
            if character.is_digit(10) {
                characters.push(character);
                input_iter.next();
            } else {
                break;
            }
        }

        match characters.parse::<i32>().ok() {
            Some(number) => Some(Token::Number(number)),
            None => None,
        }
    }
}
