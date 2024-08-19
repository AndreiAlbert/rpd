use crate::instruction::Opcode;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_number: u8 },
    IntegerOp { value: i32 },
}
