use std::str::Chars;
use std::iter::Peekable;
use lexer::{Token, ParseError, ParseResult, tokenizers::Tokenizer};

pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult {
        let mut characters = String::new();
        loop {
            match input_iter.peek() {
                None => {
                    println!("Whitespace: no more input");
                    break;
                },
                Some(&character) => {
                    let _ = input_iter.next();

                    println!("whitespace: '{}'", &character);
                    if character.is_whitespace() {
                        characters.push(character);
                    } else {
                        println!("Not whitespace");
                        break;
                    }
                }
            }
        }

        if !characters.is_empty() {
            Ok(Token::Whitespace)
        } else {
            Err(ParseError { error: String::from("ParseIntError") })
        }
    }
}