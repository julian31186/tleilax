mod types;
mod lexer;
mod parser;
mod errors;

use std::fs;
use crate::lexer::lex;
use crate::parser::parse_tokens_to_ast;

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

    let ast = parse_tokens_to_ast(&tokens);

    println!("AST: {:?}", ast);

    std::process::exit(0);
}
