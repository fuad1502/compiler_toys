use std::{
    io::{self, Write},
    process::ExitCode,
};

use jjik_simple_calculator::calculate;

fn main() -> ExitCode {
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let size = stdin.read_line(&mut line).unwrap();
        if size == 0 {
            break ExitCode::SUCCESS;
        }

        match calculate(&line) {
            Ok(res) => println!("Result = {res}"),
            Err(e) => eprintln!("{e}"),
        };
    }
}
