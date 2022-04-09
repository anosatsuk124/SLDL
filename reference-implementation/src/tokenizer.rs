use std::panic;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    TkType(String),
    TkPredicate(String),
    TkVariable(String),
    TkAtom(String),
    TkSentence(String),
    TkMain,
    TkEOF,
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
                    c if c.is_whitespace() => {
                        self.pop();
                    }
                    c @ 'A'..='Z' => {
                        let mut s = String::new();
                        s.push(c);
                        self.pop();
                        while let Some(c @ 'a'..='z') = self.peek() {
                            self.pop();
                            s.push(c);
                        }
                        match &*s {
                            "Sentence" => {
                                return self.tokenize_sentence();
                            }
                            _ => (),
                        }
                    }
                    c if c.is_ascii() => {
                        self.pop();
                        return self.tokenize_sentence();
                    }
                    c => panic!("Unexpected char: {}:{}", c, self.pos),
                },
            }
        }
    }

    fn tokenize_sentence(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            match c {
                '-' => {
                    self.pop();
                    if self.peek() == Some('>') {
                        self.pop();
                    }
                }
                '{' | '}' | ')' => {
                    self.pop();
                }
                '(' | ',' => {
                    self.pop();
                    let mut s = String::new();
                    self.skip_whitespace();
                }
                '`' => {
                    return Some(Token::TkSentence((self.tokenize_str().unwrap())));
                }
                'a'..='z' => {
                    let mut s = String::new();
                    while let Some(c @ 'a'..='z') = self.peek() {
                        s.push(c);
                        self.pop();
                    }
                    return Some(Token::TkVariable(s));
                }
                'A'..='z' => {
                    let mut s = String::new();
                    while let Some(c @ 'A'..='z') = self.peek() {
                        s.push(c);
                        self.pop();
                    }
                    return Some(Token::TkPredicate(s));
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

    fn tokenize_str(&mut self) -> Option<String> {
        while let Some(c) = self.peek() {
            let mut s = String::new();
            match c {
                '`' => {
                    self.pop();

                    while let Some(c) = self.peek() {
                        match c {
                            '`' => {
                                return Some(s);
                            }
                            _ => {
                                s.push(c);
                                self.pop();
                            }
                        }
                    }
                }
                _ => {
                    return Some(s);
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
