use crate::symbol::{NonTerminal, Symbol, Terminal};

pub trait Visitor<T> {
    fn visit(&mut self, symbol: &Symbol) -> T {
        match symbol {
            Symbol::NonTerminal(non_terminal) => self.visit_non_terminal(non_terminal),
            Symbol::Terminal(terminal) => self.visit_terminal(terminal),
        }
    }

    fn visit_terminal(&mut self, terminal: &Terminal) -> T;

    fn visit_non_terminal(&mut self, non_terminal: &NonTerminal) -> T {
        match non_terminal.rule.number {
            1 => self.visit_rule_1(&non_terminal.rule.components),
            2 => self.visit_rule_2(&non_terminal.rule.components),
            3 => self.visit_rule_3(&non_terminal.rule.components),
            4 => self.visit_rule_4(&non_terminal.rule.components),
            _ => unreachable!(),
        }
    }

    fn visit_rule_1(&mut self, components: &[Symbol]) -> T;
    fn visit_rule_2(&mut self, components: &[Symbol]) -> T;
    fn visit_rule_3(&mut self, components: &[Symbol]) -> T;
    fn visit_rule_4(&mut self, components: &[Symbol]) -> T;
}
