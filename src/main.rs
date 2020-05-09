use lr1_parser::{rule, symbol, Lr1ParserBuilder};

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

    let expr = if let Some(expr) = opt.expr {
        expr
    } else {
        let expr = "(12+(34/56)-78)";
        println!("No expression specified, defaulting to {}", expr);
        expr.into()
    };

    let mut tokens = vec![];
    let mut num_buf = String::new();

    for ch in expr.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !num_buf.is_empty() {
                    tokens.push(num_buf.to_owned());
                    num_buf.clear();
                }
                tokens.push(ch.to_string());
            }
            ch if ch.is_ascii_digit() => {
                num_buf.push(ch);
            }
            ch if ch.is_whitespace() => {
                if !num_buf.is_empty() {
                    tokens.push(num_buf.to_owned());
                    num_buf.clear();
                }
            }
            _ => panic!("Lexer error: Unexpected character {:?}", ch),
        }
    }

    if !num_buf.is_empty() {
        tokens.push(num_buf.to_owned());
        num_buf.clear();
    }

    println!("Now start parsing {:?}", tokens);

    parser.parse(tokens);
}
