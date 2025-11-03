use core::fmt::Formatter;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    rc::Rc,
};

use crate::{
    NonTerminal, Rule, Symbol, Terminal,
    parse_table_gen::{Action, ParseTableGen},
    yalr_file::YalrFile,
};

pub struct CodeGen {
    parse_table_gen: ParseTableGen,
    terminals: Vec<(Terminal, String)>,
    non_terminals: Vec<(NonTerminal, String)>,
    rules: Vec<Rc<Rule>>,
}

impl CodeGen {
    pub fn new(yalr_file: YalrFile) -> Self {
        let parse_table_gen = ParseTableGen::new(&yalr_file);
        Self {
            parse_table_gen,
            terminals: yalr_file.terminals,
            non_terminals: yalr_file.non_terminals,
            rules: yalr_file.rules,
        }
    }

    pub fn generate(&self, parser_file: &Path) -> Result<(), String> {
        let mut parser_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(parser_file)
            .map_err(|e| e.to_string())?;
        self.write_structs(&mut parser_file)
            .map_err(|e| e.to_string())
    }

    fn write_structs(&self, parser_file: &mut File) -> std::io::Result<()> {
        self.write_parser_struct(parser_file)?;
        self.write_parser_impl(parser_file)?;
        self.write_other_structs(parser_file)?;
        self.write_non_terminal_class_enum(parser_file)?;
        self.write_terminal_class_enum(parser_file)
    }

    fn write_parser_impl(&self, parser_file: &mut File) -> std::io::Result<()> {
        writeln!(parser_file, "impl Parser {{")?;
        self.write_parser_impl_new(parser_file)?;
        self.write_parser_impl_others(parser_file)?;
        writeln!(parser_file, "}}")
    }

    fn write_parser_impl_new(&self, parser_file: &mut File) -> std::io::Result<()> {
        let mut tabs = Tabs::default();
        tabs.indent();
        tabs.indent();
        self.write_parser_impl_new_prologue(parser_file)?;
        self.write_actions_field(parser_file, &mut tabs)?;
        self.write_next_states_field(parser_file, &mut tabs)?;
        self.write_rule_component_counts_field(parser_file, &mut tabs)?;
        self.write_rule_heads_field(parser_file, &mut tabs)?;
        self.write_parser_impl_new_epilogue(parser_file)
    }

    fn write_parser_struct(&self, parser_file: &mut File) -> std::io::Result<()> {
        let number_of_terminals = self.terminals.len();
        let number_of_non_terminals = self.non_terminals.len();
        let number_of_states = self.parse_table_gen.states.len();
        let number_of_rules = self.rules.len();
        write!(
            parser_file,
            r#"struct Parser {{
    state_stack: Vec<State>,
    actions: [[Action; {number_of_terminals}]; {number_of_states}],
    next_states: [[Option<usize>; {number_of_non_terminals}]; {number_of_states}],
    rule_component_counts: [usize; {number_of_rules}],
    rule_heads: [NonTerminalClass; {number_of_rules}],
}}

"#
        )
    }

    fn write_parser_impl_new_prologue(&self, parser_file: &mut File) -> std::io::Result<()> {
        write!(
            parser_file,
            r#"    fn new() -> Self {{
        let initial_state = State {{
            symbol: None,
            number: 0,
        }};
        let state_stack = vec![initial_state];
"#
        )
    }

    fn write_parser_impl_new_epilogue(&self, parser_file: &mut File) -> std::io::Result<()> {
        write!(
            parser_file,
            r#"
        Self {{
            state_stack,
            actions,
            next_states,
            rule_component_counts,
            rule_heads,
        }}
    }}
"#
        )
    }

    fn write_actions_field(&self, parser_file: &mut File, tabs: &mut Tabs) -> std::io::Result<()> {
        writeln!(parser_file, "{tabs}let actions = [")?;
        tabs.indent();
        for state in &self.parse_table_gen.states {
            writeln!(parser_file, "{tabs}[")?;
            tabs.indent();
            for (terminal, _) in &self.terminals {
                let action = &self.parse_table_gen.action_table[state][terminal];
                writeln!(parser_file, "{tabs}Action::{},", self.action_string(action))?;
            }
            tabs.deindent();
            writeln!(parser_file, "{tabs}],")?;
        }
        tabs.deindent();
        writeln!(parser_file, "{tabs}];")
    }

    fn write_next_states_field(
        &self,
        parser_file: &mut File,
        tabs: &mut Tabs,
    ) -> std::io::Result<()> {
        writeln!(parser_file, "{tabs}let next_states = [")?;
        tabs.indent();
        for state in &self.parse_table_gen.states {
            writeln!(parser_file, "{tabs}[")?;
            tabs.indent();
            for (non_terminal, _) in &self.non_terminals {
                if self.parse_table_gen.goto_table.contains_key(state)
                    && self.parse_table_gen.goto_table[state]
                        .contains_key(&Symbol::NonTerminal(*non_terminal))
                {
                    let next_state = &self.parse_table_gen.goto_table[state]
                        [&Symbol::NonTerminal(*non_terminal)];
                    writeln!(
                        parser_file,
                        "{tabs}Some({}),",
                        self.parse_table_gen.get_state_index(next_state)
                    )?;
                } else {
                    writeln!(parser_file, "{tabs}None, ")?;
                }
            }
            tabs.deindent();
            writeln!(parser_file, "{tabs}],")?;
        }
        tabs.deindent();
        writeln!(parser_file, "{tabs}];")
    }

    fn write_rule_component_counts_field(
        &self,
        parser_file: &mut File,
        tabs: &mut Tabs,
    ) -> std::io::Result<()> {
        write!(parser_file, "{tabs}let rule_component_counts = [",)?;
        for rule in &self.rules {
            write!(parser_file, "{}, ", rule.num_of_components())?;
        }
        writeln!(parser_file, "];")
    }

    fn write_rule_heads_field(
        &self,
        parser_file: &mut File,
        tabs: &mut Tabs,
    ) -> std::io::Result<()> {
        write!(parser_file, "{tabs}let rule_heads = [")?;
        for rule in &self.rules {
            let head_name = &self
                .non_terminals
                .iter()
                .find(|(non_terminal, _)| non_terminal == rule.head())
                .unwrap()
                .1;
            write!(parser_file, "NonTerminalClass::{head_name}, ")?;
        }
        writeln!(parser_file, "];")
    }

    fn write_non_terminal_class_enum(&self, parser_file: &mut File) -> Result<(), std::io::Error> {
        writeln!(
            parser_file,
            "#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\n enum NonTerminalClass {{"
        )?;
        for (_, name) in &self.non_terminals {
            let tabs = Tabs::new(1);
            writeln!(parser_file, "{tabs}{name},")?;
        }
        writeln!(parser_file, "}}")?;
        writeln!(parser_file)
    }

    fn write_terminal_class_enum(&self, parser_file: &mut File) -> Result<(), std::io::Error> {
        writeln!(
            parser_file,
            "#[derive(Clone, Copy, PartialEq, Eq, Hash)]\n enum TerminalClass {{"
        )?;
        for (_, name) in &self.terminals {
            let tabs = Tabs::new(1);
            writeln!(parser_file, "{tabs}{name},")?;
        }
        writeln!(parser_file, "}}")?;
        writeln!(parser_file)
    }

    fn write_parser_impl_others(&self, parser_file: &mut File) -> Result<(), std::io::Error> {
        let number_of_rules = self.rules.len();
        write!(
            parser_file,
            r#"
    fn parse(&mut self, mut lexer: Lexer) -> Result<Symbol, &'static str> {{
        loop {{
            let terminal = lexer.peek()?;
            match self.get_action(terminal.class) {{
                Action::Shift(state_number) => {{
                    self.shift(lexer.next().unwrap(), state_number)
                }}
                Action::Reduce(rule_number) => match rule_number {{
                    rule_number if rule_number <= {number_of_rules} => self.reduce_rule(rule_number),
                    _ => unreachable!(),
                }},
                Action::Accept => return Ok(self.get_top_symbol()),
                Action::Error => return Err("Syntax error"),
            }}
        }}
    }}

    fn reduce_rule(&mut self, rule_number: usize) {{
        let non_terminal_class = self.rule_heads[rule_number];
        let rule = Rule {{
            number: rule_number,
            components: self.get_top_symbols(self.rule_component_counts[rule_number]),
        }};
        let non_terminal = NonTerminal {{
            rule,
            class: non_terminal_class,
        }};
        let new_state = State {{
            symbol: Some(Symbol::NonTerminal(non_terminal)),
            number: self.next(non_terminal_class),
        }};
        self.state_stack.push(new_state);
    }}

    fn shift(&mut self, terminal: Terminal, next_state_number: usize) {{
        let new_state = State {{
            symbol: Some(Symbol::Terminal(terminal)),
            number: next_state_number,
        }};
        self.state_stack.push(new_state);
    }}

    fn get_top_symbol(&mut self) -> Symbol {{
        self.state_stack.pop().unwrap().symbol.unwrap()
    }}

    fn get_top_symbols(&mut self, n: usize) -> Vec<Symbol> {{
        self.state_stack
            .split_off(self.state_stack.len() - n)
            .into_iter()
            .map(|s| s.symbol.unwrap())
            .collect()
    }}

    fn get_action(&self, terminal_class: TerminalClass) -> Action {{
        self.actions[self.current_state_number()][terminal_class as usize]
    }}

    fn next(&self, non_terminal_class: NonTerminalClass) -> usize {{
        self.next_states[self.current_state_number()][non_terminal_class as usize].unwrap()
    }}

    fn current_state_number(&self) -> usize {{
        self.state_stack.last().unwrap().number
    }}
"#
        )
    }

    fn write_other_structs(&self, parser_file: &mut File) -> Result<(), std::io::Error> {
        write!(
            parser_file,
            r#"
struct State {{
    symbol: Option<Symbol>,
    number: usize,
}}

#[derive(Clone, Copy)]
enum Action {{
    Shift(usize),
    Reduce(usize),
    Accept,
    Error,
}}

enum Symbol {{
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}}

struct NonTerminal {{
    rule: Rule,
    class: NonTerminalClass,
}}

struct Terminal {{
    lexeme: String,
    class: TerminalClass,
}}

struct Rule {{
    components: Vec<Symbol>,
    number: usize,
}}

"#
        )
    }

    fn action_string(&self, action: &Action) -> String {
        match action {
            Action::Shift(state) => {
                format!("Shift({})", self.parse_table_gen.get_state_index(state))
            }
            Action::Reduce(rule) => {
                format!("Reduce({})", self.parse_table_gen.get_rule_index(rule))
            }
            Action::Accept => "Accept".to_string(),
            Action::Error => "Error".to_string(),
        }
    }
}

#[derive(Default)]
struct Tabs {
    indent: usize,
    tab: String,
}

impl Tabs {
    const TAB: &'static str = "    ";

    pub fn new(indent: usize) -> Self {
        Self {
            indent,
            tab: Self::TAB.repeat(indent),
        }
    }

    pub fn indent(&mut self) {
        self.indent += 1;
        self.tab = Self::TAB.repeat(self.indent);
    }

    pub fn deindent(&mut self) {
        self.indent -= 1;
        self.tab = Self::TAB.repeat(self.indent);
    }
}

impl std::fmt::Display for Tabs {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.tab)
    }
}
