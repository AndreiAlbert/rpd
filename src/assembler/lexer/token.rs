use crate::instruction::Opcode;

#[allow(dead_code)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_number: u8 },
    IntegerOp { value: u16 },
}
