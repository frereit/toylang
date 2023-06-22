#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    StmtList(Vec<Stmt>),
    Stmt(Box<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assign(String, Box<Expr>),
    Expr(Box<Expr>),
    If(Box<Expr>, Box<Block>),
    While(Box<Expr>, Box<Block>), 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Val(Val),
    Op(Op),
    Call(String, Vec<Box<Expr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Const(Const),
    Var(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Integer(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}
