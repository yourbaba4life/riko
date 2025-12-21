use crate::token::Token;

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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak(&self) -> char {
        if self.read_position < self.input.len() {
            return self.input[self.read_position];
        }
        return '\0';
    }

    fn string(&mut self, tokens: &mut Vec<Token>) {
        let start = self.read_position;
        while self.peak() != '"' && !self.is_at_end() {
            self.read_char();
        }

        self.read_char();

        let value = self.input[start..self.position].iter().collect::<String>();
        tokens.push(Token::STRING(value));
    }

    fn number(&mut self, tokens: &mut Vec<Token>) {
        let start = self.position;
        while self.peak().is_digit(10) && !self.is_at_end() {
            self.read_char();
        }

        println!("start: {}, end: {}", start, self.position);
        let value: f64 = self.input[start..self.position + 1]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();
        tokens.push(Token::NUMBER(value));
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.input.len() > self.position {
            if self.ch.is_whitespace() {
                self.read_char();
                continue;
            }

            println!("{:?}", self);

            match self.ch {
                '(' => tokens.push(Token::LEFT_PAREN),
                ')' => tokens.push(Token::RIGHT_PAREN),
                '{' => tokens.push(Token::LEFT_BRACE),
                '}' => tokens.push(Token::RIGHT_BRACE),
                ',' => tokens.push(Token::COMMA),
                '.' => tokens.push(Token::DOT),
                '+' => tokens.push(Token::PLUS),
                '-' => tokens.push(Token::MINUS),
                '*' => tokens.push(Token::STAR),
                '/' => tokens.push(Token::SLASH),
                '=' => {
                    if self.peak() == '=' {
                        tokens.push(Token::EQUAL_EQUAL)
                    }
                    tokens.push(Token::EQUAL)
                }
                '>' => tokens.push(Token::GREATER),
                '<' => tokens.push(Token::LESS),
                '!' => tokens.push(Token::BANG),
                '"' => self.string(&mut tokens),
                _ => {
                    if self.ch.is_digit(10) {
                        self.number(&mut tokens);
                    }
                }
            }

            self.read_char();
        }

        tokens
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![Token::LEFT_PAREN];

        let mut l = Tokenizer::new(input);

        for (i, expected_token) in tests.iter().enumerate() {
            let tok = l.next_token();

            // assert_eq! checks if the two values are equal.
            assert_eq!(
                tok, *expected_token,
                "tests[{}] - token mismatch. expected={:?}, got={:?}",
                i, expected_token, tok
            );
        }
    }
}
*/
