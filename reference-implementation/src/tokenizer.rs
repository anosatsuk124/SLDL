use std::panic;

#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenKind {
    TkType,
    TkPredicate,
    TkSentenceName(String),
    TkSentenceVar(String),
    TkSentenceDef(String),
    TkMain,
    TkEOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    fn new(kind: TokenKind) -> Self {
        Token { kind }
    }

    pub fn tokenize(code: Vec<String>) -> Vec<Self> {
        let mut code = code.into_iter().peekable();
        let mut tokens = Vec::new();
        while let Some(token) = code.next() {
            match &*token {
                "Type" => (),
                "Predicate" => (),
                "Sentence" => {
                    let token = consume(&mut code, "{").unwrap();
                    tokens.push(Self::new(TokenKind::TkSentenceName(token))); // expect Sentence name
                    while let Some(token) = code.next() {
                        //let token = code.next().unwrap();
                        println!("{:?}", token);
                        match &*token {
                            "->" => {
                                panic!("sentence predicate needs 1 or more variables");
                            }
                            _ => {
                                tokens.push(Self::new(TokenKind::TkSentenceVar(token)));
                                while let Some(token) = code.next() {
                                    match &*token {
                                        "->" => {
                                            let str = consume_str(&mut code);
                                            tokens.push(Self::new(TokenKind::TkSentenceDef(str)));
                                            break;
                                        }
                                        _ => {
                                            tokens.push(Self::new(TokenKind::TkSentenceVar(token)));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "Main" => (),
                _ => (),
            }
        }
        tokens.into_iter().rev().collect()
    }
}

fn consume_str(code: &mut impl Iterator<Item = String>) -> String {
    let mut str = String::new();
    while let Some(token) = code.peekable().peek() {
        match &*token {
            s if s.contains("`") => {
                str.push_str(s);
                while let Some(token) = code.peekable().peek() {
                    match &*token {
                        s if s.contains("`") => {
                            str.push_str(s);
                            break;
                        }
                        x => {
                            str.push_str(x);
                        }
                    }
                }
            }
            _ => break,
        }
    }

    str
}

fn consume(code: &mut impl Iterator<Item = String>, consumed_str: &str) -> Option<String> {
    if let Some(token) = code.next() {
        if token.contains(consumed_str) {
            return code.next();
        } else {
            eprintln!("{:?}", token);
        }
    }
    panic!("could not tokenize");
}

fn expect(code: &mut impl Iterator<Item = String>, expected_str: &str) -> bool {
    if let Some(token) = code.next() {
        if token.contains(expected_str) {
            return true;
        }
    }
    false
}
