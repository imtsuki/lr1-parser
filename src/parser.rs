use std::collections::HashMap;

use crate::rule::*;

#[derive(Debug)]
pub enum Action {
    Reduce(usize),
    Shift(usize),
    Accept,
}

pub struct Parser {
    pub action: HashMap<(usize, Symbol), Action>,
    pub goto: HashMap<(usize, Symbol), usize>,
    pub rules: Vec<Rule>,
}

impl Parser {
    pub fn parse(&self, mut input: Vec<String>) {
        input.push("$".into());
        let mut index = 0;
        let mut symbol_stack: Vec<Symbol> = vec![];
        let mut state_stack = vec![0usize];

        loop {
            println!(
                "Symbol stack: {:?}, State stack: {:?}",
                symbol_stack, state_stack
            );
            let s = state_stack.last().unwrap();
            let a = &input[index];

            match self.action.get(&(
                *s,
                if a.parse::<u64>().is_ok() {
                    Terminal("num".to_owned())
                } else {
                    Terminal(a.clone())
                },
            )) {
                None => panic!("Parser error: no action for state {}, token {:?}", s, a),
                Some(Shift(s)) => {
                    symbol_stack.push(Terminal(a.clone()));
                    state_stack.push(*s);
                    println!("Action: S{}", s);
                    index += 1;
                }
                Some(Reduce(r)) => {
                    let rule = &self.rules[*r];
                    for _ in 0..rule.right.len() {
                        symbol_stack.pop();
                        state_stack.pop();
                    }
                    println!("Action: R{}: {}", r, rule);
                    let s = *state_stack.last().unwrap();
                    symbol_stack.push(rule.left.clone());
                    state_stack.push(*self.goto.get(&(s, rule.left.clone())).unwrap());
                }
                Some(Accept) => {
                    println!("Action: Accept!");
                    break;
                }
            }
        }
    }
}

pub use Action::*;

pub enum ParseTree {
    Nonterminal(),
}
