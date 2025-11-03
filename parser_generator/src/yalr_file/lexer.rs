use std::{fs::File, io::Read, path::Path};

#[derive(Clone)]
pub struct Terminal {
    start_pos: usize,
    end_pos: usize,
    class: TerminalClass,
}

impl Terminal {
    pub fn class(&self) -> TerminalClass {
        self.class
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalClass {
    TokensSection,
    RulesSection,
    PrioritiesSection,
    Identifier,
    Assignment,
    Or,
    Semicolon,
    LeftParen,
    RightParen,
    Number,
    End,
}

impl Terminal {
    fn left_paren(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TerminalClass::LeftParen,
        }
    }

    fn right_paren(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TerminalClass::RightParen,
        }
    }

    fn assignment(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TerminalClass::Assignment,
        }
    }

    fn or(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TerminalClass::Or,
        }
    }

    fn semicolon(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TerminalClass::Semicolon,
        }
    }

    fn number(start_pos: usize, end_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos,
            class: TerminalClass::Number,
        }
    }

    fn tokens_section(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + "%TOKENS".len(),
            class: TerminalClass::TokensSection,
        }
    }

    fn rules_section(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + "%RULES".len(),
            class: TerminalClass::RulesSection,
        }
    }

    fn priorities_section(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + "%PRIORITIES".len(),
            class: TerminalClass::PrioritiesSection,
        }
    }

    fn id(start_pos: usize, id: &str) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + id.len(),
            class: TerminalClass::Identifier,
        }
    }

    fn end(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos,
            class: TerminalClass::End,
        }
    }
}

pub struct Lexer {
    chars: Vec<char>,
    start_pos: usize,
    current_pos: usize,
    current_terminal: Option<Terminal>,
}

impl Lexer {
    pub fn new(yalr_file: &Path) -> Result<Self, Error> {
        let mut file = File::open(yalr_file)?;
        let mut source = String::new();
        let _ = file.read_to_string(&mut source)?;
        let chars = source.chars().collect();
        Ok(Self {
            chars,
            start_pos: 0,
            current_pos: 0,
            current_terminal: None,
        })
    }

    pub fn next(&mut self) -> Result<Terminal, Error> {
        let terminal = self.peek()?.clone();
        self.move_start_pos();
        self.current_terminal = None;
        Ok(terminal)
    }

    pub fn peek(&mut self) -> Result<&Terminal, Error> {
        if self.current_terminal.is_none() {
            self.current_terminal = Some(self.get()?);
        }
        Ok(self.current_terminal.as_ref().unwrap())
    }

    fn get(&mut self) -> Result<Terminal, Error> {
        loop {
            match self.read_char() {
                Some(c) if c.is_whitespace() => {
                    self.move_start_pos();
                    continue;
                }
                Some('(') => {
                    return Ok(Terminal::left_paren(self.start_pos));
                }
                Some(')') => {
                    return Ok(Terminal::right_paren(self.start_pos));
                }
                Some('=') => {
                    return Ok(Terminal::assignment(self.start_pos));
                }
                Some('|') => {
                    return Ok(Terminal::or(self.start_pos));
                }
                Some(';') => {
                    return Ok(Terminal::semicolon(self.start_pos));
                }
                Some(c) if c.is_ascii_digit() => return self.read_number(),
                Some(c) if c == '%' => return self.read_section_id(),
                Some(c) if c.is_ascii_alphabetic() => return self.read_id(c),
                Some(c) => return Err(Error::UnexpectedChar(c, "")),
                None => return Ok(Terminal::end(self.start_pos)),
            };
        }
    }

    fn read_number(&mut self) -> Result<Terminal, Error> {
        loop {
            match self.read_char_if(|c| !Self::is_terminator(c)) {
                Some(c) if c.is_ascii_digit() => continue,
                Some(c) => return Err(Error::UnexpectedChar(c, "digit")),
                None => break,
            }
        }
        Ok(Terminal::number(self.start_pos, self.current_pos))
    }

    fn read_section_id(&mut self) -> Result<Terminal, Error> {
        let mut id = vec![];
        loop {
            match self.read_char_if(|c| !Self::is_terminator(c)) {
                Some(c) if c.is_ascii_uppercase() => id.push(c as u8),
                Some(c) => return Err(Error::UnexpectedChar(c, "uppercase character")),
                None => break,
            }
        }
        let id = String::from_utf8(id).unwrap();
        match &id[..] {
            "TOKENS" => return Ok(Terminal::tokens_section(self.start_pos)),
            "RULES" => return Ok(Terminal::rules_section(self.start_pos)),
            "PRIORITIES" => return Ok(Terminal::priorities_section(self.start_pos)),
            _ => return Err(Error::UnrecognizedSectionId(id)),
        }
    }

    fn read_id(&mut self, first_char: char) -> Result<Terminal, Error> {
        let mut id = vec![first_char as u8];
        loop {
            match self.read_char_if(|c| !Self::is_terminator(c)) {
                Some(c) if c.is_ascii_alphanumeric() => id.push(c as u8),
                Some(c) => return Err(Error::UnexpectedChar(c, "alphanumeric character")),
                None => break,
            }
        }
        let id = String::from_utf8(id).unwrap();
        if Self::is_valid_id(&id) {
            return Ok(Terminal::id(self.start_pos, &id));
        }
        Err(Error::InvalidId(id))
    }

    fn is_valid_id(id: &str) -> bool {
        id.chars().next().is_some_and(|c| c.is_ascii_uppercase())
    }

    fn is_terminator(ch: char) -> bool {
        ch.is_whitespace() || ch == ';' || ch == '(' || ch == ')'
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.current_pos).copied()
    }

    fn read_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        if ch.is_some() {
            self.current_pos += 1;
        }
        ch
    }

    fn read_char_if(&mut self, predicate: fn(char) -> bool) -> Option<char> {
        let ch = self.peek_char();
        if ch.is_some() && predicate(ch.unwrap()) {
            self.current_pos += 1;
            return ch;
        }
        None
    }

    fn move_start_pos(&mut self) {
        self.start_pos = self.current_pos;
    }
}

pub enum Error {
    Io(std::io::Error),
    UnexpectedChar(char, &'static str),
    UnrecognizedSectionId(String),
    InvalidId(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "IO error: {error}"),
            Error::UnexpectedChar(found, expected) => write!(
                f,
                "Found unexpected character: {found}, expected: {expected}"
            ),
            Error::UnrecognizedSectionId(id) => write!(f, "Unrecognized section id: {id}"),
            Error::InvalidId(id) => write!(f, "Invalid formatted id: {id}"),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::yalr_file::lexer::{Lexer, TerminalClass};

    #[test]
    fn test() {
        let mut simple_calculator_yalr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        simple_calculator_yalr.push("test/fixtures/simple_calculator.yalr");
        let mut lexer = Lexer::new(&simple_calculator_yalr).unwrap();
        let mut terminals = vec![];
        loop {
            let terminal = lexer.next().unwrap();
            if terminal.class() == TerminalClass::End {
                break;
            }
            terminals.push(terminal)
        }
        assert_eq!(terminals.len(), 34)
    }
}
