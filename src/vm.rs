use std::usize;

use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub program_counter: usize,
    pub program: Vec<u8>,
    pub remainder: i32,
    pub equality_flag: bool,
    pub heap: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        let vm = VM {
            program: Vec::new(),
            registers: [0; 32],
            program_counter: 0,
            remainder: 0,
            equality_flag: false,
            heap: Vec::new(),
        };
        return vm;
    }

    pub fn new_with_program(program: Vec<u8>) -> VM {
        VM {
            program,
            registers: [0; 32],
            program_counter: 0,
            remainder: 0,
            equality_flag: false,
            heap: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instrunction();
            println!("{:?}", self.registers)
        }
    }

    fn execute_instrunction(&mut self) -> bool {
        if self.program_counter >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.get_next_byte() as usize;
                let value = self.get_next_2_bytes();
                self.registers[register] = value as i32;
            }
            Opcode::ADD => {
                let source_register = self.get_next_byte() as usize;
                let register1 = self.registers[self.get_next_byte() as usize];
                let register2 = self.registers[self.get_next_byte() as usize];
                self.registers[source_register] = register1 + register2
            }
            Opcode::SUB => {
                let source_register = self.get_next_byte() as usize;
                let register1 = self.registers[self.get_next_byte() as usize];
                let register2 = self.registers[self.get_next_byte() as usize];
                self.registers[source_register] = register1 - register2;
            }
            Opcode::MUL => {
                let source_register = self.get_next_byte() as usize;
                let register1 = self.registers[self.get_next_byte() as usize];
                let register2 = self.registers[self.get_next_byte() as usize];
                self.registers[source_register] = register1 * register2;
            }
            Opcode::DIV => {
                let source_register = self.get_next_byte() as usize;
                let register1 = self.registers[self.get_next_byte() as usize];
                let register2 = self.registers[self.get_next_byte() as usize];
                self.registers[source_register] = register1 / register2;
                self.remainder = register1 % register2
            }
            Opcode::JMP => {
                let target = self.get_next_byte() as usize;
                self.program_counter = target;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.get_next_byte() as usize];
                let register2 = self.registers[self.get_next_byte() as usize];
                self.equality_flag = register1 == register2;
                self.get_next_byte();
            }
            Opcode::JEQ => {
                let target = self.get_next_byte() as usize;
                if self.equality_flag {
                    self.program_counter = target;
                }
            }
            Opcode::JNEQ => {
                let target = self.get_next_byte() as usize;
                if !self.equality_flag {
                    self.program_counter = target;
                }
            }
            Opcode::ALLOC => {
                let size = self.registers[self.get_next_byte() as usize];
                let new_end_heap = self.heap.len() as i32 + size;
                self.heap.resize(new_end_heap as usize, 0);
            }
            Opcode::INC => {
                let register = self.get_next_byte() as usize;
                self.registers[register] += 1;
            }
            Opcode::DEC => {
                let register = self.get_next_byte() as usize;
                self.registers[register] -= 1;
            }
            Opcode::ZERO => {
                return false;
            }
            Opcode::ILLEGAL => {
                panic!("Unrecognized instrunction");
            }
        }
        return false;
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.program_counter]);
        self.program_counter += 1;
        return opcode;
    }

    fn get_next_byte(&mut self) -> u8 {
        let byte = self.program[self.program_counter];
        self.program_counter += 1;
        return byte;
    }

    // NOTE we're doing big endian
    fn get_next_2_bytes(&mut self) -> u16 {
        let high_part = self.get_next_byte() as u16;
        let low_part = self.get_next_byte() as u16;
        return (high_part << 8) | low_part;
    }

    pub fn append_to_program(&mut self, prg: Vec<u8>) {
        for p in prg {
            self.program.push(p);
        }
    }
}

// NOTE i love this
#[allow(unused)]
impl std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "--------Value of registers---------\n");
        for (index, value) in self.registers.iter().enumerate() {
            write!(f, "Value of register {}: {}\n", index, value);
        }
        write!(f, "------------------------------------\n");
        write!(f, "Value of program counter: {}\n", self.program_counter);
        write!(f, "------------------------------------\n");
        write!(f, "Equality flag: {}\n", self.equality_flag);
        write!(f, "Program: ");
        for value in &self.program {
            write!(f, "{} ", value);
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers, [0; 32])
    }

    #[test]
    fn test_load_inst() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 10, 1, 1, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[10], 257);
    }

    #[test]
    fn test_add_inst() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 1, 0, 1, 1, 2, 0, 1, 2, 3, 1, 2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 2);
    }

    #[test]
    fn test_sub_inst() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 1, 0, 2, 1, 2, 0, 1, 3, 3, 1, 2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 1);
    }

    #[test]
    fn test_mult_inst() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 1, 0, 10, 1, 2, 0, 2, 4, 3, 1, 2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 20);
    }

    #[test]
    fn test_div_inst() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 1, 0, 10, 1, 2, 0, 3, 5, 3, 1, 2];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[3], 3);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_jump_inst() {
        let mut test_vm = VM::new();
        let test_bytes = vec![6, 0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.execute_instrunction();
        assert_eq!(test_vm.program_counter, 0);
    }

    #[test]
    fn test_eq_inst() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 69;
        test_vm.registers[1] = 69;
        let test_bytes = vec![7, 0, 1, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.equality_flag, true);
    }

    #[test]
    fn test_jeq_inst() {
        let mut test_vm = VM::new();
        test_vm.equality_flag = true;
        let test_bytes = vec![8, 1, 0];
        test_vm.program = test_bytes;
        test_vm.execute_instrunction();
        assert_eq!(test_vm.program_counter, 1);
    }

    #[test]
    fn test_alloc_inst() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 32;
        test_vm.program = [9, 0, 0].to_vec();
        test_vm.run();
        assert_eq!(test_vm.heap.len(), 32);
    }

    #[test]
    fn test_inc_inst() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 68;
        test_vm.program = [10, 0, 0].to_vec();
        test_vm.run();
        assert_eq!(test_vm.registers[0], 69);
    }
    #[test]
    fn test_dec_inst() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 70;
        test_vm.program = [11, 0, 0].to_vec();
        test_vm.run();
        assert_eq!(test_vm.registers[0], 69);
    }
}
