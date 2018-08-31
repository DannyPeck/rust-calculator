mod tokenizers;

use lexer::tokenizers::{
    number::NumberTokenizer, operator::OperatorTokenizer, parenthesis::ParenthesisTokenizer,
    whitespace::WhitespaceTokenizer, Tokenizer,
};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Token {
    Number(i32),
    LeftParenthesis,
    RightParenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Whitespace,
}

pub fn process(input_iter: &mut Peekable<Chars>) -> Option<Token> {
    let tokenizers: Vec<Box<Tokenizer>> = vec![
        Box::new(NumberTokenizer),
        Box::new(ParenthesisTokenizer),
        Box::new(OperatorTokenizer),
        Box::new(WhitespaceTokenizer),
    ];

    for tokenizer in &tokenizers {
        if let Some(token) = tokenizer.process(input_iter) {
            return Some(token);
        }
    }

    None
}
