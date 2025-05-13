use super::values::Value;

pub enum Expr {
    Literal(Value),
    Identifier(String),
    Assign {
        name: String,
        value: Box<Expr>
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>
    },
    Block(Vec<Expr>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>
    },
    AndThen {// cmd1 && cmd2
        left: Box<Expr>,
        right: Box<Expr>
    },
    OrElse {//  cmd1 || cmd2
        left: Box<Expr>,
        right: Box<Expr>
    },
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>
    },
    Return(Box<Expr>)
}


pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or
}

pub enum UnaryOp {
    Neg,
    Not
}

pub struct Ast {
    expr: Expr
}