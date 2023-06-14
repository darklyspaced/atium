#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use atium::atium::{run_file, run_repl, Atium};
use clap::Parser;
use color_eyre::Result;

// use atium::ast::Expr;

fn main() -> Result<()> {
    color_eyre::install()?;

    // let ast = Expr::Binary(
    //     Box::new(Expr::Unary(
    //         Token::new(TokenType::Minus, String::from("+"), String::from("+"), 0),
    //         Box::new(Expr::Literal(Token::new(
    //             TokenType::Number,
    //             String::from("10"),
    //             String::from("10"),
    //             0,
    //         ))),
    //     )),
    //     Token::new(TokenType::Star, String::from("*"), String::from("*"), 0),
    //     Box::new(Expr::Literal(Token::new(
    //         TokenType::Number,
    //         String::from("50"),
    //         String::from("50"),
    //         0,
    //     ))),
    // );
    //
    // println!("{}", ast);

    let cli = Atium::parse();
    if let Some(file) = cli.script {
        run_file(&file)?;
    } else {
        run_repl()?;
    }

    Ok(())
}
