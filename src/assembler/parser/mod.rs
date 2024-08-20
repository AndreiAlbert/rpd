use crate::instruction::Opcode;

use super::lexer::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AssemblyInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblyInstruction {
    pub fn to_bytes(&mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => bytes.push(code as u8),
            },
            _ => panic!("expected opcode"),
        };
        for op in vec![self.operand1, self.operand2, self.operand3] {
            match op {
                Some(op) => AssemblyInstruction::extract_operands(op, &mut bytes),
                None => {}
            };
        }
        bytes
    }

    fn extract_operands(op: Token, bytes: &mut Vec<u8>) {
        match op {
            Token::Register { reg_number } => bytes.push(reg_number),
            Token::IntegerOp { value } => {
                let converted = value as u16;
                let high_part = converted & 0xFF00;
                let low_part = converted & 0x00FF;
                bytes.push(high_part as u8);
                bytes.push(low_part as u8);
            }
            _ => {}
        };
    }
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<AssemblyInstruction>, Vec<String>> {
        let mut errors: Vec<String> = vec![];
        let mut instructions: Vec<AssemblyInstruction> = vec![];
        while self.current < self.tokens.len() {
            match self.parse_instruction() {
                Ok(token) => instructions.push(token),
                Err(e) => errors.push(e),
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<AssemblyInstruction, String> {
        let opcode_token = self
            .next_token()
            .ok_or("expected an opcode but found none")?;

        let opcode = match opcode_token {
            Token::Op { code } => code,
            _ => return Err(format!("expected an opcode but found {:?}", opcode_token)),
        };

        let (operand1, operand2, operand3) = match opcode {
            Opcode::ADD | Opcode::SUB | Opcode::DIV | Opcode::MUL => {
                let op1 = self.next_token();
                let op2 = self.next_token();
                let op3 = self.next_token();
                if !self.check_if_operand(&op1) {
                    return Err(format!("Unexpected operand for add instruction {:?}", op1));
                } else if !self.check_if_operand(&op2) {
                    return Err(format!("Unexpected operand for add instruction {:?}", op2));
                } else if !self.check_if_operand(&op3) {
                    return Err(format!("Unexpected operand for add instruction {:?}", op3));
                } else {
                    (op1, op2, op3)
                }
            }
            Opcode::LOAD | Opcode::EQ => {
                let op1 = self.next_token();
                let op2 = self.next_token();
                if !self.check_if_operand(&op1) {
                    return Err(format!("Unexpected operand for add instruction {:?}", op1));
                } else if !self.check_if_operand(&op2) {
                    return Err(format!("Unexpected operand for add instruction {:?}", op2));
                } else {
                    (op1, op2, None)
                }
            }
            Opcode::JMP | Opcode::JEQ => {
                let op = self.next_token();
                if !self.check_if_operand(&op) {
                    return Err(format!("unexpected bla bla {:?}", op));
                }
                (op, None, None)
            }
            Opcode::ZERO => (None, None, None),
            _ => (None, None, None),
        };

        Ok(AssemblyInstruction {
            opcode: opcode_token,
            operand1,
            operand2,
            operand3,
        })
    }

    fn check_if_operand(&self, token: &Option<Token>) -> bool {
        if let Some(token) = token {
            return matches!(token, Token::IntegerOp { .. } | Token::Register { .. });
        } else {
            return false;
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn is_opcode(&self, token: &Token) -> bool {
        matches!(token, Token::Op { .. })
    }
}
