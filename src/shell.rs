use ast::Ast;
use environment::ShellEnv;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};

mod ast;
mod environment;
mod lexer;
mod parser;
mod values;

pub struct Shell {
    env: ShellEnv,
    parser: Parser,
    lexer: Lexer,
}
impl Shell {
    pub fn new() -> Self {
        Shell {
            env: ShellEnv::new(),
            parser: Parser::new(),
            lexer: Lexer::new(),
        }
    }
    pub fn start(&mut self) {
        let mut buffer = String::new();
        loop {
            let prompt = if valid_delimiters(&buffer) {
                ">>: "
            } else {
                "... "
            };
            print!("{prompt}");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                println!("Error reading line");
                break;
            }
            buffer.push_str(&line);
            if valid_delimiters(&buffer) {
                if buffer.trim() == "exit" {
                    break;
                }
                if buffer.trim().is_empty() {
                    continue;
                }
                println!("{:#?}", self.parse(buffer.trim()));
                buffer.clear();
            }
        }
    }
    fn parse(&mut self, source_code: &str) -> Ast {
        self.parser
            .produce_ast(self.lexer.tokenize(source_code).unwrap())
    }
}
fn valid_delimiters(input: &str) -> bool {
    let mut stack = Vec::new();

    for ch in input.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}
