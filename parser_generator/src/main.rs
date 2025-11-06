use core::convert::From;
use std::path::PathBuf;

use parser_generator::{code_gen::CodeGen, yalr_file::YalrFile};

fn main() {
    let mut simple_calculator_yalr = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    simple_calculator_yalr.push("test/fixtures/simple_calculator.yalr");
    let yalr_file = YalrFile::new(&simple_calculator_yalr).unwrap();
    let code_gen = CodeGen::new(yalr_file);
    let output_file = PathBuf::from("example.rs");
    code_gen.generate(&output_file).unwrap();
}
