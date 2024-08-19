pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

use assembler::lexer::Lexer;

fn main() {
    let mut lx = Lexer::new("    load $1\n dsfs".to_string());
    match lx.tokenize() {
        Ok(tokens) => {
            println!("Tokenization was successfull");
            println!("{:?}", tokens);
        }
        Err(errors) => {
            println!("Errors encountered");
            println!("{:?}", errors);
        }
    }
}
