use std::path::Path;

use crate::{code_gen::CodeGen, yalr_file::YalrFile};

pub fn run(yalr_file_path: &Path, output_directory: &Path) -> Result<(), String> {
    let yalr_file = YalrFile::new(yalr_file_path)?;
    let code_gen = CodeGen::new(yalr_file);
    code_gen.generate(output_directory)
}
