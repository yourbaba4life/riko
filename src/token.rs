#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Token {
    // Structural Symbols
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    // Operators
    PLUS,
    MINUS,
    STAR,
    SLASH,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    LESS,
    BANG,
    // Literals & Identifiers
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),
    // Keywords
    HOLD,      // create variable
    STRIKE,    // define variable
    WHEN,      // if
    OTHERWISE, // else
    STALK,     // loop
    GIVE,      // return value
    SAY,       // print

    // Other
    ILLEGAL,
    EOF, // End of File
}
