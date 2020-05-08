use std::fmt;
pub use Symbol::*;

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Symbol {
    Nonterminal(String),
    Terminal(String),
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub struct Rule {
    pub left: Symbol,
    pub right: Vec<Symbol>,
}

impl Rule {
    pub fn new(left: &str) -> Rule {
        Rule {
            left: Nonterminal(left.into()),
            right: Vec::new(),
        }
    }

    pub fn push_symbol(mut self, symbol: Symbol) -> Rule {
        self.right.push(symbol);
        self
    }
}

#[macro_export]
macro_rules! symbol {
    ($symbol:ident) => {
        crate::rule::Nonterminal(stringify!($symbol).into())
    };
    ($symbol:literal) => {
        crate::rule::Terminal($symbol.into())
    };
}

#[macro_export]
macro_rules! rule {
    ($left:ident -> $($tail:tt)+) => {{
        $crate::rule!((crate::rule::Rule::new(stringify!($left))) $($tail)*)
    }};
    (($rule:expr) $symbol:ident $($tail:tt)*) => {{
        $crate::rule!(($rule.push_symbol(symbol!($symbol))) $($tail)*)
    }};
    (($rule:expr) $symbol:literal $($tail:tt)*) => {{
        $crate::rule!(($rule.push_symbol(symbol!($symbol))) $($tail)*)
    }};
    (($rule:expr)) => {
        { $rule }
    };
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nonterminal(n) => write!(f, "{}", n),
            Terminal(t) => write!(f, "{:?}", t),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ->", self.left)?;
        for s in &self.right {
            write!(f, " {:?}", s)?;
        }
        Ok(())
    }
}
