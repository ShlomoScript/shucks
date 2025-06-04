use ast::Ast;
use environment::ShellEnv;
use parser::Parser;
use std::io::{self, Write};

mod ast;
mod environment;
mod lexer;
mod parser;
mod values;

pub struct Shell {
    env: ShellEnv,
    parser: Option<Parser>,
}
impl Shell {
    pub fn new() -> Self {
        Shell {
            env: ShellEnv::new(),
            parser: None,
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
                if buffer.is_empty() {
                    continue;
                }
                println!("{buffer}");
                self.parser = Some(Parser::new(buffer.clone()));
                println!("{:#?}", self.parse());
                //println!("{:#?}", lexer::tokenize(&buffer).unwrap());
                buffer.clear();
            }
        }
    }
    fn parse(&mut self) -> Ast {
        self.parser.take().unwrap().produce_ast()
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
            _ => {} // ignore other characters
        }
    }

    stack.is_empty()
}
