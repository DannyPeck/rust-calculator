use super::Token;

use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    error: String,
}

impl ParseError {
    pub fn from(error: &str) -> ParseError {
        ParseError {
            error: String::from(error),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.error)
    }
}

#[derive(Debug)]
enum Symbol {
    Start,
    Expression,
    ExpressionTail,
    Term,
    TermTail,
    Factor,
    Terminal(Token),
}

pub struct Parser {
    postfix: Vec<Token>,
    operators: Vec<Token>,
    symbols: Vec<Symbol>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            postfix: Vec::new(),
            operators: Vec::new(),
            symbols: vec![Symbol::Start],
        }
    }

    pub fn process(&mut self, token: Token) -> bool {
        println!("{:?}", &token);
        if token == Token::Whitespace {
            return true;
        }

        loop {
            if let Some(symbol) = self.symbols.pop() {
                println!("{:?}", &symbol);
                match symbol {
                    Symbol::Start => match token {
                        Token::Number(number) => {
                            self.symbols.push(Symbol::Expression);
                        }
                        Token::LeftParenthesis => {
                            self.symbols.push(Symbol::Expression);
                        }
                        _ => return false,
                    },
                    Symbol::Expression => match token {
                        Token::Number(number) => {
                            self.symbols.push(Symbol::ExpressionTail);
                            self.symbols.push(Symbol::Term);
                        }
                        Token::LeftParenthesis => {
                            self.symbols.push(Symbol::ExpressionTail);
                            self.symbols.push(Symbol::Term);
                        }
                        _ => return false,
                    },
                    Symbol::ExpressionTail => match token {
                        Token::RightParenthesis => {
                            // do nothing but consume the symbol
                        }
                        Token::Addition => {
                            self.symbols.push(Symbol::ExpressionTail);
                            self.symbols.push(Symbol::Term);
                            self.symbols.push(Symbol::Terminal(Token::Addition));
                        }
                        Token::Subtraction => {
                            self.symbols.push(Symbol::ExpressionTail);
                            self.symbols.push(Symbol::Term);
                            self.symbols.push(Symbol::Terminal(Token::Subtraction));
                        }
                        _ => return false,
                    },
                    Symbol::Term => match token {
                        Token::Number(number) => {
                            self.symbols.push(Symbol::TermTail);
                            self.symbols.push(Symbol::Factor);
                        }
                        Token::LeftParenthesis => {
                            self.symbols.push(Symbol::TermTail);
                            self.symbols.push(Symbol::Factor);
                        }
                        _ => return false,
                    },
                    Symbol::TermTail => match token {
                        Token::RightParenthesis => {
                            // do nothing but consume the symbol
                        }
                        Token::Addition => {
                            // do nothing but consume the symbol
                        }
                        Token::Subtraction => {
                            // do nothing but consume the symbol
                        }
                        Token::Multiplication => {
                            self.symbols.push(Symbol::TermTail);
                            self.symbols.push(Symbol::Factor);
                            self.symbols.push(Symbol::Terminal(Token::Multiplication));
                        }
                        Token::Division => {
                            self.symbols.push(Symbol::TermTail);
                            self.symbols.push(Symbol::Factor);
                            self.symbols.push(Symbol::Terminal(Token::Division));
                        }
                        _ => return false,
                    },
                    Symbol::Factor => match token {
                        Token::Number(number) => {
                            self.symbols.push(Symbol::Terminal(Token::Number(number)));
                        }
                        Token::LeftParenthesis => {
                            self.symbols.push(Symbol::Terminal(Token::LeftParenthesis));
                            self.symbols.push(Symbol::Expression);
                            self.symbols.push(Symbol::Terminal(Token::RightParenthesis));
                        }
                        _ => return false,
                    },
                    Symbol::Terminal(terminal) => match terminal {
                        Token::Number(number) => {
                            return true;
                        }
                        Token::LeftParenthesis => {
                            return true;
                        }
                        Token::RightParenthesis => {
                            return true;
                        }
                        Token::Addition => {
                            return true;
                        }
                        Token::Subtraction => {
                            return true;
                        }
                        Token::Multiplication => {
                            return true;
                        }
                        Token::Division => {
                            return true;
                        }
                        _ => return false,
                    },
                }
            } else {
                return false;
            }
        }
    }
}
