#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub toylang);
mod ast;
fn main() {
    let parser = toylang::BlockParser::new();
    dbg!(parser.parse(r#"
{
    let x = 5;
    let y = 10;
    if x > y {
        print x;
    }
}
    "#));
}
