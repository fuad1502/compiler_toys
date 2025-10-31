use core::convert::From;
use std::path::PathBuf;

use parser_generator::{code_gen::CodeGen, yalr_parser::YalrFile};

fn main() {
    let yalr_file = YalrFile::simple_calculator();
    let code_gen = CodeGen::new(yalr_file);
    let output_file = PathBuf::from("example.rs");
    code_gen.generate(&output_file).unwrap();
}
