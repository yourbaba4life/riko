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

#[derive(Debug)]
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position (after current char)
    ch: char,             // current char under examination
}

impl Tokenizer {
    pub fn new(input: &str) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        tokenizer.read_char();
        tokenizer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn is_complete(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)] // This tells Rust to only compile this code when running 'cargo test'
mod tests {
    use super::*; // Allows access to your Lexer and Token types
    use crate::tokenizer::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        // In Rust, we use a Vector of the expected Enums directly
        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut l = Tokenizer::new(input);

        for (i, expected_token) in tests.iter().enumerate() {
            let tok = l.next_token();

            // assert_eq! checks if the two values are equal.
            // If they aren't, it panics and prints a helpful error message.
            assert_eq!(
                tok, *expected_token,
                "tests[{}] - token mismatch. expected={:?}, got={:?}",
                i, expected_token, tok
            );
        }
    }
}
