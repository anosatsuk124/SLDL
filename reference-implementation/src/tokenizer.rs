use crate::regexes::*;
use regex::Regex;
use std::{fmt::format, panic};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Type(String),
    PredicateName(String),
    Variable(String),
    Args(String),
    Atom(String),
    Sentence(String),
    DictName(String),
    DictValue(Vec<Token>),
    Op(String),
    XmlElem(Box<Token>),
    Main,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Tokenizer {
    pos: usize,
    input: String,
    next_token: Option<Token>,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            pos: 0,
            input,
            next_token: None,
        }
    }

    pub fn tokenize(&self) -> Option<Token> {
        match self.input {
            _ => None,
        }
    }

    pub fn apply_regex(&self) -> Vec<Token> {
        let mut v = Vec::new();
        let re = Regex::new(&SENTENCES).unwrap();

        let text = &*self.input;
        println!("{}", text);
        println!("{:?}", re.captures(text));
        v
    }

    pub fn peek_token(&mut self) -> &Option<Token> {
        if self.next_token.is_none() {
            self.next_token = self.tokenize();
        }
        println!("peek: {:?}", self.next_token);
        &self.next_token
    }

    pub fn pop_token(&mut self) -> Option<Token> {
        if self.next_token.is_none() {
            self.next_token = self.tokenize();
        }
        println!("pop: {:?}", self.next_token);
        self.next_token.take()
    }

    fn consume(&mut self, token: Token) -> bool {
        if self.peek_token() == &Some(token) {
            self.next_token.take();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: Token) {
        if self.peek_token().as_ref() == Some(&token) {
            self.next_token.take();
        } else {
            panic!("Expected: {:?}", token);
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_token()
    }
}
