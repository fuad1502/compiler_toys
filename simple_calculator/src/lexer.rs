use std::{collections::HashMap, fs::File, io::Read, path::Path};

use crate::symbol::{Span, Terminal, TerminalClass};

static N: usize = 10;

#[derive(Copy, Clone)]
struct State {
    class: Option<TerminalClass>,
}

pub struct Lexer {
    chars: Vec<u8>,
    start_pos: usize,
    current_pos: usize,
    current_token: Option<Terminal>,
    states: [State; N],
    transition_table: Vec<HashMap<char, usize>>,
    states_stack: Vec<Vec<usize>>,
}

impl Lexer {
    pub fn from_source_str(source: &str) -> Self {
        let chars = source.chars().map(|c| c as u8).collect::<Vec<u8>>();

        let states = [
            State { class: None },
            State { class: None },
            State { class: None },
            State { class: None },
            State { class: None },
            State {
                class: Some(TerminalClass::Plus),
            },
            State {
                class: Some(TerminalClass::Star),
            },
            State {
                class: Some(TerminalClass::LeftParen),
            },
            State {
                class: Some(TerminalClass::RightParen),
            },
            State {
                class: Some(TerminalClass::Number),
            },
        ];

        let initial_states = vec![0, 1, 2, 3, 4];

        let mut state_0_transisions = HashMap::new();
        state_0_transisions.insert('+', 5);
        let mut state_1_transisions = HashMap::new();
        state_1_transisions.insert('*', 6);
        let mut state_2_transisions = HashMap::new();
        state_2_transisions.insert('(', 7);
        let mut state_3_transisions = HashMap::new();
        state_3_transisions.insert(')', 8);
        let mut state_4_transisions = HashMap::new();
        state_4_transisions.insert('0', 9);
        state_4_transisions.insert('1', 9);
        state_4_transisions.insert('2', 9);
        state_4_transisions.insert('3', 9);
        state_4_transisions.insert('4', 9);
        state_4_transisions.insert('5', 9);
        state_4_transisions.insert('6', 9);
        state_4_transisions.insert('7', 9);
        state_4_transisions.insert('8', 9);
        state_4_transisions.insert('9', 9);
        let state_5_transisions = HashMap::new();
        let state_6_transisions = HashMap::new();
        let state_7_transisions = HashMap::new();
        let state_8_transisions = HashMap::new();
        let state_9_transisions = HashMap::new();

        let transition_table = vec![
            state_0_transisions,
            state_1_transisions,
            state_2_transisions,
            state_3_transisions,
            state_4_transisions,
            state_5_transisions,
            state_6_transisions,
            state_7_transisions,
            state_8_transisions,
            state_9_transisions,
        ];

        Self {
            chars,
            start_pos: 0,
            current_pos: 0,
            current_token: None,
            states,
            transition_table,
            states_stack: vec![initial_states],
        }
    }

    pub fn new(source_file: &Path) -> Result<Self, std::io::Error> {
        let mut source_file = File::open(source_file)?;
        let mut source = String::new();
        let _ = source_file.read_to_string(&mut source)?;
        Ok(Self::from_source_str(&source))
    }

    pub fn next_token(&mut self) -> Result<Terminal, String> {
        let token = self.peek_token()?.clone();
        self.move_start_pos();
        self.current_token = None;
        Ok(token)
    }

    pub fn peek_token(&mut self) -> Result<&Terminal, String> {
        if self.current_token.is_none() {
            if self.peek_char().is_none() {
                let end_token = Terminal::new(TerminalClass::End, self.current_span());
                self.current_token = Some(end_token);
            } else {
                self.current_token = Some(self.get()?);
                _ = self.states_stack.split_off(1);
            }
        }
        Ok(self.current_token.as_ref().unwrap())
    }

    pub fn get_lexeme(&self, token: &Terminal) -> &str {
        str::from_utf8(&self.chars[token.span().start_pos()..token.span().end_pos()]).unwrap()
    }

    fn move_start_pos(&mut self) {
        self.start_pos = self.current_pos;
    }

    fn get(&mut self) -> Result<Terminal, String> {
        self.skip_whitespaces();
        loop {
            match self.peek_char() {
                Some(c) => {
                    if self.move_states_on_stack(c) {
                        self.read_char();
                    } else {
                        return self.evaluate_stack();
                    }
                }
                None => return self.evaluate_stack(),
            }
        }
    }

    fn move_states_on_stack(&mut self, input: char) -> bool {
        let mut new_states = vec![];
        for state in self.states_stack.last().unwrap() {
            if let Some(new_state) = self.transition_table[*state].get(&input) {
                new_states.push(*new_state);
            }
        }
        if !new_states.is_empty() {
            self.states_stack.push(new_states);
            return true;
        }
        false
    }

    fn evaluate_stack(&mut self) -> Result<Terminal, String> {
        loop {
            let mut accepting_classes = vec![];
            for state in self.states_stack.last().unwrap() {
                if let Some(class) = self.states[*state].class {
                    accepting_classes.push(class);
                }
            }
            if let Some(prioritized_class) = accepting_classes.iter().copied().min() {
                let span = self.current_span();
                let class = prioritized_class;
                return Ok(Terminal::new(class, span));
            } else if self.states_stack.len() == 1 {
                return Err(format!(
                    "Unexpected character found: {}",
                    self.peek_char()
                        .map(|c| c.to_string())
                        .unwrap_or(String::from("EOF"))
                ));
            } else {
                self.states_stack.pop();
                self.revert_char();
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.current_pos).copied().map(|c| c as char)
    }

    fn read_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        if ch.is_some() {
            self.current_pos += 1;
        }
        ch
    }

    fn revert_char(&mut self) {
        self.current_pos -= 1;
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn current_span(&self) -> Span {
        Span::new(self.start_pos, self.current_pos)
    }
}
