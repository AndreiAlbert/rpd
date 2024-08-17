#[derive(Debug, PartialEq)]
pub enum Opcode {
    END,
    ILLEGAL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
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
            0 => Self::END,
            1 => Self::LOAD,
            2 => Self::ADD,
            3 => Self::SUB,
            4 => Self::MUL,
            5 => Self::DIV,
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
            opcode: Opcode::END,
        };
        assert_eq!(inst.opcode, Opcode::END);
    }
}
