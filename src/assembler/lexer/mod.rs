use std::fmt::{write, Display, Write};

use token::Token;

use crate::instruction::Opcode;

mod token;

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

#[allow(dead_code)]
impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ERROR: {} on line {} column {}",
            self.message, self.line, self.column,
        )
        .unwrap();
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
            ch if ch.is_alphabetic() => {
                let word = self.read_word();
                match word.as_str() {
                    "load" => Some(Token::Op { code: Opcode::LOAD }),
                    "add" => Some(Token::Op { code: Opcode::ADD }),
                    "sub" => Some(Token::Op { code: Opcode::SUB }),
                    "mul" => Some(Token::Op { code: Opcode::MUL }),
                    "div" => Some(Token::Op { code: Opcode::DIV }),
                    "eq" => Some(Token::Op { code: Opcode::EQ }),
                    "jmp" => Some(Token::Op { code: Opcode::JMP }),
                    "jeq" => Some(Token::Op { code: Opcode::JEQ }),
                    _ => {
                        self.record_error(&format!("Unexpected word: {}", word));
                        None
                    }
                }
            }
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
            '\0' => None,
            _ => {
                self.record_error(&format!("Unexpected character: {}", self.current_char));
                self.read_char();
                None
            }
        }
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
            println!("skipping whitespace");
            self.read_char();
        }
    }

    pub fn read_word(&mut self) -> String {
        let mut result = String::new();
        while self.current_char.is_alphabetic() {
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
        println!("{} {}", self.current_line, self.current_column);
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

    #[test]
    fn test_tokenize_load() {
        let mut lx = Lexer::new("load $1 #10".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Op { code: Opcode::LOAD });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
        assert_eq!(tokens[2], Token::IntegerOp { value: 10 });
    }

    #[test]
    fn test_tokenize_add() {
        let mut lx = Lexer::new("add $1 $2 $3".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Op { code: Opcode::ADD });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
        assert_eq!(tokens[2], Token::Register { reg_number: 2 });
        assert_eq!(tokens[3], Token::Register { reg_number: 3 });
    }

    #[test]
    fn test_tokenize_sub() {
        let mut lx = Lexer::new("sub $1 $2 $3".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Op { code: Opcode::SUB });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
        assert_eq!(tokens[2], Token::Register { reg_number: 2 });
        assert_eq!(tokens[3], Token::Register { reg_number: 3 });
    }
    #[test]
    fn test_tokenize_div() {
        let mut lx = Lexer::new("div $1 $2 $3".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Op { code: Opcode::DIV });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
        assert_eq!(tokens[2], Token::Register { reg_number: 2 });
        assert_eq!(tokens[3], Token::Register { reg_number: 3 });
    }
    #[test]
    fn test_tokenize_mul() {
        let mut lx = Lexer::new("mul $1 $2 $3".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Op { code: Opcode::MUL });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
        assert_eq!(tokens[2], Token::Register { reg_number: 2 });
        assert_eq!(tokens[3], Token::Register { reg_number: 3 });
    }

    #[test]
    fn test_tokenize_jmp() {
        let mut lx = Lexer::new("jmp $1".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Op { code: Opcode::JMP });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
    }

    #[test]
    fn test_tokenize_eq() {
        let mut lx = Lexer::new("eq $1 $2".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Op { code: Opcode::EQ });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
        assert_eq!(tokens[2], Token::Register { reg_number: 2 });
    }

    #[test]
    fn test_tokenize_jeq() {
        let mut lx = Lexer::new("jeq $1".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
        let tokens = result_tokenization.unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Op { code: Opcode::JEQ });
        assert_eq!(tokens[1], Token::Register { reg_number: 1 });
    }

    #[test]
    fn test_errors() {
        let mut lx = Lexer::new("load $1 gibrish".to_string());
        let result_tokenization = lx.tokenize();
        assert!(!result_tokenization.is_ok());
        if let Err(errors) = result_tokenization {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn test_white_space() {
        let mut lx = Lexer::new("   load $1   ".to_string());
        let result_tokenization = lx.tokenize();
        assert!(result_tokenization.is_ok());
    }
}
