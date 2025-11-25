mod code_gen;
mod lexer_generator;
mod regex_parser;

pub use code_gen::generate;

pub struct TokenSpec {
    id: usize,
    name: String,
    pattern: String,
}

impl TokenSpec {
    pub fn new(id: usize, name: String, pattern: String) -> Self {
        Self { id, name, pattern }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
