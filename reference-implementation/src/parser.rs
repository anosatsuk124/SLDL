use crate::tokenizer::Tokenizer;

#[derive(Debug, PartialEq)]
pub enum Node {
    Atome(String),
    Type(String),
    Predicate {
        name: String,
        args: Vec<Box<Node>>,
    },
    Sentence {
        predicate: Box<Node>,
        def_sentence: String,
    },
}

struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    //    fn parse_sentence(&mut self) -> Option<Node> {
    //
    //}
}
