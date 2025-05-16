use std::io::{self, Write};
use ast::Ast;
use environment::ShellEnv;
use parser::Parser;

mod lexer;
mod values;
mod environment;
mod ast;
mod parser;

pub struct Shell {
    env: ShellEnv,
    parser: Option<Parser>

}
impl Shell {
    pub fn new() -> Self {
        Shell {
            env: ShellEnv::new(),
            parser: None
        }
    }
    pub fn start(&mut self) {
        loop {
            print!(">>: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_owned();
            if input == "exit" {
                break;
            }
            if input .is_empty() {
                continue;
            }
            self.parser = Some(Parser::new(input));
            println!("{:#?}", self.parse());
        }
    }
    fn parse(&mut self) -> Ast {
        self.parser.take().unwrap().produce_ast()
    }
}

