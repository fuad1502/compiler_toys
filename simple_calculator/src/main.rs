use std::process::ExitCode;

use simple_calculator::{lexer::Lexer, parser::Parser};

fn main() -> ExitCode {
    let mut lexer = Lexer::from_source_str("1 + (2 + 3 * 4 + 5) + 6");
    let mut parser = Parser::new();
    let root = match parser.parse(&mut lexer) {
        Ok(node) => node,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    root.pretty_print(&lexer, 0);
    ExitCode::SUCCESS
}
