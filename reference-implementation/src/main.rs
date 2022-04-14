mod parser;
mod tokenizer;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let code = fs::read_to_string(filepath).unwrap();

    let mut token = tokenizer::Tokenizer::new(code);
    loop {
        let tokenized = token.tokenize();
        if !tokenized.is_none() {
            println!("{:?}", tokenized);
        } else {
            break;
        }
    }
}
