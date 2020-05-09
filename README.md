# LR(1) Parser Generator

## Usage

### As a Binary

```bash
lr1-parser "1 + 2 * 3 / 4"
```

Detailed usage:

```bash
lr1-parser 0.1.0
imtsuki <me@qjx.app>
A simple LR(1) parser

USAGE:
    lr1-parser [expr]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <expr>    Expression to be parsed
```

### As a Library

```rust
use lr1_parser::{rule, symbol, Lr1ParserBuilder};

fn main() {
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
        "12".to_owned(),
        "/".to_owned(),
        "34".to_owned(),
        "*".to_owned(),
        "56".to_owned(),
    ]);
}
```
