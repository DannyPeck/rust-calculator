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
        loop {
            match input_iter.peek() {
                None => break,
                Some(&character) => {
                    let _ = input_iter.next();

                    if character.is_digit(10) {
                        println!("char: {}", &character);
                        characters.push(character);
                    } else {
                        break;
                    }
                }
            }
        }

        match characters.parse::<i32>() {
            Err(_) => Err(ParseError { error: String::from("ParseIntError") }),
            Ok(number) => Ok(Token::Number(number))
        }
    }
}