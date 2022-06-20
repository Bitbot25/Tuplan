use std::{
    borrow::Cow,
    io::{Cursor, Read, Write},
};

pub const INST_PUSH_U64: u8 = 0;
pub const INST_POP: u8 = 1;
pub const INST_ADD_U64: u8 = 2;
pub const INST_DUMP_U64: u8 = 3;

// FIXME: Using a std::io::Cursor here is not neccessarily what we want. When we call write the pos is increased by one. Simplified: We cannot read what we write; only what it was initialized with.
pub type ByteStream = Cursor<Vec<u8>>;

pub fn bytestream() -> ByteStream {
    Cursor::new(Vec::with_capacity(32))
}

pub fn bytestream_with(bytes: &[u8]) -> ByteStream {
    Cursor::new(bytes.to_vec())
}

pub fn disassemble(mut bytes: &[u8]) -> String {
    let mut acc = String::with_capacity(bytes.len() * 8);
    while !bytes.is_empty() {
        let (cont, disassembly) = disassemble_single(bytes);
        acc.push_str(&disassembly);
        bytes = &bytes[cont..];
    }
    acc
}

pub fn disassemble_single(bytes: &[u8]) -> (usize, Cow<str>) {
    #[inline(always)]
    fn simple_instruction(name: &str) -> (usize, Cow<str>) {
        (1, Cow::Borrowed(name))
    }

    match bytes[0] {
        INST_PUSH_U64 => {
            let mut u64_bytes = [0; 8];
            Read::read_exact(&mut &bytes[1..9], &mut u64_bytes)
                .expect("Invalid bytecode sequence.");
            let value = u64::from_le_bytes(u64_bytes);

            (9, Cow::Owned(format!("PUSH_U64 {}", value)))
        }
        INST_POP => simple_instruction("POP"),
        INST_ADD_U64 => simple_instruction("ADD_U64"),
        INST_DUMP_U64 => simple_instruction("DUMP_U64"),
        _ => unimplemented!("Unknown instruction: {}", bytes[0]),
    }
}
