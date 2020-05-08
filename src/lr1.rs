use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::parser::*;
use crate::rule::*;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Lr1Item(pub Rule, pub usize, pub Symbol);

type First = HashSet<Symbol>;
type Closure = HashSet<Lr1Item>;

#[derive(Default)]
pub struct Lr1ParserBuilder {
    rules: Vec<Rule>,
    starting_symbol: Option<Symbol>,
}

impl Lr1ParserBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: Rule) -> &mut Self {
        println!("Add Rule: {}", rule);
        self.rules.push(rule);
        self
    }

    pub fn starting_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.starting_symbol = Some(symbol);
        self
    }

    pub fn build(&mut self) -> Parser {
        println!("# Phase 0. Prepend Starting Rule");

        let starting_rule;

        if let Some(Nonterminal(n)) = self.starting_symbol.clone() {
            starting_rule = Rule {
                left: Nonterminal(n.clone() + "'"),
                right: vec![Nonterminal(n.clone())],
            };
            self.add_rule(starting_rule.clone());
        } else {
            panic!("No starting symbol specified");
        }

        println!("# Phase 1. Compute First Sets");

        let ns: HashSet<Symbol> = self.rules.iter().map(|rule| rule.left.clone()).collect();

        let mut firsts: HashMap<Symbol, First> = HashMap::new();

        for n in ns {
            firsts.insert(n, HashSet::new());
        }

        loop {
            let prev_firsts = firsts.clone();
            for rule in &self.rules {
                let left = &rule.left;
                let right = &rule.right;
                match right.get(0) {
                    Some(symbol) => {
                        let first = firsts.get_mut(left).unwrap();
                        if matches!(symbol, Terminal(_)) {
                            first.insert(symbol.clone());
                        } else {
                            first.extend(
                                prev_firsts
                                    .get(symbol)
                                    .unwrap()
                                    .into_iter()
                                    .map(|i| i.clone()),
                            );
                        }
                    }
                    None => unreachable!(),
                }
            }
            if prev_firsts == firsts {
                break;
            }
        }

        println!("First Sets: {:?}", firsts);

        println!("# Phase 2. Compute LR(1) Closures and go(I, S) Function");

        let mut closures: Vec<Closure> = Vec::new();
        let mut go: HashMap<(usize, Symbol), usize> = HashMap::new();

        let mut starting_kernel: Closure = HashSet::new();

        starting_kernel.insert(Lr1Item(starting_rule.clone(), 0, symbol!("$")));

        let starting_closure = self.compute_closure(starting_kernel, &firsts);

        closures.push(starting_closure);
        let mut index: usize = 0;

        while index < closures.len() {
            let this = &closures[index];

            let edges: HashSet<Symbol> = this
                .iter()
                .filter_map(|item| item.0.right.get(item.1))
                .map(|s| s.clone())
                .collect();

            for edge in edges {
                let this = &closures[index];

                let kernel: Closure = this
                    .iter()
                    .filter(|item| {
                        if let Some(s) = item.0.right.get(item.1) {
                            if s == &edge {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .map(|item| Lr1Item(item.0, item.1 + 1, item.2))
                    .collect();

                let closure = self.compute_closure(kernel, &firsts);

                match closures.iter().position(|c| c == &closure) {
                    None => {
                        closures.push(closure);
                        go.insert((index, edge.clone()), closures.len() - 1);
                    }
                    Some(pos) => {
                        go.insert((index, edge.clone()), pos);
                    }
                }
            }

            index += 1;
        }

        println!("LR(1) closures count: {}", closures.len());

        println!("# Phase 3. Construct LR(1) Table");

        println!("# Phase 3.1. Construct Action Table");

        let mut action: HashMap<(usize, Symbol), Action> = HashMap::new();

        for (i, closure) in closures.iter().enumerate() {
            for item in closure {
                match item.0.right.get(item.1) {
                    None => {
                        if item.0.left == starting_rule.left {
                            action.insert((i, item.2.clone()), Accept);
                        } else {
                            let index = self.rules.iter().position(|r| r == &item.0).unwrap();
                            action.insert((i, item.2.clone()), Reduce(index));
                        }
                    }
                    Some(s) => {
                        if matches!(s, Terminal(_)) {
                            action.insert((i, s.clone()), Shift(go[&(i, s.clone())]));
                        }
                    }
                }
            }
        }

        println!("action = {:?}", action);

        println!("# Phase 3.2. Construct Goto Table");

        let mut goto: HashMap<(usize, Symbol), usize> = HashMap::new();

        for ((i, s), j) in go {
            if matches!(s, Nonterminal(_)) {
                assert_eq!(
                    goto.insert((i, s.clone()), j),
                    None,
                    "Conflicting items in goto table"
                );
            }
        }

        println!("goto = {:?}", goto);

        Parser {
            action,
            goto,
            rules: self.rules.clone(),
        }
    }

    fn compute_closure(&self, kernel: Closure, firsts: &HashMap<Symbol, First>) -> Closure {
        let mut closure = kernel;

        loop {
            let prev_closure = closure.clone();
            for item in &prev_closure {
                match item.0.right.get(item.1) {
                    None => (/* Reduce Item */),
                    Some(s) => {
                        if matches!(s, Nonterminal(_)) {
                            let lookaheads = match item.0.right.get(item.1 + 1) {
                                None => vec![item.2.clone()],
                                Some(f) => {
                                    if matches!(f, Nonterminal(_)) {
                                        firsts.get(f).unwrap().clone().into_iter().collect()
                                    } else {
                                        vec![f.clone()]
                                    }
                                }
                            };
                            for rule in &self.rules {
                                if &rule.left == s {
                                    for lookahead in &lookaheads {
                                        closure.insert(Lr1Item(rule.clone(), 0, lookahead.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if prev_closure == closure {
                break;
            }
        }
        closure
    }
}

impl fmt::Debug for Lr1Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ->", self.0.left)?;
        for (i, s) in self.0.right.iter().enumerate() {
            if i == self.1 {
                write!(f, " ·")?;
            }
            write!(f, " {:?}", s)?;
        }
        if self.1 == self.0.right.len() {
            write!(f, " ·")?;
        }
        write!(f, ", {:?}", self.2)?;
        Ok(())
    }
}

pub enum ParseTree {
    Nonterminal(),
}
