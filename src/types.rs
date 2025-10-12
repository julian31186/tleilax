#[derive(Debug)]
pub enum Token {
    // Identifiers and values
    Identifier(String),    // keys / values
    String(String),        // quoted strings
    Number(f64),           // numbers (handle leading zeros)
    Boolean(bool),         // true/false
    Null,                  // ~ or null
    
    // Structure tokens
    Colon,                // :
    Dash,                 // -
    LeftBrace,            // {
    RightBrace,           // }
    LeftBracket,          // [
    RightBracket,         // ]
    Comma,                // ,
    
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
