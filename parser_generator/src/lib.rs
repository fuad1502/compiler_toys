pub mod code_gen;
pub mod parse_table_gen;
pub mod yalr_parser;

use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Rule {
    head: NonTerminal,
    symbols: Vec<Symbol>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NonTerminal {
    id: usize,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Terminal {
    End,
    Empty,
    Other(usize),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum TerminalOrRule {
    Terminal(Terminal),
    Rule(Rc<Rule>),
}

impl Rule {
    pub fn num_of_components(&self) -> usize {
        self.symbols.len()
    }

    pub fn head(&self) -> &NonTerminal {
        &self.head
    }
}
