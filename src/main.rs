mod types;
mod lexer;
mod errors;

use std::fs;
use crate::lexer::lex;

fn main() {
    let input = fs::read_to_string("../examples/simple.yaml").unwrap_or_else(|e| { return e.to_string() });

    let tokens = match lex(&input) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Tokens: {:?}", tokens);
}
