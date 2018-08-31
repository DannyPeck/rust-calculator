use std::str::Chars;
use std::iter::Peekable;
use lexer::{Token, ParseError, ParseResult, tokenizers::Tokenizer};

pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        let mut characters = String::new();

        while let Some(&character) = input_iter.peek() {
            let _ = input_iter.next();

            if character.is_whitespace() {
                characters.push(character);
            } else {
                break;
            }
        }

        if !characters.is_empty() {
            Ok(Token::Whitespace)
        } else {
            Err(ParseError::from("Not whitespace"))
        }
    }
}