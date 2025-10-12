use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    // Identifiers and values
    Identifier(String),    // keys / values
    
    // Structure tokens
    Colon,                // :
    Dash,                 // -
    LeftBrace,            // {
    RightBrace,           // }
    LeftBracket,          // [
    RightBracket,         // ]
    Comma,                // ,
    Quote,                // "
    
    // Special tokens
    Newline,
    Indent,               // indentation detected (parser handles level)
    Dedent,               // dedentation detected (parser handles level)
    
    // Block scalars
    LiteralBlock,         // |
    FoldedBlock,          // >
    
    // Anchors
    Anchor(String),       // &name
    Alias(String),        // *name
    Merge,                // <<
}

pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Null,
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}
