use std::collections::HashMap;

use crate::ast::{Block, Const, Expr, Op, Stmt, Val};

pub struct ToylangInterpreter {
    symbols: HashMap<String, i64>,
}

impl ToylangInterpreter {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn eval_block(&mut self, block: &Block) {
        match block {
            Block::StmtList(stmts) => {
                for stmt in stmts {
                    self.eval_stmt(stmt);
                }
            }
            Block::Stmt(stmt) => {
                self.eval_stmt(stmt);
            }
        }
    }

    fn eval_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign(var_name, expr) => {
                self.symbols.insert(var_name.clone(), self.eval_expr(expr));
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr);
            }
            Stmt::If(cond, block) => {
                if self.eval_expr(cond) != 0 {
                    self.eval_block(block);
                }
            }
            Stmt::While(cond, block) => {
                while self.eval_expr(cond) != 0 {
                    self.eval_block(block);
                }
            }
        }
    }

    fn eval_expr(&self, expr: &Expr) -> i64 {
        match expr {
            Expr::Val(val) => match val {
                Val::Const(c) => match c {
                    Const::Integer(i) => *i,
                },
                Val::Var(var_name) => *self.symbols.get(var_name).unwrap(),
            },
            Expr::Op(op) => match op {
                Op::Add(lhs, rhs) => self.eval_expr(lhs) + self.eval_expr(rhs),
                Op::Sub(lhs, rhs) => self.eval_expr(lhs) - self.eval_expr(rhs),
                Op::Mul(lhs, rhs) => self.eval_expr(lhs) * self.eval_expr(rhs),
                Op::Div(lhs, rhs) => self.eval_expr(lhs) / self.eval_expr(rhs),
                Op::Gt(lhs, rhs) => {
                    if self.eval_expr(lhs) > self.eval_expr(rhs) {
                        1
                    } else {
                        0
                    }
                }
                Op::Eq(lhs, rhs) => {
                    if self.eval_expr(lhs) == self.eval_expr(rhs) {
                        1
                    } else {
                        0
                    }
                }
                Op::Lt(lhs, rhs) => {
                    if self.eval_expr(lhs) < self.eval_expr(rhs) {
                        1
                    } else {
                        0
                    }
                }
                Op::Mod(lhs, rhs) => self.eval_expr(lhs) % self.eval_expr(rhs),
            },
            Expr::Call(func, args) => match func.as_str() {
                "print" => {
                    let mut out = String::new();
                    for arg in args {
                        out.push_str(format!("{:?} ", self.eval_expr(arg)).as_str());
                    }
                    println!("{}", out);
                    out.len() as i64
                }
                _ => panic!("Unknown function {}", func),
            },
        }
    }
}
