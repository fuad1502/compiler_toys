//! A simple example of how to use [JJIK]().
//!
//! This crate shows how you can utilize JJIK to parse and evaluate mathematical expressions.
//!
//! This crate generates the parser using the following grammar specified in a GG file:
//!
//! ```text
//! %TERMINALS
//! Number("\d\d*")
//! Plus("+")
//! Minus("-")
//! Star("\*")
//! Slash("/")
//! LeftParen("\(")
//! RightParen("\)")
//!
//! %RULES
//! E = E Plus E
//! | E Minus E
//! | E Star E
//! | E Slash E
//! | LeftParen E RightParen
//! | Number;
//!
//! %PRIORITIES
//! E(3)
//! E(4)
//! Star
//! Slash
//!
//! E(1)
//! E(2)
//! Plus
//! Minus
//! ```

mod evaluator;
#[allow(unused)]
mod lexer;
mod parser;
#[allow(unused)]
mod symbol;

use crate::{evaluator::Evaluator, lexer::Lexer, parser::Parser};

/// Calculate the result of a mathematical expression.
///
/// As a side effect, it prints out the concrete syntax tree representation of `expression`.
///
/// # Example
/// ```rust
/// use jjik_simple_calculator::calculate;
///
/// assert_eq!(calculate("(5 + 5) * 2").unwrap(), 20.0);
/// ```
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
