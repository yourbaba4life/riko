use crate::tokenizer::Tokenizer;

mod token;
mod tokenizer;

fn main() {
    use std::io::{Write, stdin, stdout};
    let mut input = String::new();

    loop {
        print!("Please enter some text: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        // removing next line
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        println!("You typed: {}", input);
        let mut input_tokenizer = Tokenizer::new(&input);
        let tokens = input_tokenizer.tokenize();

        for token in tokens.iter() {
            print!("{:?} ", token);
        }
        println!();

        input.clear();
    }

    /*
    let mut my_tokenizer = tokenizer::Tokenizer::new("what's good dawg!");
    while !my_tokenizer.is_complete() {
        println!("{:?}", my_tokenizer);
        my_tokenizer.read_char();
    }
    */
}
