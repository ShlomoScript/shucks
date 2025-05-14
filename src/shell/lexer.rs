use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    //literal types
    Number,
    String,
    Identifier,

    //keywords
    Function,// only one so far, will add more later

    //grouping operators
    Equals,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Operator,
    EOF //end of file
}

#[derive(Clone, Debug)]
pub struct Token {
    token_value: String,
    token_type: TokenType
}

impl Token {
    pub fn new(token_value: &str, token_type: TokenType) -> Self {
        Token {
            token_value: token_value.to_string(),
            token_type,
        }
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn value(&self) -> &String {
        &self.token_value
    }
}

fn is_skippable(c: &char) -> bool {
    *c == ' ' || *c == '\n' || *c == '\t'
}

fn make_keywords_map() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert(String::from("function"), TokenType::Function);
    keywords
}

pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut src = source_code.chars().peekable();
    let keywords = make_keywords_map();
    while let Some(current) = src.peek() {
        match current {
            '(' => {
                tokens.push(Token::new("(", TokenType::OpenParen));
                src.next();
            },
            ')' => {
                tokens.push(Token::new(")", TokenType::CloseParen));
                src.next();
            },
            '[' => {
                tokens.push(Token::new("[", TokenType::OpenBracket));
                src.next();
            },
            ']' => {
                tokens.push(Token::new("]", TokenType::CloseBracket));
                src.next();
            },
            '{' => {
                tokens.push(Token::new("{", TokenType::OpenBrace));
                src.next();
            },
            '}' => {
                tokens.push(Token::new("}", TokenType::CloseBrace));
                src.next();
            },
            _ if "+-*/%<>=!&|".contains(*current) => {
                let mut op = String::new();
                op.push(*current);
                src.next();
                if let Some(next) = src.peek() {
                    if "=&|".contains(*next) {
                        op.push(*next);
                        src.next();
                    }
                }
                match op.as_str() {
                    "+" | "-" | "*" | "/" | "%" | "!" | "<" | ">" | "<=" | ">=" | "!=" | "==" | "|" | "&&" | "||" => {
                        tokens.push(Token::new(&op, TokenType::Operator));
                    },
                    "=" => {tokens.push(Token::new(&op, TokenType::Equals))}
                    _ => panic!("unknown operator")
                }
            },
            '"' => {
                let mut string = String::new();
                while let Some(next) = src.peek() {
                    if *next != '"' {
                        string.push(*next);
                        src.next();
                    } else {
                        string.push(*next);
                        src.next();
                        break;
                    }
                }
                tokens.push(Token::new(&string, TokenType::String));
            },
            _ if current.is_digit(10) => {
                let mut num = String::new();
                while let Some(next) = src.peek() {
                    if next.is_digit(10) {
                        num.push(*next);
                        src.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(&num, TokenType::Number));
            },
            _ if current.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(next) = src.peek() {
                    if next.is_alphabetic() {
                        ident.push(*next);
                        src.next();
                    } else {
                        break;
                    }
                }
                if let Some(token_type) = keywords.get(&ident) {
                    tokens.push(Token::new(&ident, token_type.clone()));
                } else if ident == "and".to_string() || ident == "or".to_string() || ident == "not".to_string() {
                    tokens.push(Token::new(&ident, TokenType::Operator));
                } else {
                    tokens.push(Token::new(&ident, TokenType::Identifier));
                }
            },
            _ if is_skippable(current) => {src.next();}
            x => {
                println!("Unrecognized character found: {}", x);
                std::process::exit(1);
            }
        }
    }
    tokens.push(Token::new("EndOfFile", TokenType::EOF));
    tokens
}