pub mod instruction;
pub mod vm;

fn main() {
    let mut test_vm = vm::VM::new();
    let test_bytes = vec![1, 1, 0, 1, 1, 2, 0, 1, 2, 3, 1, 2];
    test_vm.program = test_bytes;
    test_vm.run();
    println!("{:?}", test_vm.registers);
}
