mod code_gen;
mod lexer_generator;
mod regex_parser;

pub use code_gen::generate_lexer;

pub struct TokenSpec {
    id: usize,
    pattern: String,
}

impl TokenSpec {
    pub fn new(id: usize, pattern: String) -> Self {
        Self { id, pattern }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}
