use std::collections::HashMap;

use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug, PartialEq)]
pub enum Node {
    Atom(String),
    Type(String),
    String(String),
    Predicate {
        name: String,
        args: Box<Node>,
    },
    Variables(HashMap<String, Option<String>>),
    Arg(String),
    Sentence {
        predicate: Box<Node>,
        sentence_def: Vec<Node>,
    },
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    // fn parse_struct(&mut self) -> Option<Node> {
    //     if let Some()
    // }

    pub fn parse_sentence(&mut self) -> Option<Node> {
        if let Some(Token::PredicateName(s)) = self.tokenizer.peek_token() {
            let mut vec = Vec::new();
            let mut map = HashMap::new();
            let predicate_name = s.clone();

            self.tokenizer.pop_token();

            while let Some(Token::Args(s)) = self.tokenizer.peek_token() {
                map.insert(s.clone(), None);
                self.tokenizer.pop_token();
            }

            if Some(Token::Op("->".to_string())) == self.tokenizer.peek_token().clone() {
                self.tokenizer.pop_token();

                while let Some(token) = self.tokenizer.peek_token() {
                    match token {
                        Token::Sentence(s) => {
                            let sentence_def = Node::String(s.clone());
                            self.tokenizer.pop_token();
                            vec.push(sentence_def);
                        }
                        Token::Op(op) if op == "+" => {
                            self.tokenizer.pop_token();
                            if let Some(s) = self.tokenizer.peek_token() {
                                match s {
                                    Token::Args(s) => {
                                        let arg = Node::Arg(s.clone());
                                        vec.push(arg);
                                        self.tokenizer.pop_token();
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => {
                            self.tokenizer.pop_token();
                            return Some(Node::Sentence {
                                predicate: Box::new(Node::Predicate {
                                    name: predicate_name,
                                    args: Box::new(Node::Variables(map)),
                                }),
                                sentence_def: vec,
                            });
                        }
                    }
                }
            } else {
                self.tokenizer.pop_token();
            }
            None
        } else {
            None
        }
    }
}
