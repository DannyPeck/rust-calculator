// The calculator will accept String input and return a Result<i32, Error>
// The calculator will own a lexical analyzer and a parser
// The lexical analyzer will consume the input and forward its tokens to the parser
// The parser will maintain an internal state of the world and use a parse table to process the inputs

mod lexer;

pub fn evaluate(input: &String) {
    let mut input_iter = input.trim().chars().peekable();
    let input_iter_ref= &mut input_iter;
    loop {
        match input_iter_ref.peek() {
            None => break,
            Some(_) => {
                match lexer::process(input_iter_ref) {
                    Err(err) => {
                        println!("{:?}", err);
                        break
                    },
                    Ok(token) => {}
                }
            }
        }
    }
}