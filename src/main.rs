mod tokenizer;

fn main() {
    let mut my_tokenizer = tokenizer::Tokenizer::new("what's good dawg!");
    while !my_tokenizer.is_complete() {
        println!("{:?}", my_tokenizer);
        my_tokenizer.read_char();
    }
}
