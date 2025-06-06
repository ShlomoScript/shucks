use super::values::Value;

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Identifier(String),
    ShellWord(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    Block(Vec<Expr>),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
    AndThen {
        // cmd1 && cmd2
        left: Box<Expr>,
        right: Box<Expr>,
    },
    OrElse {
        //  cmd1 || cmd2
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Return(Box<Expr>),
    CommandCall {
        command: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
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
    Or,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub struct Ast {
    pub expr: Expr,
}
