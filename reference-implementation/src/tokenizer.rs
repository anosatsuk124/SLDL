use std::panic;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Type(String),
    PredicateName(String),
    Variable(String),
    Atom(String),
    String(String),
    Op(Op),
    Main,
    EOF,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Op {
    OpenCurly,
    CloseCurly,
    OpenParenthesis,
    CloseParenthesis,
    StringConcat,
    DefArrow,
    Desjunctive,
    Colon,
    Comma,
    SentencesDef,
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

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn pop(&mut self) -> Option<char> {
        self.pos += 1;
        self.input.chars().nth(self.pos - 1)
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        loop {
            match self.peek() {
                None => return None,
                Some(c) => match c {
                    'A'..='Z' => return self.tokenize_str(),
                    c if c.is_ascii() => {
                        self.pop();
                        return self.tokenize_sentence();
                    }
                    c if c.is_whitespace() => self.skip_whitespace(),
                    c => panic!("Unexpected char: {}:{}", c, self.pos),
                },
            }
        }
    }

    fn tokenize_sentence(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            match c {
                '-' | '+' | '{' | '}' | '(' | ')' | ':' | ',' | '|' => {
                    return self.tokenize_op();
                }
                '"' | 'A'..='z' => {
                    return self.tokenize_str();
                }
                c if c.is_whitespace() => {
                    self.skip_whitespace();
                }
                c => panic!("Unexpected char: {}:{}", c, self.pos),
            }
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                c if c.is_whitespace() => {
                    self.pop();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn tokenize_op(&mut self) -> Option<Token> {
        if let Some(c) = self.peek() {
            match c {
                '{' => return Some(Token::Op(Op::OpenCurly)),
                '}' => return Some(Token::Op(Op::CloseCurly)),
                '(' => return Some(Token::Op(Op::OpenParenthesis)),
                ')' => return Some(Token::Op(Op::CloseParenthesis)),
                '+' => return Some(Token::Op(Op::StringConcat)),
                ',' => return Some(Token::Op(Op::Comma)),
                ':' => return Some(Token::Op(Op::Colon)),
                '|' => return Some(Token::Op(Op::Desjunctive)),
                '-' => {
                    self.pop();
                    if let Some('>') = self.peek() {
                        return Some(Token::Op(Op::DefArrow));
                    }
                }
                _ => {
                    panic!("Unexpected char: {}:{}", c, self.pos);
                }
            }
        }
        None
    }

    fn tokenize_str(&mut self) -> Option<Token> {
        if let Some(c) = self.peek() {
            let mut s = String::new();
            match c {
                '"' => {
                    self.pop();
                    while let Some(c) = self.peek() {
                        match c {
                            '"' => {
                                return Some(Token::String(s));
                            }
                            _ => {
                                s.push(c);
                                self.pop();
                            }
                        }
                    }
                }
                'A'..='Z' => {
                    s.push(c);
                    self.pop();
                    while let Some(c @ 'a'..='z') = self.peek() {
                        self.pop();
                        s.push(c);
                    }
                    match &*s {
                        "Sentences" => {
                            return Some(Token::Op(Op::SentencesDef));
                        }
                        _ => return Some(Token::PredicateName(s.to_string())),
                    }
                }
                'a'..='z' => {
                    while let Some(c) = self.peek() {
                        match c {
                            'a'..='z' => {
                                s.push(c);
                                self.pop();
                            }
                            _ => {
                                return Some(Token::Variable(s));
                            }
                        }
                    }
                }
                _ => {
                    panic!("Unexpected char: {}:{}", c, self.pos);
                }
            }
        }
        None
    }

    fn peek_token(&mut self) -> &Option<Token> {
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
