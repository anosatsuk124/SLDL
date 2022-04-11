use regex::Regex;
use std::{fmt::format, panic};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Type(String),
    PredicateName(String),
    Variable(String),
    Args(String),
    Atom(String),
    SentenceDef(String),
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
        let index = r"([a-z]+)";
        let predicate_name = r"([A-Z]+\w*)";
        let sentence_def = r"(.+)";
        let atom = r"\w+";
        let comma = r"\s*,\s*";
        let colon = r"\s*:\s*";
        let def_op = r"\s*->\s*";
        let curly_bracket = (r"\s*\{\s*", r"\s*\}\s*");
        let arg = format!(r"(\w+)");
        let args = format!(r"(\(({Arg}{Comma})*{Arg}\s*)\)", Arg = arg, Comma = comma);
        let predicate = format!(
            r"({PredicateName}{Args})",
            PredicateName = &predicate_name,
            Args = &args,
        );
        let sentence = format!(
            r"({Predicate}{DefOp}{OpenCurlyBracket}(({Index}{Colon}{SentenceDef})+{Comma})*({Index}{Colon}{SentenceDef}){CloseCurlyBracket}|{Predicate}{DefOp}{SentenceDef})",
            Predicate = &predicate,
            DefOp = &def_op,
            OpenCurlyBracket = &curly_bracket.0,
            CloseCurlyBracket = &curly_bracket.1,
            SentenceDef = &sentence_def,
            Index = &index,
            Colon = &colon,
            Comma = &comma,
        );
        let sentences = format!(
            r"(Sentence{OpenCurlyBracket}({Sentence}{Comma})*{Sentence}+{CloseCurlyBracket})",
            Sentence = &sentence,
            OpenCurlyBracket = &curly_bracket.0,
            CloseCurlyBracket = &curly_bracket.1,
            Comma = &comma,
        );
        let program = sentences.clone();

        println!("{}", sentences);
        let re = Regex::new(&*sentences).unwrap();
        let text = &*self.input;

        println!("{:?}", re.captures(text));
        None
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
