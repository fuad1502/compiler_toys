// E ->
// | E '+' E { E.val = E1.val + E2.val }
// | E '*' E { E.val = E1.val + E2.val }
// | '(' E ')' { E.val = E1.val + E2.val }
// | number

use std::collections::HashMap;

struct Lexer {
    position: usize,
    source: Vec<char>,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Lexer {
            position: 0,
            source: source.chars().collect(),
        }
    }

    fn next(&mut self) -> Result<Terminal, &'static str> {
        let terminal = self.peek()?;
        self.position += 1;
        Ok(terminal)
    }

    fn peek(&self) -> Result<Terminal, &'static str> {
        if self.position > self.source.len() - 1 {
            Ok(Terminal {
                class: TerminalClass::End,
                lexeme: String::new(),
            })
        } else {
            Self::char_to_terminal(self.source[self.position])
        }
    }

    fn char_to_terminal(ch: char) -> Result<Terminal, &'static str> {
        match ch {
            '+' => Ok(Terminal {
                lexeme: ch.to_string(),
                class: TerminalClass::Plus,
            }),
            '*' => Ok(Terminal {
                lexeme: ch.to_string(),
                class: TerminalClass::Star,
            }),
            '(' => Ok(Terminal {
                lexeme: ch.to_string(),
                class: TerminalClass::LeftParen,
            }),
            ')' => Ok(Terminal {
                lexeme: ch.to_string(),
                class: TerminalClass::RightParen,
            }),
            ch if ch.is_ascii_digit() => Ok(Terminal {
                lexeme: ch.to_string(),
                class: TerminalClass::Number,
            }),
            _ => Err("Unrecognized character found"),
        }
    }
}

struct Parser {
    state_stack: Vec<State>,
    actions: HashMap<(usize, TerminalClass), Action>,
    next_states: HashMap<(usize, NonTerminalClass), usize>,
    rule_component_counts: HashMap<usize, usize>,
    rule_heads: HashMap<usize, NonTerminalClass>,
}

impl Parser {
    fn new() -> Self {
        let initial_state = State {
            symbol: None,
            number: 0,
        };
        let state_stack = vec![initial_state];

        let mut actions = HashMap::new();

        actions.insert((0, TerminalClass::Number), Action::Shift(3));
        actions.insert((0, TerminalClass::LeftParen), Action::Shift(2));

        actions.insert((1, TerminalClass::Plus), Action::Shift(4));
        actions.insert((1, TerminalClass::Star), Action::Shift(5));
        actions.insert((1, TerminalClass::End), Action::Accept);

        actions.insert((2, TerminalClass::Number), Action::Shift(3));
        actions.insert((2, TerminalClass::LeftParen), Action::Shift(2));

        actions.insert((3, TerminalClass::Plus), Action::Reduce(4));
        actions.insert((3, TerminalClass::Star), Action::Reduce(4));
        actions.insert((3, TerminalClass::RightParen), Action::Reduce(4));
        actions.insert((3, TerminalClass::End), Action::Reduce(4));

        actions.insert((4, TerminalClass::Number), Action::Shift(3));
        actions.insert((4, TerminalClass::LeftParen), Action::Shift(2));

        actions.insert((5, TerminalClass::Number), Action::Shift(3));
        actions.insert((5, TerminalClass::LeftParen), Action::Shift(2));

        actions.insert((6, TerminalClass::Plus), Action::Shift(4));
        actions.insert((6, TerminalClass::Star), Action::Shift(5));
        actions.insert((6, TerminalClass::RightParen), Action::Shift(9));

        actions.insert((7, TerminalClass::Plus), Action::Reduce(1));
        actions.insert((7, TerminalClass::Star), Action::Shift(5));
        actions.insert((7, TerminalClass::RightParen), Action::Reduce(1));
        actions.insert((7, TerminalClass::End), Action::Reduce(1));

        actions.insert((8, TerminalClass::Plus), Action::Reduce(2));
        actions.insert((8, TerminalClass::Star), Action::Reduce(2));
        actions.insert((8, TerminalClass::RightParen), Action::Reduce(2));
        actions.insert((8, TerminalClass::End), Action::Reduce(2));

        actions.insert((9, TerminalClass::Plus), Action::Reduce(3));
        actions.insert((9, TerminalClass::Star), Action::Reduce(3));
        actions.insert((9, TerminalClass::RightParen), Action::Reduce(3));
        actions.insert((9, TerminalClass::End), Action::Reduce(3));

        let mut next_states = HashMap::new();

        next_states.insert((0, NonTerminalClass::Expr), 1);
        next_states.insert((2, NonTerminalClass::Expr), 6);
        next_states.insert((4, NonTerminalClass::Expr), 7);
        next_states.insert((5, NonTerminalClass::Expr), 8);

        let mut rule_component_counts = HashMap::new();

        rule_component_counts.insert(1, 3);
        rule_component_counts.insert(2, 3);
        rule_component_counts.insert(3, 3);
        rule_component_counts.insert(4, 1);

        let mut rule_heads = HashMap::new();

        rule_heads.insert(1, NonTerminalClass::Expr);
        rule_heads.insert(2, NonTerminalClass::Expr);
        rule_heads.insert(3, NonTerminalClass::Expr);
        rule_heads.insert(4, NonTerminalClass::Expr);

        Self {
            state_stack,
            actions,
            next_states,
            rule_component_counts,
            rule_heads,
        }
    }

    fn parse(&mut self, mut lexer: Lexer) -> Result<Symbol, &'static str> {
        loop {
            let terminal = lexer.peek()?;
            match self.get_action(terminal.class) {
                Some(Action::Shift(state_number)) => {
                    self.shift(lexer.next().unwrap(), state_number)
                }
                Some(Action::Reduce(rule_number)) => match rule_number {
                    rule_number if rule_number <= 4 => self.reduce_rule(rule_number),
                    _ => unreachable!(),
                },
                Some(Action::Accept) => return Ok(self.get_top_symbol()),
                None => return Err("Syntax error"),
            }
        }
    }

    fn reduce_rule(&mut self, rule_number: usize) {
        let non_terminal_class = self.rule_heads[&rule_number];
        let rule = Rule {
            number: rule_number,
            components: self.get_top_symbols(self.rule_component_counts[&rule_number]),
        };
        let non_terminal = NonTerminal {
            rule,
            class: non_terminal_class,
        };
        let new_state = State {
            symbol: Some(Symbol::NonTerminal(non_terminal)),
            number: self.next(non_terminal_class),
        };
        self.state_stack.push(new_state);
    }

    fn shift(&mut self, terminal: Terminal, next_state_number: usize) {
        let new_state = State {
            symbol: Some(Symbol::Terminal(terminal)),
            number: next_state_number,
        };
        self.state_stack.push(new_state);
    }

    fn get_top_symbol(&mut self) -> Symbol {
        self.state_stack.pop().unwrap().symbol.unwrap()
    }

    fn get_top_symbols(&mut self, n: usize) -> Vec<Symbol> {
        self.state_stack
            .split_off(self.state_stack.len() - n)
            .into_iter()
            .map(|s| s.symbol.unwrap())
            .collect()
    }

    fn get_action(&self, terminal_class: TerminalClass) -> Option<Action> {
        self.actions
            .get(&(self.current_state_number(), terminal_class))
            .copied()
    }

    fn next(&self, non_terminal_class: NonTerminalClass) -> usize {
        self.next_states
            .get(&(self.current_state_number(), non_terminal_class))
            .copied()
            .unwrap()
    }

    fn current_state_number(&self) -> usize {
        self.state_stack.last().unwrap().number
    }
}

struct State {
    symbol: Option<Symbol>,
    number: usize,
}

#[derive(Clone, Copy)]
enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}

enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

struct NonTerminal {
    rule: Rule,
    class: NonTerminalClass,
}

struct Terminal {
    lexeme: String,
    class: TerminalClass,
}

struct Rule {
    components: Vec<Symbol>,
    number: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum NonTerminalClass {
    Expr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TerminalClass {
    Number,
    Plus,
    Star,
    LeftParen,
    RightParen,
    End,
}

impl Symbol {
    fn pretty_print(&self, indent: usize) {
        let indent_str = "    ".repeat(indent);
        match self {
            Symbol::NonTerminal(non_terminal) => {
                println!(
                    "{indent_str}{:?}({}):",
                    non_terminal.class, non_terminal.rule.number
                );
                for symbol in non_terminal.rule.components.iter() {
                    symbol.pretty_print(indent + 1)
                }
            }
            Symbol::Terminal(terminal) => println!("{indent_str}{}", terminal.lexeme),
        }
    }
}

fn main() {
    let lexer = Lexer::new("1+(2+3*4+5)+6");
    let mut parser = Parser::new();
    let root = parser.parse(lexer).unwrap();
    root.pretty_print(0);
}
