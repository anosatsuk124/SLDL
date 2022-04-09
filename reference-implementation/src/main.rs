mod tokenizer;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let code = fs::read_to_string(filepath).unwrap();

    let code_splited: Vec<String> = code.split_whitespace().map(|s| s.to_string()).collect();

    println!("{:?}", tokenizer::Token::tokenize(code_splited));
}
