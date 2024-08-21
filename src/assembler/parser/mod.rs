use crate::instruction::Opcode;

use super::lexer::token::Token;

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
        while bytes.len() < 4 {
            bytes.push(0);
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
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
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
        println!("assembly instructions: {:?}", instructions);
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
            Opcode::INC | Opcode::DEC | Opcode::ALLOC => {
                let op = self.next_token();
                if !self.check_if_operand(&op) {
                    return Err(format!("Unexpeted operand for inc/dec/alloc"));
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

#[cfg(test)]
mod tests {
    use super::Token;
    use crate::{assembler::parser::Parser, instruction::Opcode};

    #[test]
    fn test_parse_load() {
        let tokens = [
            Token::Op { code: Opcode::LOAD },
            Token::Register { reg_number: 1 },
            Token::IntegerOp { value: 10 },
        ];
        let mut parser = Parser::new(tokens.to_vec());
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [1, 1, 0, 10]);
    }

    #[test]
    fn test_parse_add() {
        let tokens = [
            Token::Op { code: Opcode::ADD },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [2, 2, 0, 1])
    }

    #[test]
    fn test_parse_sub() {
        let tokens = [
            Token::Op { code: Opcode::SUB },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [3, 2, 0, 1])
    }

    #[test]
    fn test_parse_mul() {
        let tokens = [
            Token::Op { code: Opcode::MUL },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [4, 2, 0, 1])
    }

    #[test]
    fn test_parse_div() {
        let tokens = [
            Token::Op { code: Opcode::DIV },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [5, 2, 0, 1])
    }

    #[test]
    fn test_parse_jmp() {
        let tokens = [
            Token::Op { code: Opcode::JMP },
            Token::Register { reg_number: 10 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [6, 10, 0, 0]);
    }

    #[test]
    fn test_parse_eq() {
        let tokens = [
            Token::Op { code: Opcode::EQ },
            Token::Register { reg_number: 1 },
            Token::Register { reg_number: 2 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [7, 1, 2, 0]);
    }

    #[test]
    fn test_parse_jeq() {
        let tokens = [
            Token::Op { code: Opcode::JEQ },
            Token::Register { reg_number: 5 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [8, 5, 0, 0]);
    }

    #[test]
    fn test_parse_alloc() {
        let tokens = [
            Token::Op {
                code: Opcode::ALLOC,
            },
            Token::Register { reg_number: 5 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [9, 5, 0, 0]);
    }

    #[test]
    fn test_parse_inc() {
        let tokens = [
            Token::Op { code: Opcode::INC },
            Token::Register { reg_number: 5 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [10, 5, 0, 0]);
    }

    #[test]
    fn test_parse_dec() {
        let tokens = [
            Token::Op { code: Opcode::DEC },
            Token::Register { reg_number: 5 },
        ]
        .to_vec();
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), 1);
        let bytes = insts[0].to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, [11, 5, 0, 0]);
    }
}
