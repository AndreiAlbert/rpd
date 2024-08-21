pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    // let program = r###"
    //     load $1 #1
    //     load $2 #2
    //     add $3 $1 $2
    //     load $4 #3
    //     eq $4 $3
    // "###
    // .to_string();
    // Assembler::run(program);
    let mut repl = repl::REPL::new();
    repl.run();
}
