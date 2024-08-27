use crate::instruction::Opcode;

use super::{
    lexer::token::{DirectiveType, Token},
    symbol::symbol_table::SymbolTable,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct AssemblyInstruction {
    opcode: Option<Token>,
    pub label: Option<Token>,
    directive: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblyInstruction {
    pub fn to_bytes(&mut self, symbol_table: &SymbolTable) -> Option<Vec<u8>> {
        if let None = self.opcode {
            return None;
        }
        let mut bytes: Vec<u8> = vec![];
        if let Some(token) = self.opcode.clone() {
            match token {
                Token::Op { code } => match code {
                    _ => bytes.push(u8::from(code)),
                },

                _ => {}
            };
        }
        for op in vec![
            self.label.clone(),
            self.operand1.clone(),
            self.operand2.clone(),
            self.operand3.clone(),
        ] {
            match op {
                Some(op) => AssemblyInstruction::extract_operands(op, &mut bytes, &symbol_table),
                None => {}
            };
        }
        while bytes.len() < 4 {
            bytes.push(0);
        }
        Some(bytes)
    }

    fn extract_operands(op: Token, bytes: &mut Vec<u8>, st: &SymbolTable) {
        match op {
            Token::Register { reg_number } => bytes.push(reg_number),
            Token::IntegerOp { value } => {
                let converted = value as u16;
                let high_part = converted & 0xFF00;
                let low_part = converted & 0x00FF;
                bytes.push(high_part as u8);
                bytes.push(low_part as u8);
            }
            Token::LabelUsage { value } => {
                if let Some(offset) = st.get_symbol_value(&value) {
                    bytes.push(offset);
                }
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
        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<AssemblyInstruction, String> {
        let token = self
            .next_token()
            .ok_or("expected an opcode but found none")?;

        match token {
            Token::LabelDeclaration { .. } => {
                return Ok(AssemblyInstruction {
                    directive: None,
                    opcode: None,
                    operand1: None,
                    operand2: None,
                    operand3: None,
                    label: Some(token),
                });
            }
            Token::Directive { directive_type, .. } => {
                if directive_type == DirectiveType::Asciiz {
                    if let Some(string_literla_tok) = self.next_token() {
                        match string_literla_tok {
                            Token::StringLiteral { .. } => {
                                return Ok(AssemblyInstruction {
                                    directive: Some(token),
                                    operand3: None,
                                    operand2: None,
                                    operand1: Some(string_literla_tok),
                                    opcode: None,
                                    label: None,
                                });
                            }
                            _ => {
                                return Err(format!(
                                    "Expected string literal after asciiz directive"
                                ))
                            }
                        }
                    }
                }
                return Ok(AssemblyInstruction {
                    directive: Some(token),
                    opcode: None,
                    operand1: None,
                    operand2: None,
                    operand3: None,
                    label: None,
                });
            }
            _ => {}
        };

        if let Token::LabelDeclaration { .. } = token {
            return Ok(AssemblyInstruction {
                opcode: None,
                directive: None,
                operand1: None,
                operand2: None,
                operand3: None,
                label: Some(token),
            });
        }

        let opcode = match token {
            Token::Op { code } => code,
            _ => return Err(format!("expected an opcode but found {:?}", token)),
        };

        let (operand1, operand2, operand3, label) = match opcode {
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
                    (op1, op2, op3, None)
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
                    (op1, op2, None, None)
                }
            }
            Opcode::JMP | Opcode::JEQ | Opcode::JNEQ => {
                let op = self.next_token();
                if !self.check_if_operand(&op) {
                    return Err(format!("unexpected bla bla {:?}", op));
                }
                (None, None, None, op)
            }
            Opcode::INC | Opcode::DEC | Opcode::ALLOC => {
                let op = self.next_token();
                if !self.check_if_operand(&op) {
                    return Err(format!("Unexpeted operand for inc/dec/alloc"));
                }
                (op, None, None, None)
            }
            Opcode::ZERO => (None, None, None, None),
            _ => (None, None, None, None),
        };

        Ok(AssemblyInstruction {
            opcode: Some(token),
            operand1,
            operand2,
            operand3,
            label,
            directive: None,
        })
    }

    fn check_if_operand(&self, token: &Option<Token>) -> bool {
        if let Some(token) = token {
            return matches!(
                token,
                Token::IntegerOp { .. } | Token::Register { .. } | Token::LabelUsage { .. }
            );
        } else {
            return false;
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
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
    use crate::{
        assembler::{
            lexer::token::DirectiveType,
            parser::{AssemblyInstruction, Parser},
            symbol::symbol_table::SymbolTable,
        },
        instruction::Opcode,
    };

    fn parse_and_check(tokens: Vec<Token>, expected_bytes: Option<&[u8]>, expected_len: usize) {
        let mut parser = Parser::new(tokens);
        let insts = parser.parse();
        assert!(insts.is_ok());
        let mut insts = insts.unwrap();
        assert_eq!(insts.len(), expected_len);
        if let Some(expected_bytes) = expected_bytes {
            let st = SymbolTable::new();
            let bytes = insts[0].to_bytes(&st);
            assert!(bytes.is_some());
            if let Some(bytes) = bytes {
                assert_eq!(bytes.len(), expected_bytes.len());
                assert_eq!(bytes, expected_bytes);
            }
        }
    }

    #[test]
    fn test_parse_load() {
        let tokens = vec![
            Token::Op { code: Opcode::LOAD },
            Token::Register { reg_number: 1 },
            Token::IntegerOp { value: 10 },
        ];
        parse_and_check(tokens, Some(&[1, 1, 0, 10]), 1);
    }

    #[test]
    fn test_parse_add() {
        let tokens = vec![
            Token::Op { code: Opcode::ADD },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ];
        parse_and_check(tokens, Some(&[2, 2, 0, 1]), 1);
    }

    #[test]
    fn test_parse_sub() {
        let tokens = vec![
            Token::Op { code: Opcode::SUB },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ];
        parse_and_check(tokens, Some(&[3, 2, 0, 1]), 1);
    }

    #[test]
    fn test_parse_mul() {
        let tokens = vec![
            Token::Op { code: Opcode::MUL },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ];
        parse_and_check(tokens, Some(&[4, 2, 0, 1]), 1);
    }

    #[test]
    fn test_parse_div() {
        let tokens = vec![
            Token::Op { code: Opcode::DIV },
            Token::Register { reg_number: 2 },
            Token::Register { reg_number: 0 },
            Token::Register { reg_number: 1 },
        ];
        parse_and_check(tokens, Some(&[5, 2, 0, 1]), 1);
    }

    #[test]
    fn test_parse_jmp() {
        let tokens = vec![
            Token::Op { code: Opcode::JMP },
            Token::Register { reg_number: 10 },
        ];
        parse_and_check(tokens, Some(&[6, 10, 0, 0]), 1);
    }

    #[test]
    fn test_parse_eq() {
        let tokens = vec![
            Token::Op { code: Opcode::EQ },
            Token::Register { reg_number: 1 },
            Token::Register { reg_number: 2 },
        ];
        parse_and_check(tokens, Some(&[7, 1, 2, 0]), 1);
    }

    #[test]
    fn test_parse_jeq() {
        let tokens = vec![
            Token::Op { code: Opcode::JEQ },
            Token::Register { reg_number: 5 },
        ];
        parse_and_check(tokens, Some(&[8, 5, 0, 0]), 1);
    }

    #[test]
    fn test_parse_alloc() {
        let tokens = vec![
            Token::Op {
                code: Opcode::ALLOC,
            },
            Token::Register { reg_number: 5 },
        ];
        parse_and_check(tokens, Some(&[9, 5, 0, 0]), 1);
    }

    #[test]
    fn test_parse_inc() {
        let tokens = vec![
            Token::Op { code: Opcode::INC },
            Token::Register { reg_number: 5 },
        ];
        parse_and_check(tokens, Some(&[10, 5, 0, 0]), 1);
    }

    #[test]
    fn test_parse_dec() {
        let tokens = vec![
            Token::Op { code: Opcode::DEC },
            Token::Register { reg_number: 5 },
        ];
        parse_and_check(tokens, Some(&[11, 5, 0, 0]), 1);
    }

    #[test]
    fn test_parse_labels() {
        let tokens = vec![
            Token::Op { code: Opcode::JMP },
            Token::LabelUsage {
                value: "label".to_string(),
            },
        ];
        parse_and_check(tokens, None, 1);
    }

    #[test]
    fn test_parse_directives() {
        let tokens = vec![
            Token::Directive {
                directive_type: DirectiveType::Data,
                literal: ".data".to_string(),
            },
            Token::LabelDeclaration {
                value: "test".to_string(),
            },
            Token::Directive {
                directive_type: DirectiveType::Asciiz,
                literal: ".asciiz".to_string(),
            },
            Token::StringLiteral {
                value: "goodbye world!".to_string(),
            },
        ];
        let mut parser = Parser::new(tokens);
        let result_parse = parser.parse();
        assert!(result_parse.is_ok());
        todo!();
    }
}
