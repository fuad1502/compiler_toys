use std::path::PathBuf;

fn main() {
    let gg_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("gg")
        .join("simple_calculator.gg");
    let output_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    if let Err(e) = jjik::driver::run(&gg_file_path, &output_directory) {
        eprintln!("{}", e);
        panic!("Failed to compile .gg file!");
    }
    println!("cargo:rerun-if-changed=gg/simple_calculator.gg");
}
