use super::ast::{Ast, BinaryOp, Expr, UnaryOp};
use super::lexer::{Bool, Token};
use super::values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,      // e.g., for `if`, `while`, etc. (special forms)
    OrElse,      // ||
    AndThen,     // &&
    Assignment,  // =
    Pipe,        // |
    Redirect,    // ->, >>, <-
    CommandArg,  // used for collecting args of a command call
    Or,          // or
    And,         // and
    Equality,    // ==, !=
    Comparison,  // <, <=, >, >=
    Term,        // +, -
    Factor,      // *, /, %
    Unary,       // !, not
    CallOrIndex, // function calls, array indexing: f(x), x[0]
    Primary,     // literals, identifiers, (grouped), blocks
}
impl Precedence {
    pub fn next_higher(self) -> Precedence {
        use Precedence::*;
        match self {
            Lowest => OrElse,
            OrElse => AndThen,
            AndThen => Assignment,
            Assignment => Pipe,
            Pipe => Redirect,
            Redirect => CommandArg,
            CommandArg => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => CallOrIndex,
            CallOrIndex => Primary,
            Primary => Primary,
        }
    }
}
macro_rules! match_literals {
    ($x:pat) => {
        Token::Number($x)
            | Token::Bool($x)
            | Token::String($x)
            | Token::Identifier($x)
            | Token::ShellWord($x)
    };
}
macro_rules! match_keywords {
    () => {
        Token::If | Token::While | Token::Function | Token::For
    };
}
macro_rules! match_shell_ops {
    () => {
        Token::AndThen
            | Token::OrElse
            | Token::Pipe
            | Token::RedirectIn
            | Token::RedirectOut
            | Token::RedirectOutAppend
    };
}
macro_rules! match_ops {
    () => {
        Token::Equals
            | Token::Add
            | Token::Sub
            | Token::Mul
            | Token::Div
            | Token::Mod
            | Token::And
            | Token::Or
            | Token::Not
    };
}
macro_rules! match_open_groupers {
    () => {
        Token::OpenParen | Token::OpenBrace | Token::OpenBracket
    };
}
macro_rules! match_close_groupers {
    () => {
        Token::CloseParen | Token::CloseBrace | Token::CloseBracket
    };
}
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }
    fn at(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn eat(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }
    fn expect(&mut self, token: Token, err: &str) -> &Token {
        self.current += 1;
        let prev = &self.tokens[self.current - 1];
        if *prev != token {
            panic!(
                "Parser Error:\n{}\nExpected: {:?}\nFound: {:?}",
                err, token, prev
            )
        }
        prev
    }
    fn peek_next(&self) -> Option<&Token> {
        if self.current + 2 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
        } else {
            None
        }
    }
    fn peek_prev(&self) -> Option<&Token> {
        if self.current > 0 {
            Some(&self.tokens[self.current - 1])
        } else {
            None
        }
    }
    fn is_arg(&self) -> bool {
        matches!(self.at(), match_literals!(_))
    }
    pub fn produce_ast(&mut self, tokens: Vec<Token>) -> Ast {
        self.tokens = tokens;
        println!("{:#?}", self.tokens);
        let ast = Ast {
            expr: self.parse_expression(Precedence::Lowest),
        };
        self.tokens.clear();
        self.current = 0;
        ast
    }
    fn collect_args(&mut self, prec: Precedence) -> Vec<Expr> {
        let mut args = Vec::new();
        self.eat();
        while self.is_arg() {
            if self.get_precedence() < prec {
                break;
            }
            args.push(self.parse_expression(prec));
        }
        args
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Expr {
        let mut left = self.nud();

        while precedence < self.get_precedence() || precedence == Precedence::CallOrIndex {
            left = self.led(left);
        }
        left
    }
    fn nud(&mut self) -> Expr {
        match self.at().clone() {
            Token::Number(x) => {
                self.eat();
                Expr::Literal(Value::Number(x.to_owned()))
            }
            Token::String(x) => {
                self.eat();
                Expr::Literal(Value::String(x.to_string()))
            }
            Token::Bool(x) => {
                self.eat();
                Expr::Literal(Value::Boolean(match x {
                    Bool::True => true,
                    Bool::False => false,
                }))
            }
            Token::Identifier(x) => {
                if let Some(match_literals!(_)) = self.peek_next() {
                    if let Some(prev) = self.peek_prev() {
                        match prev {
                            Token::Newline
                            | match_shell_ops!()
                            | match_keywords!()
                            | match_open_groupers!()
                            | Token::Comma => {
                                let args = self.collect_args(Precedence::CommandArg);

                                Expr::CommandCall {
                                    command: Box::new(Expr::Identifier(x)),
                                    args,
                                }
                            }
                            _ => {
                                self.eat();
                                Expr::Identifier(x)
                            }
                        }
                    } else {
                        let args = self.collect_args(Precedence::CommandArg);
                        Expr::CommandCall {
                            command: Box::new(Expr::Identifier(x)),
                            args,
                        }
                    }
                } else {
                    self.eat();
                    Expr::Identifier(x)
                }
            }
            Token::ShellWord(x) => {
                if let Some(match_literals!(_)) = self.peek_next() {
                    if let Some(prev) = self.peek_prev() {
                        match prev {
                            Token::Newline
                            | match_shell_ops!()
                            | match_keywords!()
                            | match_open_groupers!()
                            | Token::Comma => {
                                let args = self.collect_args(Precedence::CommandArg);
                                Expr::CommandCall {
                                    command: Box::new(Expr::ShellWord(x)),
                                    args,
                                }
                            }
                            _ => {
                                self.eat();
                                Expr::ShellWord(x)
                            }
                        }
                    } else {
                        let args = self.collect_args(Precedence::CommandArg);
                        Expr::CommandCall {
                            command: Box::new(Expr::ShellWord(x)),
                            args,
                        }
                    }
                } else {
                    self.eat();
                    Expr::ShellWord(x)
                }
            }
            Token::Function => todo!(),
            Token::If => todo!(),
            Token::While => todo!(),
            Token::For => todo!(),
            Token::OpenParen => {
                self.eat();
                let expr = self.parse_expression(Precedence::Lowest);
                self.expect(Token::CloseParen, "Expected ')' after expression");
                expr
            }
            Token::OpenBrace => {
                let mut expression = Vec::new();
                self.eat();
                if *self.at() == Token::Newline {
                    self.eat();
                }

                while *self.at() != Token::CloseBrace {
                    let expr = self.parse_expression(Precedence::Lowest);
                    expression.push(expr);

                    if *self.at() == Token::Newline {
                        self.eat();
                    }
                }
                self.expect(Token::CloseBrace, "Expected closing brace");
                Expr::Block(expression)
            }
            Token::OpenBracket => todo!(),
            Token::Not => {
                self.eat();
                let right = self.parse_expression(Precedence::Unary);
                Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(right),
                }
            }
            _ => panic!("Unexpected token in nud: {:?}", self.at()),
        }
    }
    fn led(&mut self, left: Expr) -> Expr {
        macro_rules! return_op {
            ($x:expr) => {{
                let prec = self.get_precedence();
                self.eat();
                let right = self.parse_expression(prec.next_higher());
                Expr::BinaryOp {
                    left: Box::new(left),
                    op: $x,
                    right: Box::new(right),
                }
            }};
        }
        let token = self.at().clone();
        match token {
            Token::AndThen => {
                let prec = self.get_precedence();
                self.eat();
                let right = self.parse_expression(prec.next_higher());
                Expr::AndThen {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::OrElse => {
                let prec = self.get_precedence();
                self.eat();
                let right = self.parse_expression(prec.next_higher());
                Expr::OrElse {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::Add => return_op!(BinaryOp::Add),
            Token::Sub => return_op!(BinaryOp::Sub),
            Token::Mul => return_op!(BinaryOp::Mul),
            Token::Div => return_op!(BinaryOp::Div),
            Token::Mod => return_op!(BinaryOp::Mod),
            Token::EqualTo => return_op!(BinaryOp::Eq),
            Token::NotEqualTo => return_op!(BinaryOp::Neq),
            Token::LessThan => return_op!(BinaryOp::Lt),
            Token::GreaterThan => return_op!(BinaryOp::Gt),
            Token::LessThanEqual => return_op!(BinaryOp::Le),
            Token::GreaterThanEqual => return_op!(BinaryOp::Ge),
            Token::And => return_op!(BinaryOp::And),
            Token::Or => return_op!(BinaryOp::Or),
            Token::Equals => {
                if let Expr::Identifier(name) = left {
                    self.eat();
                    let value = self.parse_expression(Precedence::Lowest);
                    Expr::Assign {
                        name,
                        value: Box::new(value),
                    }
                } else {
                    panic!("Invalid assignment target")
                }
            }
            Token::OpenParen => {
                self.eat();
                let mut args = Vec::new();
                if *self.at() != Token::CloseParen {
                    loop {
                        args.push(self.parse_expression(Precedence::Lowest));
                        match self.at() {
                            Token::Comma => {
                                self.eat();
                            }
                            Token::CloseParen => break,
                            other => panic!("Unexpected token {:?} in argument list", other),
                        }
                    }
                }
                self.expect(Token::CloseParen, "Expected ')' after function arguments.");
                Expr::Call {
                    callee: Box::new(left),
                    args,
                }
            }
            Token::Number(_) => todo!(),
            Token::String(_) => todo!(),
            Token::Identifier(_) => todo!(),
            Token::OpenBracket => todo!(), // indexing
            Token::Comma => todo!(),
            _ => panic!("Unexpected token in led: {:?}", self.at()),
        }
    }
    fn get_precedence(&self) -> Precedence {
        self.get_token_precedence(self.at())
    }
    fn get_token_precedence(&self, token: &Token) -> Precedence {
        use Precedence::*;
        match token {
            Token::OrElse => OrElse,
            Token::AndThen => AndThen,

            Token::Equals => Assignment,
            Token::Pipe => Pipe,

            Token::RedirectIn | Token::RedirectOut | Token::RedirectOutAppend => Redirect,

            match_literals!(_) => CommandArg,

            Token::Or => Or,
            Token::And => And,

            Token::EqualTo | Token::NotEqualTo => Equality,

            Token::GreaterThan
            | Token::GreaterThanEqual
            | Token::LessThan
            | Token::LessThanEqual => Comparison,

            Token::Add | Token::Sub => Term,
            Token::Mul | Token::Div | Token::Mod => Factor,

            Token::OpenParen | Token::OpenBracket => CallOrIndex,

            _ => Lowest,
        }
    }
}
