extern crate calculator;

use std::io;

// 5 + 4 / 2 * 3

// Evaluate
// [5]
// [5], [+]
// [5,4], [+]
// [5,4], [+,/]
// [5,4,2], [+,/]
// [5,2], [+]
// [5,2], [+,*]
// [5,2,3], [+,*]
// [5,6], [+]
// [11]

// Postfix
// [5]
// [5], [+]
// [5,4], [+]
// [5,4], [+,/]
// [5,4,2], [+,/]
// [5,4,2,/], [+]
// [5,4,2,/], [+,*]
// [5,4,2,/,3], [+,*]
// [5,4,2,/,3,*], [+]
// [5,4,2,/,3,*,+]

// [5] <- 5
// [5,4] <- 4
// [5,4,2] <- 2
// [5,2] <- /
// [5,2,3] <- 3
// [5,6] <- *
// [11] <- +


// Two stacks
// numbers, operators (binary, unary)
// Parsing of strings
// Detection of no more input
// Unwind the final stack to finalize the input
// Error handling
// Tokenizing of input

// Step one is to convert the incoming data into an internal representation of the inputs

// I could use a parsing table as I parse the incoming input to perform syntactic and semantic validation
// It would still be constructing the internal representation
// Tokenize -> Parse -> Semantic Analysis (once it has matched a parse item)
// This means you must first identify what the token type is, forward it to the parser who then makes sense
// of it

// The parser will perform the state transitions and checking against the defined grammar while building the IR
// The parser can have an internal state object that it defers syntactic decisions to based on the state of
// the system
// There just needs to be a Parse Table object that maintains the current state of the system and performs
// transitions

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn main() {
    match read_input() {
        Err(err) => print!("{}", err),
        Ok(input) => {
            let result = calculator::evaluate(&input);
        }
    }
}
