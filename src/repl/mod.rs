use crate::vm::VM;
use std;
use std::io::Write;
use std::io::{self};
use std::num::ParseIntError;

#[allow(dead_code)]
pub struct REPL {
    history: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        return REPL {
            history: vec![],
            vm: VM::new(),
        };
    }

    pub fn run(&mut self) {
        println!("Welcome to rpd");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>>");
            if let Err(e) = io::stdout().flush() {
                eprintln!("Failed to flush stdout {}", e)
            }
            if let Err(e) = stdin.read_line(&mut buffer) {
                eprint!("Failed to read line {}", e);
            }
            let buffer = buffer.trim();
            self.history.push(buffer.to_string());
            match buffer {
                ".exit" => {
                    println!("Have a good day ^^");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.history {
                        println!("{}", command);
                    }
                }
                ".debug" => {
                    println!("{}", self.vm);
                }
                _ => {
                    let bytes = self.parse_hex(buffer);
                    match bytes {
                        Ok(bytes) => {
                            self.vm.program = bytes;
                            self.vm.run();
                        }
                        Err(e) => {
                            eprintln!("Could not parse bytes: {}", e)
                        }
                    }
                }
            }
        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_str in split {
            let byte = u8::from_str_radix(&hex_str, 16);
            match byte {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
        }
        return Ok(results);
    }
}
