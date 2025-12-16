enum Token {
    // Structural Symbols
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    ARROW,
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
    IDENTIFIER,
    STRING,
    NUMBER,
    // KEYWORDS
    HOLD,      // create variable
    STRIKE,    // define variable
    WHEN,      // if
    OTHERWISE, // else
    STALK,     // loop
    GIVE,      // return value
    SAY,       // print
    EOF,       // End of File
}
