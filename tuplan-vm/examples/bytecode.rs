use std::time::Instant;

use tuplan_ir::{disassemble, Inst};
use tuplan_vm::Vm;

fn main() {
    /*

    i = 0; -- pushu64 0
    while i < 10 { -- localcopy 0; pushu64 10; ltu64; gotoifnot <end of loop>
        i += 1; -- pushu64
    }

     */

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
        Inst::LocalCopy as u8,
        0,
        0,
        0,
        0,
        Inst::PushU64 as u8,
        255,
        255,
        0,
        0,
        0,
        0,
        0,
        0,
        Inst::LtU64 as u8,
        Inst::GotoIfNot as u8,
        55,
        0,
        0,
        0,
        Inst::LocalCopy as u8,
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
        Inst::LocalSet as u8,
        0,
        0,
        0,
        0,
        Inst::Goto as u8,
        9,
        0,
        0,
        0,
    ]);
    //println!("{}", disassemble(&bytecode));
    let mut vm = Vm::new(bytecode);
    unsafe {
        vm.run();
    }
}
