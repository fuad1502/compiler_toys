use parser::{lexer::Lexer, parser::Parser};

fn main() {
    let lexer = Lexer::new("1+(2+3*4+5)+6");
    let mut parser = Parser::new();
    let root = parser.parse(lexer).unwrap();
    root.pretty_print(0);
}
