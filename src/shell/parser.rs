use super::lexer::{tokenize, Token, TokenType};
use super::ast::{Ast, Expr, BinaryOp, UnaryOp};
use super::values::Value;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    LogicalOr,
    LogicalAnd,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary
}
impl Precedence {
    pub fn next_higher(self) -> Precedence {
        match self {
            Precedence::Lowest => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(source_code: String) -> Self {
        let tokens = tokenize(source_code);
        Parser {tokens, current: 0}
    }
    fn not_eof(&self) -> bool {
        self.current < self.tokens.len() && *self.tokens[self.current].token_type() != TokenType::EOF
    }
    fn at(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn eat(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }
    fn expect(&mut self, token_type: TokenType, err: &str) -> &Token {
        self.current += 1;
        let prev = &self.tokens[self.current - 1];
        if prev.value() == "" || *prev.token_type() != token_type {
            panic!("Parser Error:\n{}\nExpected: {:?}\nFound: {:?}", err, token_type, prev)
        }
        prev
    }
    fn check(&self, expected: TokenType) -> bool {
        self.current < self.tokens.len() && *self.tokens[self.current].token_type() == expected
    }
    pub fn produce_ast(&mut self) -> Ast {
        Ast {expr: self.parse_expression(Precedence::Lowest)}
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Expr {
        let mut left = self.nud();

        while precedence < self.get_precedence() || precedence == Precedence::Call {
            left = self.led(left);
        }
        left
    }
    fn nud(&mut self) -> Expr {
        match self.at().token_type() {
            TokenType::Number => {
                let value = self.at().value().parse().unwrap();
                self.eat();
                Expr::Literal(Value::Number(value))
            },
            TokenType::String => {
                let value = self.at().value().clone();//.parse().unwrap();
                self.eat();
                Expr::Literal(Value::String(value))
            },
            TokenType::Identifier => {
                let val = self.at().value().clone();
                self.eat();
                Expr::Identifier(val)
            },
            TokenType::Function => todo!(),
            TokenType::Equals => todo!(),
            TokenType::OpenParen => {
                self.eat();
                let expr = self.parse_expression(Precedence::Lowest);
                self.expect(TokenType::CloseParen, "Expected ')' after expression");
                expr
            },
            TokenType::CloseParen => todo!(),
            TokenType::OpenBrace => todo!(),
            TokenType::CloseBrace => todo!(),
            TokenType::OpenBracket => todo!(),
            TokenType::CloseBracket => todo!(),
            TokenType::Operator if self.at().value().as_str() == "!" || self.at().value().as_str() == "-" || self.at().value().as_str() == "not" => {
                let op = self.at().clone();
                self.eat();
                let right = self.parse_expression(Precedence::Unary);
                Expr::UnaryOp {
                    op: match op.value().as_str() {
                        "!" | "not"=> UnaryOp::Not,
                        "-" => UnaryOp::Neg,
                        _ => panic!("It's litteraly impossible for this message to be shown because of the match guard earlier.")
                    },
                    expr: Box::new(right)
                }
            },
            TokenType::EOF => todo!(),
            _ => panic!("Unexpected token in nud: {:?}", self.at())
        }
    }
    fn led(&mut self, left: Expr) -> Expr {
        let token = self.at().clone();
        match token.token_type() {
            TokenType::Operator if token.value() != "!" && token.value() != "not" => {
                let prec = self.get_precedence();
                self.eat();
                let right = self.parse_expression(prec.next_higher());
                match token.value().as_str() {
                    "&&" => {
                        return Expr::AndThen {
                            left: Box::new(left),
                            right: Box::new(right)
                        }
                    },
                    "||" => {
                        return Expr::OrElse {
                            left: Box::new(left),
                            right: Box::new(right)
                        }
                    },
                    _ => {
                        return Expr::BinaryOp {
                            left: Box::new(left),
                            op: match token.value().as_str() {
                                "+" => BinaryOp::Add,
                                "-" => BinaryOp::Sub,
                                "*" => BinaryOp::Mul,
                                "/" => BinaryOp::Div,
                                "%" => BinaryOp::Mod,
                                "==" => BinaryOp::Eq,
                                "!=" => BinaryOp::Neq,
                                "<" => BinaryOp::Lt,
                                "<=" => BinaryOp::Le,
                                ">" => BinaryOp::Gt,
                                ">=" => BinaryOp::Ge,
                                "and" => BinaryOp::And,
                                "or" => BinaryOp::Or,
                                _ => panic!("If you're seeing this, run")
                            },
                            right: Box::new(right)
                        };
                    }
                }
                
            },
            TokenType::OpenParen => {
                self.eat();
                let mut args = Vec::new();
                if *self.at().token_type() != TokenType::CloseParen {
                    loop {
                        args.push(self.parse_expression(Precedence::Lowest));
                        match self.at().token_type() {
                            TokenType::Comma => {self.eat();},
                            TokenType::CloseParen => break,
                            other => panic!("Unexpected token {:?} in argument list", other)
                        }
                    }
                }
                self.expect(TokenType::CloseParen, "Expected ')' after function arguments.");
                Expr::Call {
                    callee: Box::new(left),
                    args
                }
            },
            _ => panic!("Unexpected token in led: {:?}", self.at())
        }
    }
    fn get_precedence(&self) -> Precedence {
        self.get_token_precedence(self.at())
    }
    fn get_token_precedence(&self, token: &Token) -> Precedence {
        match token.token_type() {
            TokenType::Operator => match token.value().as_str() {
                "||" => Precedence::LogicalOr,
                "&&" => Precedence::LogicalAnd,
                "==" | "!=" => Precedence::Equality,
                "<" | "<=" | ">" | ">=" => Precedence::Comparison,
                "+" | "-" => Precedence::Term,
                "*" | "/" | "%" => Precedence::Factor,
                _ => Precedence::Lowest
            }
            TokenType::OpenParen => Precedence::Call,
            _ => Precedence::Lowest
        }
        
    }
}