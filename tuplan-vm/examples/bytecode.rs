#![feature(cursor_remaining)]

use tuplan_ir::{INST_PUSH_U64, INST_DUMP_U64, INST_ADD_U64, disassemble};
use tuplan_vm::Vm;

fn main() {
    let bytes_10 = u64::to_le_bytes(10);
    let bytes_20 = u64::to_le_bytes(20);
    let bytecode = tuplan_ir::bytestream_with(&[
        INST_PUSH_U64,
        bytes_10[0],
        bytes_10[1],
        bytes_10[2],
        bytes_10[3],
        bytes_10[4],
        bytes_10[5],
        bytes_10[6],
        bytes_10[7],
        INST_PUSH_U64,
        bytes_20[0],
        bytes_20[1],
        bytes_20[2],
        bytes_20[3],
        bytes_20[4],
        bytes_20[5],
        bytes_20[6],
        bytes_20[7],
        INST_ADD_U64,
        INST_DUMP_U64,
    ]);
    eprintln!("BYTECODE\n{}", disassemble(bytecode.remaining_slice()));
    let mut vm = Vm::new(bytecode);
    unsafe {
        vm.run();
    }
}