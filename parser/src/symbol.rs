pub enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

pub struct NonTerminal {
    rule: Rule,
    class: NonTerminalClass,
}

pub struct Terminal {
    lexeme: String,
    class: TerminalClass,
}

pub struct Rule {
    components: Vec<Symbol>,
    number: usize,
}

impl Symbol {
    pub fn pretty_print(&self, indent: usize) {
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

impl Terminal {
    pub fn new(lexeme: String, class: TerminalClass) -> Self {
        Self { lexeme, class }
    }

    pub fn class(&self) -> TerminalClass {
        self.class
    }
}

impl NonTerminal {
    pub fn new(rule: Rule, class: NonTerminalClass) -> Self {
        Self { rule, class }
    }
}

impl Rule {
    pub fn new(number: usize, components: Vec<Symbol>) -> Self {
        Self { number, components }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NonTerminalClass {
    Expr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalClass {
    Number,
    Plus,
    Star,
    LeftParen,
    RightParen,
    End,
}
