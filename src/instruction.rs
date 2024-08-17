#[derive(Debug, PartialEq)]
pub enum Opcode {
    ZERO,
    ILLEGAL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    EQ,
    JEQ,
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
