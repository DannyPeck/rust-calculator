use super::Token;
use lexer::tokenizers::Tokenizer;
use std::iter::Peekable;
use std::str::Chars;

pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> Option<Token> {
        let mut characters = String::new();

        while let Some(&character) = input_iter.peek() {
            if character.is_whitespace() {
                characters.push(character);
                input_iter.next();
            } else {
                break;
            }
        }

        if !characters.is_empty() {
            Some(Token::Whitespace)
        } else {
            None
        }
    }
}
