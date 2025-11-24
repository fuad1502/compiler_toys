use std::path::PathBuf;

use lexer_generator::{TokenSpec, generate_lexer};

fn main() {
    let number = TokenSpec::new(0, "\\d\\d*".to_string());
    let output_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    generate_lexer(vec![number], &output_directory).unwrap()
}
