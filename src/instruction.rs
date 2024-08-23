#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    ZERO,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    EQ,
    JEQ,
    JNEQ,
    ALLOC,
    INC,
    DEC,
    ILLEGAL,
}

#[allow(dead_code)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(op: Opcode) -> Instruction {
        return Instruction { opcode: op };
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::ZERO => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MUL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::EQ => 7,
            Opcode::JEQ => 8,
            Opcode::ALLOC => 9,
            Opcode::INC => 10,
            Opcode::DEC => 11,
            Opcode::JNEQ => 12,
            Opcode::ILLEGAL => panic!("cannot convert to u8 from illegal"),
        }
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        return match byte {
            0 => Self::ZERO,
            1 => Self::LOAD,
            2 => Self::ADD,
            3 => Self::SUB,
            4 => Self::MUL,
            5 => Self::DIV,
            6 => Self::JMP,
            7 => Self::EQ,
            8 => Self::JEQ,
            9 => Self::ALLOC,
            10 => Self::INC,
            11 => Self::DEC,
            12 => Self::JNEQ,
            _ => Self::ILLEGAL,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instrunction() {
        let inst = Instruction {
            opcode: Opcode::ZERO,
        };
        assert_eq!(inst.opcode, Opcode::ZERO);
    }
}
