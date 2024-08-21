pub mod lexer;
pub mod parser;

use lexer::Lexer;
use parser::Parser;

use crate::vm::VM;

pub struct Assembler {}

impl Assembler {
    pub fn run(program: String) {
        let mut lexer = Lexer::new(program);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(errs) => panic!("Lexing error: {:?}", errs),
        };
        let mut parser = Parser::new(tokens);
        let instructions = match parser.parse() {
            Ok(insts) => insts,
            Err(errors) => {
                panic!("Parser errors: {:?} ", errors);
            }
        };

        let mut bytes: Vec<u8> = vec![];
        for mut instruction in instructions {
            for byte in instruction.to_bytes() {
                bytes.push(byte);
            }
        }
        let mut vm = VM::new_with_program(bytes);
        vm.run();
        println!("{}", vm);
    }

    pub fn parse_to_bytes(program: String) -> Vec<u8> {
        let mut lexer = Lexer::new(program);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(errs) => panic!("Lexing error: {:?}", errs),
        };
        let mut parser = Parser::new(tokens);
        let instructions = match parser.parse() {
            Ok(insts) => insts,
            Err(errors) => {
                panic!("Parser errors: {:?} ", errors);
            }
        };

        let mut bytes: Vec<u8> = vec![];
        for mut instruction in instructions {
            for byte in instruction.to_bytes() {
                bytes.push(byte);
            }
        }
        bytes
    }
}
