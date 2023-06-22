use std::error::Error;

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub content: Block,
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Declare {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        cond: Expr,
        then: Block,
        r#else: Block,
    },
    While {
        cond: Expr,
        r#do: Block,
    },
    Expr {
        expr: Expr,
    },
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Stmt::Expr { expr }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(Const),
    Var(String),
    Op(Box<Op>),
    Call { name: String, args: Vec<Expr> },
}

impl From<Const> for Expr {
    fn from(item: Const) -> Self {
        Expr::Const(item)
    }
}

impl From<Op> for Expr {
    fn from(item: Op) -> Self {
        Expr::Op(Box::new(item))
    }
}

impl TryFrom<String> for Expr {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"[a-zA-Z_]+").unwrap();
        if re.is_match(&value) {
            Ok(Expr::Var(value))
        } else {
            Err("Invalid identifier name.".into())
        }
    }
}

impl TryFrom<Val> for Expr {
    type Error = String;
    fn try_from(value: Val) -> Result<Self, Self::Error> {
        match value {
            Val::Const(c) => Ok(c.into()),
            Val::Var(v) => v.try_into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Val {
    Const(Const),
    Var(String),
}

impl From<Const> for Val {
    fn from(c: Const) -> Self {
        Val::Const(c)
    }
}

impl From<String> for Val {
    fn from(v: String) -> Self {
        Val::Var(v)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Integer(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Gt(Expr, Expr),
    Eq(Expr, Expr),
    Lt(Expr, Expr),
    Mod(Expr, Expr),
}
