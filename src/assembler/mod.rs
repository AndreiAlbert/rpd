pub mod lexer;
pub mod parser;
pub mod symbol;

use lexer::{token::Token, Lexer};
use parser::{AssemblyInstruction, Parser};
use symbol::{
    symbol::{Symbol, SymbolType},
    symbol_table::SymbolTable,
};

use crate::vm::VM;

pub struct Assembler {
    symbol_table: SymbolTable,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbol_table: SymbolTable::new(),
        }
    }

    fn extract_labels(&mut self, insts: &Vec<AssemblyInstruction>) {
        let mut counter = 0;
        for i in insts {
            if let Some(label) = &i.label {
                if let Token::LabelDeclaration { value } = label {
                    let symbol = Symbol::new(value.to_string(), counter, SymbolType::Label);
                    self.symbol_table.add_symbol(symbol);
                }
            }
            counter += 4;
        }
    }

    pub fn run(&mut self, program: String) {
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
        self.extract_labels(&instructions);
        let mut bytes = vec![];
        for mut i in instructions {
            let inst_to_bytes = i.to_bytes(&self.symbol_table);
            if let Some(inst_to_bytes) = inst_to_bytes {
                for b in inst_to_bytes {
                    bytes.push(b);
                }
            }
        }
        println!("{:?}", bytes);
        let mut vm = VM::new_with_program(bytes);
        vm.run();
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

        let bytes: Vec<u8> = vec![];
        for instruction in instructions {
            println!("{:?}", instruction);
        }
        bytes
    }
}
