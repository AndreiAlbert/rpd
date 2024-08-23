pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;
use std::fs::{self};

use assembler::Assembler;

fn get_file_content(file_name: &String) -> std::io::Result<String> {
    fs::read_to_string(file_name).map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("Error while reading file {}: {}", file_name, e),
        )
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        let mut repl = repl::REPL::new();
        repl.run();
    } else if args.len() == 2 {
        if args[0] == "-f" {
            let file_content = match get_file_content(&args[1]) {
                Ok(file_content) => file_content,
                Err(err) => panic!("{}", err),
            };
            let mut ass = Assembler::new();
            ass.run(file_content);
        }
    }
}
