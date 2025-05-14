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
    pub fn produce_ast(&mut self) -> Ast {
        Ast {expr: self.parse_expression(Precedence::Lowest)}
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Expr {
        let token = self.eat().clone();
        let mut left = self.nud(&token);

        while precedence < self.get_precedence() {
            let tok = self.eat().clone();
            left = self.led(&tok, left);
        }
        left
    }
    fn nud(&mut self, token: &Token) -> Expr {
        match token.token_type() {
            TokenType::Number => {
                let value = token.value().parse().unwrap();
                Expr::Literal(Value::Number(value))
            },
            TokenType::String => {
                let value = token.value().parse().unwrap();
                Expr::Literal(Value::String(value))
            },
            TokenType::Identifier => {
                Expr::Identifier(token.value().clone())
            },
            TokenType::Function => todo!(),
            TokenType::Equals => todo!(),
            TokenType::OpenParen => {
                let expr = self.parse_expression(Precedence::Lowest);
                self.expect(TokenType::CloseParen, "Expected ')' after expression");
                expr
            },
            TokenType::CloseParen => todo!(),
            TokenType::OpenBrace => todo!(),
            TokenType::CloseBrace => todo!(),
            TokenType::OpenBracket => todo!(),
            TokenType::CloseBracket => todo!(),
            TokenType::Operator if token.value().as_str() == "!" || token.value().as_str() == "-" || token.value().as_str() == "not" => {
                let right = self.parse_expression(Precedence::Unary);
                Expr::UnaryOp {
                    op: match token.value().as_str() {
                        "!" | "not"=> UnaryOp::Not,
                        "-" => UnaryOp::Neg,
                        _ => panic!("It's litteraly impossible for this message to be shown because of the match guard earlier.")
                    },
                    expr: Box::new(right)
                }
            },
            TokenType::EOF => todo!(),
            _ => panic!("Unexpected token in nud: {:?}", token)
        }
    }
    fn led(&mut self, token: &Token, left: Expr) -> Expr {
        match token.token_type() {
            TokenType::Operator if token.value() != "!" => {
                let prec = self.get_token_precedence(token);
                let right = self.parse_expression(prec);
                Expr::BinaryOp {
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
                }
            },
            _ => panic!("Unexpected token in led: {:?}", token)
        }
    }
    fn get_precedence(&self) -> Precedence {
        self.get_token_precedence(self.at())
    }
    fn get_token_precedence(&self, token: &Token) -> Precedence {
        match token.value().as_str() {
            "||" => Precedence::LogicalOr,
            "&&" => Precedence::LogicalAnd,
            "==" | "!=" => Precedence::Equality,
            "<" | "<=" | ">" | ">=" => Precedence::Comparison,
            "+" | "-" => Precedence::Term,
            "*" | "/" | "%" => Precedence::Factor,
            _ => Precedence::Lowest
        }
    }
}