mod parser;
mod regexes;
mod tokenizer;
use crate::regexes::*;
use regex::Regex;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = &args[1];

    let code = fs::read_to_string(filepath).unwrap();

    let token = tokenizer::Tokenizer::new(code.clone());
    token.apply_regex();
    token.tokenize();
    let split = r#","#;
    let text = code;
    // println!("{:?}", tokenizer.tokenize());
    // let mut parser = parser::Parser::new(tokenizer);
    // let node = parser.parse_sentence();
    // println!("{:?}", node);
}
