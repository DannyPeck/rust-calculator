use super::Token;
use std::iter::Peekable;
use std::str::Chars;

pub trait Tokenizer {
    fn process(&self, input_iter: &mut Peekable<Chars>) -> Option<Token>;
}

pub mod number;
pub mod operator;
pub mod parenthesis;
pub mod whitespace;
