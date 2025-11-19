use std::path::PathBuf;

fn main() {
    let yalr_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("yalr")
        .join("simple_calculator.yalr");
    let output_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    if let Err(e) = parser_generator::driver::run(&yalr_file_path, &output_directory) {
        eprintln!("{}", e);
        panic!("Failed to compile yalr file!");
    }
    println!("cargo:rerun-if-changed=yalr/simple_calculator.rs");
}
