use std::fmt::Error;

use crate::types::Token;
use crate::types::Value;

pub fn parse_tokens_to_ast(tokens: &Vec<Token>) -> Result<Value, Error> {
    return Err(Error::default());
}