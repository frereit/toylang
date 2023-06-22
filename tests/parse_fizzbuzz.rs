use ::toylang::toylang;
use ::toylang::ast;

#[test]
fn it_parses_fizzbuzz() {
    let expected = vec![ast::Function {
        name: "main".into(),
        params: vec![],
        content: vec![
            ast::Stmt::Declare {
                name: "x".into(),
                value: ast::Expr::Const(ast::Const::Integer(0)),
            },
            ast::Stmt::While {
                cond: ast::Expr::Op(Box::new(ast::Op::Lt(
                    ast::Expr::Var("x".into()),
                    ast::Expr::Const(ast::Const::Integer(100)),
                ))),
                r#do: vec![
                    ast::Stmt::If {
                        cond: ast::Op::Eq(
                            ast::Op::Mod(
                                ast::Expr::Var("x".into()),
                                ast::Const::Integer(15).into(),
                            )
                            .into(),
                            ast::Const::Integer(0).into(),
                        )
                        .into(),
                        then: vec![ast::Expr::Call {
                            name: "println".into(),
                            args: vec![ast::Const::Integer(-15).into()],
                        }
                        .into()],
                        r#else: vec![ast::Stmt::If {
                            cond: ast::Op::Eq(
                                ast::Op::Mod(
                                    ast::Expr::Var("x".into()),
                                    ast::Const::Integer(5).into(),
                                )
                                .into(),
                                ast::Const::Integer(0).into(),
                            )
                            .into(),
                            then: vec![ast::Expr::Call {
                                name: "println".into(),
                                args: vec![ast::Const::Integer(-5).into()],
                            }
                            .into()],
                            r#else: vec![ast::Stmt::If {
                                cond: ast::Op::Eq(
                                    ast::Op::Mod(
                                        ast::Expr::Var("x".into()),
                                        ast::Const::Integer(3).into(),
                                    )
                                    .into(),
                                    ast::Const::Integer(0).into(),
                                )
                                .into(),
                                then: vec![ast::Expr::Call {
                                    name: "println".into(),
                                    args: vec![ast::Const::Integer(-3).into()],
                                }
                                .into()],
                                r#else: vec![ast::Expr::Call {
                                    name: "println".into(),
                                    args: vec![ast::Expr::Var("x".into())],
                                }
                                .into()],
                            }],
                        }],
                    },
                    ast::Stmt::Assign {
                        name: "x".into(),
                        value: ast::Op::Add(
                            ast::Expr::Var("x".into()),
                            ast::Const::Integer(1).into(),
                        )
                        .into(),
                    },
                ],
            },
        ],
    }];
    let parser = toylang::SourceFileParser::new();
    let parsed = parser
        .parse(
            r#"
        fn main() {
            let x = 0;
            while x < 100 {
                if (x % 15) == 0 {
                    println(-15);
                }
                else { 
                    if (x % 5) == 0 {
                        println(-5);
                    } else { 
                        if (x % 3) == 0 {
                            println(-3);
                        } else {
                            println(x);
                        }
                    } 
                }
                x = x + 1;
            }
        }
            "#,
        )
        .unwrap();
    assert_eq!(parsed, expected);
}
