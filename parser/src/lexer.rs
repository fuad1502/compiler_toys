use super::symbol::{Terminal, TerminalClass};

pub struct Lexer {
    position: usize,
    source: Vec<char>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            position: 0,
            source: source.chars().collect(),
        }
    }

    pub fn next_token(&mut self) -> Result<Terminal, &'static str> {
        let terminal = self.peek_token()?;
        self.position += 1;
        Ok(terminal)
    }

    pub fn peek_token(&self) -> Result<Terminal, &'static str> {
        if self.position > self.source.len() - 1 {
            Ok(Terminal::new(String::new(), TerminalClass::End))
        } else {
            Self::char_to_token(self.source[self.position])
        }
    }

    fn char_to_token(ch: char) -> Result<Terminal, &'static str> {
        match ch {
            '+' => Ok(Terminal::new(ch.to_string(), TerminalClass::Plus)),
            '*' => Ok(Terminal::new(ch.to_string(), TerminalClass::Star)),
            '(' => Ok(Terminal::new(ch.to_string(), TerminalClass::LeftParen)),
            ')' => Ok(Terminal::new(ch.to_string(), TerminalClass::RightParen)),
            ch if ch.is_ascii_digit() => Ok(Terminal::new(ch.to_string(), TerminalClass::Number)),
            _ => Err("Unrecognized character found"),
        }
    }
}
