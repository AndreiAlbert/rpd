mod token;

#[allow(dead_code)]
pub struct Lexer {
    source: Vec<char>,
    current_char: char,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        return Lexer {
            source: input.chars().collect(),
            current_char: '0',
            position: 0,
            read_position: 0,
        };
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.current_char = '0';
        } else {
            self.current_char = self.source[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.current_char;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }
}
