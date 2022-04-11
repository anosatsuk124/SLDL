mod parser;
mod regex;
mod tokenizer;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let code = fs::read_to_string(filepath).unwrap();

    let mut tokenizer = tokenizer::Tokenizer::new(code);
    loop {
        let tokenized = tokenizer.tokenize();
        if tokenized.is_some() {
            println!("{:?}", tokenized);
        } else {
            break;
        }
    }
    // println!("{:?}", tokenizer.tokenize());
    // let mut parser = parser::Parser::new(tokenizer);
    // let node = parser.parse_sentence();
    // println!("{:?}", node);
}
