use crate::instruction::Opcode;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op {
        code: Opcode,
    },
    LabelDeclaration {
        value: String,
    },
    LabelUsage {
        value: String,
    },
    Register {
        reg_number: u8,
    },
    IntegerOp {
        value: i32,
    },
    Directive {
        directive_type: DirectiveType,
        literal: String,
    },
    StringLiteral {
        value: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum DirectiveType {
    Code,
    Data,
    Asciiz,
}
