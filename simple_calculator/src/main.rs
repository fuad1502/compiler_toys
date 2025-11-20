use simple_calculator::{lexer::Lexer, parser::Parser};

fn main() {
    let mut lexer = Lexer::from_source_str("1 + (2 + 3 * 4 + 5) + 6");
    let mut parser = Parser::new();
    let root = parser.parse(&mut lexer).unwrap();
    root.pretty_print(&lexer, 0);
}
