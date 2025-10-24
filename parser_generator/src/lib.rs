use core::{cmp::PartialEq, hash::Hash};
use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Terminal {
    End,
    Empty,
    Other(usize),
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct NonTerminal {
    id: usize,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Rule {
    head: NonTerminal,
    symbols: Vec<Symbol>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
struct Item {
    rule: Rc<Rule>,
    position: usize,
    lookahead: Terminal,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct State {
    items: Vec<Item>,
}

pub enum Action {
    Shift(Rc<State>),
    Reduce(Rc<State>),
}

pub struct ParseTableGen {
    rules: Vec<Rc<Rule>>,
    symbols: Vec<Symbol>,
    first_table: HashMap<NonTerminal, Vec<Terminal>>,
    pub states: Vec<Rc<State>>,
    pub goto_table: HashMap<Rc<State>, Vec<(Symbol, Rc<State>)>>,
    pub action_table: HashMap<Rc<State>, HashMap<Terminal, Action>>,
}

impl ParseTableGen {
    pub fn new(rules: Vec<Rule>, symbols: Vec<Symbol>) -> Self {
        let rules = rules.into_iter().map(|r| Rc::new(r)).collect();
        Self {
            rules,
            symbols,
            first_table: HashMap::new(),
            states: vec![],
            goto_table: HashMap::new(),
            action_table: HashMap::new(),
        }
        .create_first_table()
        .create_states()
    }

    fn create_first_table(mut self) -> Self {
        let non_terminals = self.non_terminals();
        loop {
            let mut changed = false;

            for non_terminal in &non_terminals {
                let rules = self.rules_with_head(non_terminal);
                for rule in rules {
                    for symbol in &rule.symbols {
                        match symbol {
                            Symbol::NonTerminal(production) => {
                                if let Some(terminals) = self.first_table.get(&production) {
                                    let terminals = terminals.clone();
                                    for terminal in &terminals {
                                        if self.add_terminal_to_first_table_entry(
                                            non_terminal,
                                            terminal,
                                        ) {
                                            changed = true;
                                        }
                                    }
                                    if !terminals.contains(&Terminal::Empty) {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            Symbol::Terminal(terminal) => {
                                if self.add_terminal_to_first_table_entry(non_terminal, &terminal) {
                                    changed = true;
                                }
                                break;
                            }
                        }
                    }
                }
            }

            if !changed {
                break;
            }
        }
        self
    }

    fn non_terminals(&self) -> Vec<NonTerminal> {
        self.symbols
            .iter()
            .filter_map(|s| match s {
                Symbol::NonTerminal(nt) => Some(nt),
                _ => None,
            })
            .copied()
            .collect()
    }

    fn rules_with_head(&self, non_terminal: &NonTerminal) -> Vec<Rc<Rule>> {
        self.rules
            .iter()
            .filter(|r| r.head == *non_terminal)
            .map(Rc::clone)
            .collect()
    }

    fn copy_non_terminal_first_entries() {
        todo!()
    }

    fn add_terminal_to_first_table_entry(
        &mut self,
        non_terminal: &NonTerminal,
        terminal: &Terminal,
    ) -> bool {
        if let Some(terminals) = self.first_table.get_mut(non_terminal) {
            if terminals.contains(terminal) {
                return false;
            }
            terminals.push(*terminal);
            true
        } else {
            self.first_table.insert(*non_terminal, vec![*terminal]);
            true
        }
    }

    fn create_states(mut self) -> Self {
        let kernel_items = vec![Item {
            rule: self.rules[0].clone(),
            position: 0,
            lookahead: Terminal::End,
        }];
        let state_0 = Rc::new(self.closure(kernel_items));
        self.states.push(state_0.clone());
        let mut unvisited_states = vec![state_0];

        loop {
            if unvisited_states.is_empty() {
                break;
            }
            let state = unvisited_states.pop().unwrap();
            for symbol in &self.symbols {
                if let Some(next_state) = self.goto(&state, *symbol) {
                    let next_state = if let Some(existing_state) =
                        self.states.iter().find(|s| ***s == next_state)
                    {
                        existing_state.clone()
                    } else {
                        let next_state = Rc::new(next_state);
                        self.states.push(next_state.clone());
                        unvisited_states.push(next_state.clone());
                        next_state
                    };
                    Self::add_goto_entry(&mut self.goto_table, &state, symbol, &next_state);
                }
            }
        }
        self
    }

    fn add_goto_entry(
        goto_table: &mut HashMap<Rc<State>, Vec<(Symbol, Rc<State>)>>,
        state: &Rc<State>,
        symbol: &Symbol,
        next_state: &Rc<State>,
    ) {
        if let Some(entries) = goto_table.get_mut(state) {
            entries.push((*symbol, next_state.clone()));
        } else {
            goto_table.insert(state.clone(), vec![(*symbol, next_state.clone())]);
        }
    }

    fn closure(&self, kernel_items: Vec<Item>) -> State {
        let mut items = kernel_items.clone();
        let mut unvisited_items = kernel_items;
        loop {
            if unvisited_items.is_empty() {
                break;
            }
            let item = unvisited_items.pop().unwrap();
            let non_terminal = match item.symbol_right_of_dot() {
                Some(Symbol::NonTerminal(non_terminal)) => non_terminal,
                _ => continue,
            };
            let lookaheads = self.first(item.symbol_after_right_of_dot().copied(), item.lookahead);
            for rule in self.rules.iter().filter(|r| r.head == *non_terminal) {
                for lookahead in &lookaheads {
                    let new_item = Item {
                        rule: rule.clone(),
                        position: 0,
                        lookahead: *lookahead,
                    };
                    if !items.contains(&new_item) {
                        items.push(new_item.clone());
                        unvisited_items.push(new_item);
                    }
                }
            }
        }
        items.sort();
        State { items }
    }

    fn goto(&self, state: &State, symbol: Symbol) -> Option<State> {
        let mut kernel_items = vec![];
        for item in &state.items {
            if item.symbol_right_of_dot() == Some(&symbol) {
                kernel_items.push(Item {
                    rule: item.rule.clone(),
                    position: item.position + 1,
                    lookahead: item.lookahead,
                })
            }
        }
        if kernel_items.is_empty() {
            return None;
        }
        Some(self.closure(kernel_items))
    }

    fn first(&self, symbol: Option<Symbol>, terminal: Terminal) -> Vec<Terminal> {
        match symbol {
            Some(Symbol::Terminal(terminal)) => vec![terminal],
            Some(Symbol::NonTerminal(non_terminal)) => {
                let mut terminals = self.first_table[&non_terminal].clone();
                if self.first_table[&non_terminal].contains(&Terminal::Empty) {
                    terminals.push(terminal);
                }
                terminals
            }
            None => vec![terminal],
        }
    }
}

impl Item {
    fn symbol_right_of_dot(&self) -> Option<&Symbol> {
        self.rule.symbols.get(self.position)
    }

    fn symbol_after_right_of_dot(&self) -> Option<&Symbol> {
        self.rule.symbols.get(self.position + 1)
    }
}

#[cfg(test)]
mod test {
    use core::assert_eq;

    use crate::{NonTerminal, ParseTableGen, Rule, Symbol, Terminal};

    #[test]
    fn test() {
        let s_acc = NonTerminal { id: 0 };
        let s = NonTerminal { id: 1 };
        let e = NonTerminal { id: 2 };
        let c = Terminal::Other(0);
        let d = Terminal::Other(1);

        let rule_acc = Rule {
            head: s_acc,
            symbols: vec![Symbol::NonTerminal(s)],
        };
        let rule_0 = Rule {
            head: s,
            symbols: vec![Symbol::NonTerminal(e), Symbol::NonTerminal(e)],
        };
        let rule_1 = Rule {
            head: e,
            symbols: vec![Symbol::Terminal(c), Symbol::NonTerminal(e)],
        };
        let rule_2 = Rule {
            head: e,
            symbols: vec![Symbol::Terminal(d)],
        };

        let rules = vec![rule_acc, rule_0, rule_1, rule_2];
        let symbols = vec![
            Symbol::NonTerminal(s_acc),
            Symbol::NonTerminal(s),
            Symbol::NonTerminal(e),
            Symbol::Terminal(c),
            Symbol::Terminal(d),
        ];
        let parse_table_gen = ParseTableGen::new(rules, symbols);

        assert_eq!(parse_table_gen.states.len(), 10);
    }
}
