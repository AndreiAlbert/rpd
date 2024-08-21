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
