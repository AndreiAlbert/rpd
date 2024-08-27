pub mod lexer;
pub mod parser;
pub mod symbol;

use core::panicking::panic;

use lexer::{token::Token, Lexer};
use parser::{AssemblyInstruction, Parser};
use symbol::{
    symbol::{Symbol, SymbolType},
    symbol_table::SymbolTable,
};

use crate::vm::VM;

pub enum AssemblerSection {
    Data { starting_offset: Option<u32> },
    Code { starting_offset: Option<u32> },
    Unkown,
}

pub struct Assembler {
    source: String,

    symbol_table: SymbolTable,
    pub read_only_secion: Vec<u8>,

    pub bytecode: Vec<u8>,

    read_only_offset: u32,

    sections: Vec<AssemblerSection>,

    current_section: Option<AssemblerSection>,

    current_inst: u32,
    // errros: Vec<String>,
}

impl Assembler {
    pub fn new(source: String) -> Assembler {
        Assembler {
            source,
            symbol_table: SymbolTable::new(),
            read_only_secion: vec![],
            bytecode: vec![],
            read_only_offset: 0,
            sections: vec![],
            current_section: None,
            current_inst: 0,
        }
    }

    fn get_tokens(&mut self) -> Vec<Token> {
        let mut lexer = Lexer::new(&self.source);
        match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(errors) => panic!("{:?}", errors),
        }
    }

    fn get_instructions(&mut self, tokens: Vec<Token>) -> Vec<AssemblyInstruction> {
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(instructions) => instructions,
            Err(errros) => panic!("{:?}", errros),
        }
    }

    pub fn assemble(&mut self) {
        let tokens = self.get_tokens();
        let instructions = self.get_instructions(tokens);
        self.first_phase(&instructions);
    }

    pub fn first_phase(&mut self, insts: &Vec<AssemblyInstruction>) {
        for i in insts {}
    }
}
