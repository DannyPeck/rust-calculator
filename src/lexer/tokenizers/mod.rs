use lexer::ParseResult;
use std::iter::Peekable;
use std::str::Chars;

pub trait Tokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult;
}

pub mod number;
pub mod operator;
pub mod parenthesis;
pub mod whitespace;
