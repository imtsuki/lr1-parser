#[macro_use]
pub mod rule;
pub mod lr1;
pub mod parser;

use lr1::Lr1ParserBuilder;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opt {
    #[structopt(help = "Expression to be parsed")]
    expr: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let parser = Lr1ParserBuilder::new()
        .add_rule(rule!(E -> E "+" T))
        .add_rule(rule!(E -> E "-" T))
        .add_rule(rule!(E -> T))
        .add_rule(rule!(T -> T "*" F))
        .add_rule(rule!(T -> T "/" F))
        .add_rule(rule!(T -> F))
        .add_rule(rule!(F -> "(" E ")"))
        .add_rule(rule!(F -> "num"))
        .starting_symbol(symbol!(E))
        .build();

    parser.parse(vec![
        "num".into(),
        "+".into(),
        "num".into(),
        "*".into(),
        "num".into(),
    ]);

    let _expr = if let Some(expr) = opt.expr {
        expr
    } else {
        let expr = "(3+(2/5)-3)";
        println!("No expression specified, defaulting to {}", expr);
        expr.into()
    };
}
