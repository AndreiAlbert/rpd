use std::fmt::{Display, Write};

use token::{DirectiveType, Token};

use crate::instruction::Opcode;

pub mod token;

#[allow(dead_code)]
pub struct Lexer {
    source: Vec<char>,
    current_char: char,
    position: usize,
    read_position: usize,
    current_line: usize,
    current_column: usize,
    errors: Vec<LexerError>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LexerError {
    message: String,
    line: usize,
    column: usize,
    context: Option<String>,
}

#[allow(unused)]
impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ERROR: {} on line {} column {}",
            self.message, self.line, self.column,
        );
        Ok(())
    }
}

impl LexerError {
    pub fn new(message: &str, line: usize, column: usize, context: Option<String>) -> LexerError {
        return LexerError {
            message: message.to_string(),
            line,
            column,
            context,
        };
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            source: input.chars().collect(),
            current_char: '0',
            position: 0,
            read_position: 0,
            current_line: 1,
            current_column: 0,
            errors: Vec::new(),
        };
        lexer.read_char();
        return lexer;
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, &Vec<LexerError>> {
        let mut tokens: Vec<Token> = vec![];
        while self.read_position < self.source.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        if !self.errors.is_empty() {
            return Err(&self.errors);
        }
        return Ok(tokens);
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.current_char {
            ch if ch.is_alphabetic() || ch == '@' || ch == '.' || ch == '"' => {
                if ch == '"' {
                    let string_literal = self.read_string_literal();
                    return Some(Token::StringLiteral {
                        value: string_literal,
                    });
                }
                let mut word = self.read_word();
                let mut chars = word.chars();
                if let Some(ch) = chars.nth_back(0) {
                    if ch == ':' {
                        word.pop();
                        return Some(Token::LabelDeclaration { value: word });
                    }
                }
                let mut chars = word.chars();
                if let Some(ch) = chars.nth(0) {
                    if ch == '@' {
                        word.remove(0);
                        return Some(Token::LabelUsage { value: word });
                    }
                }
                println!("word {}", word);
                match word.as_str() {
                    ".asciiz" => Some(Token::Directive {
                        directive_type: DirectiveType::Asciiz,
                        literal: ".asciiz".to_string(),
                    }),
                    ".code" => Some(Token::Directive {
                        directive_type: DirectiveType::Code,
                        literal: ".code".to_string(),
                    }),
                    ".data" => Some(Token::Directive {
                        directive_type: DirectiveType::Data,
                        literal: ".data".to_string(),
                    }),
                    "load" => Some(Token::Op { code: Opcode::LOAD }),
                    "add" => Some(Token::Op { code: Opcode::ADD }),
                    "sub" => Some(Token::Op { code: Opcode::SUB }),
                    "mul" => Some(Token::Op { code: Opcode::MUL }),
                    "div" => Some(Token::Op { code: Opcode::DIV }),
                    "eq" => Some(Token::Op { code: Opcode::EQ }),
                    "jmp" => Some(Token::Op { code: Opcode::JMP }),
                    "jeq" => Some(Token::Op { code: Opcode::JEQ }),
                    "jneq" => Some(Token::Op { code: Opcode::JNEQ }),
                    "alloc" => Some(Token::Op {
                        code: Opcode::ALLOC,
                    }),
                    "inc" => Some(Token::Op { code: Opcode::INC }),
                    "dec" => Some(Token::Op { code: Opcode::DEC }),
                    _ => {
                        self.record_error(&format!("Unexpected word: {:?}", word.into_bytes()));
                        None
                    }
                }
            }

            '\0' => Some(Token::Op { code: Opcode::ZERO }),
            '$' => {
                self.read_char();
                if self.current_char.is_numeric() {
                    let register = self.read_number() as u8;
                    Some(Token::Register {
                        reg_number: register,
                    })
                } else {
                    self.record_error(&format!("Expected number after '$' symbol"));
                    self.read_char();
                    None
                }
            }
            '#' => {
                self.read_char();
                if self.current_char.is_numeric() {
                    let number = self.read_number();
                    Some(Token::IntegerOp { value: number })
                } else {
                    self.record_error(&format!("Expected number after '#' symbol"));
                    None
                }
            }
            _ => {
                self.record_error(&format!("Unexpected character: {}", self.current_char));
                self.read_char();
                None
            }
        }
    }

    fn read_string_literal(&mut self) -> String {
        self.read_char();
        let mut result = String::new();
        while self.current_char != '"' {
            result.push(self.current_char);
            self.read_char();
        }
        self.read_char();
        return result;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.source[self.read_position];
            self.current_column += 1;
        }
        if self.current_char == '\n' {
            self.current_line += 1;
            self.current_column = 0;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.current_char == ' '
            || self.current_char == '\t'
            || self.current_char == '\n'
            || self.current_char == '\r'
        {
            self.read_char();
        }
    }

    pub fn read_word(&mut self) -> String {
        let mut result = String::new();
        while self.current_char.is_alphanumeric()
            || self.current_char == ':'
            || self.current_char == '_'
            || self.current_char == '-'
            || self.current_char == '@'
            || self.current_char == '@'
            || self.current_char == '.'
        {
            if let Err(_) = write!(&mut result, "{}", self.current_char) {
                panic!(
                    "Could not add char to the string buffer {}",
                    self.current_char
                )
            }
            self.read_char();
        }
        return result;
    }

    pub fn read_number(&mut self) -> i32 {
        let mut result = String::new();
        while self.current_char.is_numeric() {
            if let Err(_) = write!(&mut result, "{}", self.current_char) {
                panic!(
                    "Coud not add char to the string buffer {}",
                    self.current_char
                );
            }
            self.read_char();
        }
        return result.parse::<i32>().expect("Failed to parse number");
    }

    fn record_error(&mut self, message: &str) {
        let context = self.get_context();
        let err = LexerError::new(message, self.current_line, self.current_column, context);
        self.errors.push(err);
    }

    fn get_context(&self) -> Option<String> {
        Some(format!("Characther: {}", self.current_char))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize_and_check(input: &str, expected_tokens: &[Token], expected_len: usize) {
        let mut lexer = Lexer::new(input.to_string());
        let result_tokenization = lexer.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), expected_len);
        assert_eq!(tokens, expected_tokens.to_vec());
    }

    fn tokenize_and_expect_error(input: &str) {
        let mut lexer = Lexer::new(input.to_string());
        let result_tokenization = lexer.tokenize();
        assert!(!result_tokenization.is_ok());
        if let Err(errors) = result_tokenization {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn test_tokenize_load() {
        tokenize_and_check(
            "load $1 #10",
            &[
                Token::Op { code: Opcode::LOAD },
                Token::Register { reg_number: 1 },
                Token::IntegerOp { value: 10 },
            ],
            3,
        );
    }

    #[test]
    fn test_tokenize_add() {
        tokenize_and_check(
            "add $1 $2 $3",
            &[
                Token::Op { code: Opcode::ADD },
                Token::Register { reg_number: 1 },
                Token::Register { reg_number: 2 },
                Token::Register { reg_number: 3 },
            ],
            4,
        );
    }

    #[test]
    fn test_tokenize_sub() {
        tokenize_and_check(
            "sub $1 $2 $3",
            &[
                Token::Op { code: Opcode::SUB },
                Token::Register { reg_number: 1 },
                Token::Register { reg_number: 2 },
                Token::Register { reg_number: 3 },
            ],
            4,
        );
    }

    #[test]
    fn test_tokenize_div() {
        tokenize_and_check(
            "div $1 $2 $3",
            &[
                Token::Op { code: Opcode::DIV },
                Token::Register { reg_number: 1 },
                Token::Register { reg_number: 2 },
                Token::Register { reg_number: 3 },
            ],
            4,
        );
    }

    #[test]
    fn test_tokenize_mul() {
        tokenize_and_check(
            "mul $1 $2 $3",
            &[
                Token::Op { code: Opcode::MUL },
                Token::Register { reg_number: 1 },
                Token::Register { reg_number: 2 },
                Token::Register { reg_number: 3 },
            ],
            4,
        );
    }

    #[test]
    fn test_tokenize_jmp() {
        tokenize_and_check(
            "jmp $1",
            &[
                Token::Op { code: Opcode::JMP },
                Token::Register { reg_number: 1 },
            ],
            2,
        );
    }

    #[test]
    fn test_tokenize_eq() {
        tokenize_and_check(
            "eq $1 $2",
            &[
                Token::Op { code: Opcode::EQ },
                Token::Register { reg_number: 1 },
                Token::Register { reg_number: 2 },
            ],
            3,
        );
    }

    #[test]
    fn test_tokenize_jeq() {
        tokenize_and_check(
            "jeq $1",
            &[
                Token::Op { code: Opcode::JEQ },
                Token::Register { reg_number: 1 },
            ],
            2,
        );
    }

    #[test]
    fn test_tokenize_alloc() {
        tokenize_and_check(
            "alloc $0",
            &[
                Token::Op {
                    code: Opcode::ALLOC,
                },
                Token::Register { reg_number: 0 },
            ],
            2,
        );
    }

    #[test]
    fn test_tokenize_inc() {
        tokenize_and_check(
            "inc $0",
            &[
                Token::Op { code: Opcode::INC },
                Token::Register { reg_number: 0 },
            ],
            2,
        );
    }

    #[test]
    fn test_tokenize_dec() {
        tokenize_and_check(
            "dec $0",
            &[
                Token::Op { code: Opcode::DEC },
                Token::Register { reg_number: 0 },
            ],
            2,
        );
    }

    #[test]
    fn test_tokenize_label_decl() {
        tokenize_and_check(
            r###"
                test_label:
                load $1 #10
                jmp @test_label
            "###,
            &[
                Token::LabelDeclaration {
                    value: "test_label".to_string(),
                },
                Token::Op { code: Opcode::LOAD },
                Token::Register { reg_number: 1 },
                Token::IntegerOp { value: 10 },
                Token::Op { code: Opcode::JMP },
                Token::LabelUsage {
                    value: "test_label".to_string(),
                },
                Token::Op { code: Opcode::ZERO },
            ],
            7,
        );
    }

    #[test]
    fn test_tokenize_directives() {
        tokenize_and_check(
            r###"
                .data
                string: .asciiz "hello world!"
            "###,
            &[
                Token::Directive {
                    directive_type: DirectiveType::Data,
                    literal: ".data".to_string(),
                },
                Token::LabelDeclaration {
                    value: "string".to_string(),
                },
                Token::Directive {
                    directive_type: DirectiveType::Asciiz,
                    literal: ".asciiz".to_string(),
                },
                Token::StringLiteral {
                    value: "hello world!".to_string(),
                },
                Token::Op { code: Opcode::ZERO },
            ],
            5,
        )
    }

    #[test]
    fn test_errors() {
        tokenize_and_expect_error("load $1 gibrish");
    }
}
