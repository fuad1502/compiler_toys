use crate::{NonTerminal, Rule, Symbol, Terminal};

pub struct YalrFile {
    pub terminals: Vec<(Terminal, String)>,
    pub non_terminals: Vec<(NonTerminal, String)>,
    pub rules: Vec<Rule>,
}

impl YalrFile {
    pub fn example() -> Self {
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

        let non_terminal_names = vec!["SAcc".to_string(), "S".to_string(), "E".to_string()];
        let terminal_names = vec!["C".to_string(), "D".to_string(), "End".to_string()];
        let terminals = vec![c, d, Terminal::End];
        let non_terminals = vec![s_acc, s, e];
        let rules = vec![rule_acc, rule_0, rule_1, rule_2];

        YalrFile {
            terminals: terminals.into_iter().zip(terminal_names).collect(),
            non_terminals: non_terminals.into_iter().zip(non_terminal_names).collect(),
            rules,
        }
    }
}
