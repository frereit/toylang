#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub toylang);
mod ast;
mod interpreter;

fn main() {
    let parser = toylang::BlockParser::new();
    let stmts = parser.parse(
        r#"
{
    let x = 0;
    while x < 100 {
        if (x % 15) == 0 {
            print[-15];
        }
        if (x % 5) == 0 {
            if x % 3 {
                print[-5];
            }
        }
        if (x % 3) == 0 {
            if x % 5 {
                print[-3];
            }
        }
        if x % 3 {
            if x % 5 {
                print[x];
            }
        }
        let x = x + 1;
    }
}
    "#,
    ).unwrap();
    dbg!(&stmts);
    let mut interpreter = interpreter::ToylangInterpreter::new();
    interpreter.eval_block(&stmts);
}
