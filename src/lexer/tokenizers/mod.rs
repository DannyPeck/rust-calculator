use std::str::Chars;
use std::iter::Peekable;
use lexer::ParseResult;

pub trait Tokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> ParseResult;
}

pub mod number;
pub mod operator;
pub mod whitespace;