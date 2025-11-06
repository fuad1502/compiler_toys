use std::{fs::File, io::Read, path::Path};

#[derive(Clone)]
pub struct Token {
    start_pos: usize,
    end_pos: usize,
    class: TokenClass,
}

impl Token {
    pub fn class(&self) -> TokenClass {
        self.class
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenClass {
    TerminalsSection,
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

impl Token {
    fn left_paren(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TokenClass::LeftParen,
        }
    }

    fn right_paren(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TokenClass::RightParen,
        }
    }

    fn assignment(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TokenClass::Assignment,
        }
    }

    fn or(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TokenClass::Or,
        }
    }

    fn semicolon(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + 1,
            class: TokenClass::Semicolon,
        }
    }

    fn number(start_pos: usize, end_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos,
            class: TokenClass::Number,
        }
    }

    fn terminals_section(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + "%TERMINALS".len(),
            class: TokenClass::TerminalsSection,
        }
    }

    fn rules_section(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + "%RULES".len(),
            class: TokenClass::RulesSection,
        }
    }

    fn priorities_section(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + "%PRIORITIES".len(),
            class: TokenClass::PrioritiesSection,
        }
    }

    fn id(start_pos: usize, id: &str) -> Self {
        Self {
            start_pos,
            end_pos: start_pos + id.len(),
            class: TokenClass::Identifier,
        }
    }

    fn end(start_pos: usize) -> Self {
        Self {
            start_pos,
            end_pos: start_pos,
            class: TokenClass::End,
        }
    }
}

pub struct Lexer {
    chars: Vec<u8>,
    start_pos: usize,
    current_pos: usize,
    current_token: Option<Token>,
}

impl Lexer {
    pub fn new(yalr_file: &Path) -> Result<Self, std::io::Error> {
        let mut file = File::open(yalr_file)?;
        let mut source = String::new();
        let _ = file.read_to_string(&mut source)?;
        let chars = source.chars().map(|c| c as u8).collect();
        Ok(Self {
            chars,
            start_pos: 0,
            current_pos: 0,
            current_token: None,
        })
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        let token = self.peek()?.clone();
        self.move_start_pos();
        self.current_token = None;
        Ok(token)
    }

    pub fn peek(&mut self) -> Result<&Token, Error> {
        if self.current_token.is_none() {
            self.current_token = Some(self.get()?);
        }
        Ok(self.current_token.as_ref().unwrap())
    }

    pub fn get_lexeme(&self, token: &Token) -> &str {
        str::from_utf8(&self.chars[token.start_pos..token.end_pos]).unwrap()
    }

    fn get(&mut self) -> Result<Token, Error> {
        loop {
            match self.read_char() {
                Some(c) if c.is_whitespace() => {
                    self.move_start_pos();
                    continue;
                }
                Some('(') => {
                    return Ok(Token::left_paren(self.start_pos));
                }
                Some(')') => {
                    return Ok(Token::right_paren(self.start_pos));
                }
                Some('=') => {
                    return Ok(Token::assignment(self.start_pos));
                }
                Some('|') => {
                    return Ok(Token::or(self.start_pos));
                }
                Some(';') => {
                    return Ok(Token::semicolon(self.start_pos));
                }
                Some(c) if c.is_ascii_digit() => return self.read_number(),
                Some(c) if c == '%' => return self.read_section_id(),
                Some(c) if c.is_ascii_alphabetic() => return self.read_id(c),
                Some(c) => return Err(Error::UnexpectedChar(c, "")),
                None => return Ok(Token::end(self.start_pos)),
            };
        }
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        loop {
            match self.read_char_if(|c| !Self::is_terminator(c)) {
                Some(c) if c.is_ascii_digit() => continue,
                Some(c) => return Err(Error::UnexpectedChar(c, "digit")),
                None => break,
            }
        }
        Ok(Token::number(self.start_pos, self.current_pos))
    }

    fn read_section_id(&mut self) -> Result<Token, Error> {
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
            "TERMINALS" => return Ok(Token::terminals_section(self.start_pos)),
            "RULES" => return Ok(Token::rules_section(self.start_pos)),
            "PRIORITIES" => return Ok(Token::priorities_section(self.start_pos)),
            _ => return Err(Error::UnrecognizedSectionId(id)),
        }
    }

    fn read_id(&mut self, first_char: char) -> Result<Token, Error> {
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
            return Ok(Token::id(self.start_pos, &id));
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
        self.chars.get(self.current_pos).copied().map(|c| c as char)
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
            return ch.map(|c| c as char);
        }
        None
    }

    fn move_start_pos(&mut self) {
        self.start_pos = self.current_pos;
    }
}

impl std::fmt::Display for TokenClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenClass::TerminalsSection => write!(f, "%TERMINALS"),
            TokenClass::RulesSection => write!(f, "%RULES"),
            TokenClass::PrioritiesSection => write!(f, "%PRIORITIES"),
            TokenClass::Identifier => write!(f, "identifier"),
            TokenClass::Assignment => write!(f, "'='"),
            TokenClass::Or => write!(f, "'|'"),
            TokenClass::Semicolon => write!(f, "';'"),
            TokenClass::LeftParen => write!(f, "'('"),
            TokenClass::RightParen => write!(f, "')'"),
            TokenClass::Number => write!(f, "number"),
            TokenClass::End => write!(f, "EOF"),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class)
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

    use crate::yalr_file::lexer::{Lexer, TokenClass};

    #[test]
    fn main() {
        let mut simple_calculator_yalr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        simple_calculator_yalr.push("test/fixtures/simple_calculator.yalr");
        let mut lexer = Lexer::new(&simple_calculator_yalr).unwrap();
        let mut tokens = vec![];
        loop {
            let token = lexer.next().unwrap();
            if token.class() == TokenClass::End {
                break;
            }
            tokens.push(token)
        }
        assert_eq!(tokens.len(), 34)
    }
}
