use tuplan_ir::{Inst, disassemble};
use tuplan_vm::Vm;

fn main() {
    let bytecode = tuplan_ir::bytestream_with(vec![
        Inst::PushU64 as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        Inst::PeekU64 as u8,
        Inst::PushU64 as u8,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        Inst::AddU64 as u8,
        Inst::Goto as u8,
        9,
        0,
        0,
        0,  
    ]);
    let mut vm = Vm::new(bytecode);
    unsafe {
        vm.run();
    }
}