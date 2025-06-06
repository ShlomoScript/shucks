use std::error::Error;

use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Bool {
    True,
    False,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // literal types
    Number(f64),
    Bool(Bool),
    String(String),
    Identifier(String),
    ShellWord(String), // things like args and paths

    // keywords
    If,
    While,
    Function,
    For,

    // grouping operators
    Equals,            // =
    OpenParen,         // (
    CloseParen,        // )
    OpenBrace,         // {
    CloseBrace,        // }
    OpenBracket,       // [
    CloseBracket,      // ]
    Add,               // +
    Sub,               // -
    Mul,               // *
    Div,               // /
    Mod,               // %
    And,               // and
    Or,                // or
    AndThen,           // &&
    OrElse,            // ||
    Pipe,              // |
    RedirectIn,        // <-
    RedirectOut,       // ->
    RedirectOutAppend, // >>
    GreaterThan,       // >
    GreaterThanEqual,  // >=
    LessThan,          // <
    LessThanEqual,     // <=
    EqualTo,           // ==
    NotEqualTo,        // !=

    // unary
    Not, // not / !

    // other
    Comma,   // ,
    Newline, // \n
    Eof,     // end of file
}
pub struct Lexer {
    number_regex: Regex,
    identifier_regex: Regex,
}
impl Lexer {
    pub fn new() -> Self {
        Self {
            number_regex: Regex::new(r"^-?\d+(\.\d+)?$").unwrap(),
            identifier_regex: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap(),
        }
    }

    pub fn tokenize(&self, source_code: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();
        let mut src = source_code.chars().peekable();
        macro_rules! push_next {
            ($x:expr) => {{
                tokens.push($x);
                src.next();
            }};
        }
        while let Some(current) = src.peek() {
            match current {
                '(' => push_next!(Token::OpenParen),
                ')' => push_next!(Token::CloseParen),
                '[' => push_next!(Token::OpenBracket),
                ']' => push_next!(Token::CloseBracket),
                '{' => push_next!(Token::OpenBrace),
                '}' => push_next!(Token::CloseBrace),
                ',' => push_next!(Token::Comma),
                '!' => push_next!(Token::Not),
                ' ' | '\t' => {
                    src.next();
                }
                '\n' => push_next!(Token::Newline),
                '"' => {
                    let mut string = String::new();
                    src.next();
                    while let Some(next) = src.peek() {
                        if *next != '"' {
                            string.push(*next);
                            src.next();
                        } else {
                            src.next();
                            break;
                        }
                    }
                    tokens.push(Token::String(string));
                }
                _ if current.is_alphanumeric() || "_'+-*/%=<>&|!.".contains(*current) => {
                    let mut word = String::new();
                    while let Some(next) = src.peek() {
                        if "_'+-*/%=<>&|!.".contains(*next) || next.is_alphanumeric() {
                            word.push(*next);
                            src.next();
                        } else {
                            break;
                        }
                    }
                    match word.as_str() {
                        "+" => tokens.push(Token::Add),
                        "-" => tokens.push(Token::Sub),
                        "*" => tokens.push(Token::Mul),
                        "/" => tokens.push(Token::Div),
                        "%" => tokens.push(Token::Mod),
                        "=" => tokens.push(Token::Equals),
                        "|" => tokens.push(Token::Pipe),
                        "||" => tokens.push(Token::OrElse),
                        "&&" => tokens.push(Token::AndThen),
                        ">" => tokens.push(Token::GreaterThan),
                        "<" => tokens.push(Token::LessThan),
                        ">=" => tokens.push(Token::GreaterThanEqual),
                        "<=" => tokens.push(Token::LessThanEqual),
                        "==" => tokens.push(Token::EqualTo),
                        "!=" => tokens.push(Token::NotEqualTo),
                        "<-" => tokens.push(Token::RedirectIn),
                        "->" => tokens.push(Token::RedirectOut),
                        ">>" => tokens.push(Token::RedirectOutAppend),
                        "not" => tokens.push(Token::Not),
                        "and" => tokens.push(Token::And),
                        "or" => tokens.push(Token::Or),
                        "if" => tokens.push(Token::If),
                        "while" => tokens.push(Token::While),
                        "function" => tokens.push(Token::Function),
                        "for" => tokens.push(Token::For),
                        "true" => tokens.push(Token::Bool(Bool::True)),
                        "false" => tokens.push(Token::Bool(Bool::False)),
                        _ if word.starts_with('\'') => {
                            tokens.push(Token::ShellWord(word.split_off(1)))
                        }
                        _ if self.number_regex.is_match(&word) => {
                            tokens.push(Token::Number(word.parse()?))
                        }
                        _ if self.identifier_regex.is_match(&word) => {
                            tokens.push(Token::Identifier(word))
                        }
                        _ => tokens.push(Token::ShellWord(word)),
                    }
                }
                x => panic!("unknown token: {:?}", x),
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
