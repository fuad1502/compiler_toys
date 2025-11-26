mod evaluator;
#[allow(unused)]
mod lexer;
mod parser;
#[allow(unused)]
mod symbol;

use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

pub fn calculate(expression: &str) -> Result<f32, String> {
    let mut lexer = Lexer::from_source_str(expression);
    let mut parser = Parser::new();
    let root = parser.parse(&mut lexer)?;
    root.pretty_print(&lexer, 0);
    let mut evaluator = Evaluator::new(&lexer);
    let result = evaluator.visit(&root);
    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::calculate;

    #[test]
    fn main() {
        assert_eq!(calculate("1 + (2 + 3 * 4 - 5) + 6 / 2").unwrap(), 13f32)
    }
}
