use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser, visitor::Visitor};

mod evaluator;
mod lexer;
mod parser;
mod symbol;
mod visitor;

pub fn calculate(expression: &str) -> Result<f32, String> {
    let mut lexer = Lexer::from_source_str(expression);
    let mut parser = Parser::new();
    let root = parser.parse(&mut lexer)?;
    let mut evaluator = Evaluator::new(&lexer);
    let result = evaluator.visit(&root);
    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::calculate;

    #[test]
    fn main() {
        assert_eq!(calculate("1 + (2 + 3 * 4 + 5) + 6").unwrap(), 26f32)
    }
}
