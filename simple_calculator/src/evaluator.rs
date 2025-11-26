use crate::{
    lexer::Lexer,
    symbol::{NonTerminal, Symbol, Terminal, TerminalClass},
};

pub struct Evaluator<'a> {
    lexer: &'a Lexer,
}

impl<'a> Evaluator<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        Self { lexer }
    }

    pub fn visit(&mut self, symbol: &Symbol) -> f32 {
        match symbol {
            Symbol::NonTerminal(non_terminal) => self.visit_non_terminal(non_terminal),
            Symbol::Terminal(terminal) => self.visit_terminal(terminal),
        }
    }

    fn visit_non_terminal(&mut self, non_terminal: &NonTerminal) -> f32 {
        match non_terminal.rule.number {
            1 => self.visit_rule_add(&non_terminal.rule.components),
            2 => self.visit_rule_substract(&non_terminal.rule.components),
            3 => self.visit_rule_multiply(&non_terminal.rule.components),
            4 => self.visit_rule_divide(&non_terminal.rule.components),
            5 => self.visit_rule_parenthesize(&non_terminal.rule.components),
            6 => self.visit_rule_number(&non_terminal.rule.components),
            _ => unreachable!(),
        }
    }

    fn visit_terminal(&mut self, terminal: &Terminal) -> f32 {
        match terminal.class() {
            TerminalClass::Number => self.parse_number(terminal),
            _ => unreachable!(),
        }
    }

    fn visit_rule_add(&mut self, components: &[Symbol]) -> f32 {
        let left = self.visit(&components[0]);
        let right = self.visit(&components[2]);
        left + right
    }

    fn visit_rule_substract(&mut self, components: &[Symbol]) -> f32 {
        let left = self.visit(&components[0]);
        let right = self.visit(&components[2]);
        left - right
    }

    fn visit_rule_multiply(&mut self, components: &[Symbol]) -> f32 {
        let left = self.visit(&components[0]);
        let right = self.visit(&components[2]);
        left * right
    }

    fn visit_rule_divide(&mut self, components: &[Symbol]) -> f32 {
        let left = self.visit(&components[0]);
        let right = self.visit(&components[2]);
        left / right
    }

    fn visit_rule_parenthesize(&mut self, components: &[Symbol]) -> f32 {
        self.visit(&components[1])
    }

    fn visit_rule_number(&mut self, components: &[Symbol]) -> f32 {
        self.visit(&components[0])
    }

    fn parse_number(&self, terminal: &Terminal) -> f32 {
        self.lexer.get_lexeme(terminal).parse().unwrap()
    }
}
