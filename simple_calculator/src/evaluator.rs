use crate::{
    lexer::Lexer,
    symbol::{Symbol, Terminal, TerminalClass},
    visitor::Visitor,
};

pub struct Evaluator<'a> {
    lexer: &'a Lexer,
}

impl<'a> Evaluator<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        Self { lexer }
    }

    fn parse_number(&self, terminal: &Terminal) -> f32 {
        self.lexer.get_lexeme(terminal).parse().unwrap()
    }
}

impl<'a> Visitor<f32> for Evaluator<'a> {
    fn visit_terminal(&mut self, terminal: &Terminal) -> f32 {
        match terminal.class() {
            TerminalClass::Number => self.parse_number(terminal),
            _ => unreachable!(),
        }
    }

    fn visit_rule_1(&mut self, components: &[Symbol]) -> f32 {
        let left = self.visit(&components[0]);
        let right = self.visit(&components[2]);
        left + right
    }

    fn visit_rule_2(&mut self, components: &[Symbol]) -> f32 {
        let left = self.visit(&components[0]);
        let right = self.visit(&components[2]);
        left * right
    }

    fn visit_rule_3(&mut self, components: &[Symbol]) -> f32 {
        self.visit(&components[1])
    }

    fn visit_rule_4(&mut self, components: &[Symbol]) -> f32 {
        self.visit(&components[0])
    }
}
