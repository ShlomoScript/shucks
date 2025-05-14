use std::io::{self, Write};

mod lexer;
mod values;
mod environment;
mod ast;
mod parser;

pub fn shell_loop() {
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
        println!("{:#?}", parser::Parser::new(input).produce_ast());
    }
}
