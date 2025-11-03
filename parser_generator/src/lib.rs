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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Priority {
    assigned_priority: Option<usize>,
    is_shift: Option<bool>,
    rule_order: Option<usize>,
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

impl Priority {
    pub fn new(priority: usize) -> Self {
        Self {
            assigned_priority: Some(priority),
            is_shift: None,
            rule_order: None,
        }
    }

    pub fn shift() -> Self {
        Self {
            assigned_priority: None,
            is_shift: Some(true),
            rule_order: None,
        }
    }

    pub fn reduce(rule_order: usize) -> Self {
        Self {
            assigned_priority: None,
            is_shift: Some(false),
            rule_order: Some(rule_order),
        }
    }
}
