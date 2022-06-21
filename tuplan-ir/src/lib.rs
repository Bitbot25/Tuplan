//! Tuplan IR specification
//!
//! `localset` - `(stack val: any), (inline slot: u32)` Sets the stack slot `slot` to `val`.
//! `localcopy` - `(inline slot: u32)` - Pushes the stack slot `slot` onto the top of the stack.
//! `pushu64` - `(inline val: u64)` Pushes `val` onto the stack.
//! `pop` - Pops any value off of the stack.
//! `ret` - `(stack retptr: u32)` Returns to `retptr`.
//! `goto` - `(inline loc: u32)` Moves the instruction pointer to `loc`.
//! `addu64` - `(stack a: u64), (stack b: u64)` Pushes a new `u64` onto the stack which is the result of adding `a` and `b`.
//! `peeku64` - `(stack val: u64)` Displays a u64 to stdout without popping it off the stack.

use disc::{disc, FromDiscriminant};
use std::mem;
use std::ops::Index;

#[disc]
pub enum Inst {
    LocalSet,
    LocalCopy,
    PushU64,
    Pop,
    Ret,
    Goto,
    GotoIf,
    GotoIfNot,
    AddU64,
    SubU64,
    LtU64,
    GtU64,
    PeekU64,
    PeekBool,
}

pub struct ByteStream {
    bytes: Vec<u8>,
    index: usize,
}

impl ByteStream {
    #[inline]
    #[cold]
    pub fn new() -> ByteStream {
        ByteStream {
            bytes: Vec::new(),
            index: 0,
        }
    }

    #[inline]
    #[cold]
    pub fn new_with_bytes(bytes: Vec<u8>) -> ByteStream {
        ByteStream { bytes, index: 0 }
    }

    #[inline]
    pub fn push(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.index >= self.bytes.len() {
            return None;
        }
        let byte = unsafe { *self.bytes.get_unchecked(self.index) };
        self.index += 1;
        Some(byte)
    }

    #[must_use]
    pub fn read_into_const<const N: usize>(&mut self, buf: &mut [u8; N]) -> bool {
        if N > self.bytes.len() {
            return false;
        }

        *buf = unsafe {
            self.bytes[self.index..self.index + N]
                .try_into()
                .unwrap_unchecked()
        };
        self.index += N;
        return true;
    }

    pub fn peek_byte(&mut self) -> Option<u8> {
        if self.index >= self.bytes.len() {
            return None;
        }
        unsafe { Some(*self.bytes.get_unchecked(self.index)) }
    }

    pub fn peek_into<'a>(&'a mut self, buf: &mut &'a [u8]) -> bool {
        let n = buf.len();
        if n > self.bytes.len() {
            return false;
        }

        *buf = &self.bytes[self.index..self.index + n];
        return true;
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<u8> {
        match self.bytes.get(index) {
            Some(v) => Some(*v),
            None => None,
        }
    }

    #[inline]
    pub fn jump_unchecked(&mut self, index: usize) {
        self.index = index;
    }
}

impl Index<usize> for ByteStream {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}

#[inline]
#[must_use]
#[cold]
pub fn bytestream() -> ByteStream {
    ByteStream::new()
}

#[inline]
#[must_use]
#[cold]
pub fn bytestream_with(bytes: Vec<u8>) -> ByteStream {
    ByteStream::new_with_bytes(bytes)
}

#[must_use]
pub fn disassemble(bytes: &ByteStream) -> String {
    let mut buf = String::with_capacity(bytes.len() * 2);
    let mut index = 0;
    while index < bytes.len() {
        index = disassemble_one(bytes, index, &mut buf);
        buf.push('\n');
    }
    buf
}

pub fn disassemble_one(bytes: &ByteStream, start: usize, buffer: &mut String) -> usize {
    #[inline(always)]
    fn simple(str: &str, index: usize, buf: &mut String) -> usize {
        buf.push_str(&*format!("{index} | {str}"));
        index + 1
    }

    let header = bytes
        .get(start)
        .expect("Invalid bytecode: Expected instruction header.");
    let inst: Inst =
        Inst::from_discriminant(header).expect("Invalid bytecode: Expected instruction header.");

    match inst {
        Inst::LocalSet => {
            let (slot, index) = get_u32(bytes, start + 1);
            buffer.push_str(&*format!("{start} | localset {slot}"));
            index
        }
        Inst::LocalCopy => {
            let (slot, index) = get_u32(bytes, start + 1);
            buffer.push_str(&*format!("{start} | localcopy {slot}"));
            index
        }
        Inst::PushU64 => {
            let (val, index) = get_u64(bytes, start + 1);
            buffer.push_str(&*format!("{start} | pushu64 {val}"));
            index
        }
        Inst::Pop => simple("pop", start, buffer),
        Inst::Ret => simple("ret", start, buffer),
        Inst::Goto => {
            let (val, index) = get_u32(bytes, start + 1);
            buffer.push_str(&*format!("{start} | goto {val}"));
            index
        }
        Inst::GotoIf => {
            let (val, index) = get_u32(bytes, start + 1);
            buffer.push_str(&*format!("{start} | gotoif {val}"));
            index
        }
        Inst::GotoIfNot => {
            let (val, index) = get_u32(bytes, start + 1);
            buffer.push_str(&*format!("{start} | gotoif {val}"));
            index
        }
        Inst::AddU64 => simple("addu64", start, buffer),
        Inst::SubU64 => simple("subu64", start, buffer),
        Inst::LtU64 => simple("ltu64", start, buffer),
        Inst::GtU64 => simple("gtu64", start, buffer),
        Inst::PeekU64 => simple("peeku64", start, buffer),
        Inst::PeekBool => simple("peekbool", start, buffer),
    }
}

fn get_u32(bytes: &ByteStream, start: usize) -> (u32, usize) {
    let mut u32_b = [0; mem::size_of::<u32>()];
    u32_b[0] = bytes[start];
    u32_b[1] = bytes[start + 1];
    u32_b[2] = bytes[start + 2];
    u32_b[3] = bytes[start + 3];
    (u32::from_le_bytes(u32_b), start + mem::size_of::<u32>())
}

fn get_u64(bytes: &ByteStream, start: usize) -> (u64, usize) {
    let mut u64_b = [0; mem::size_of::<u64>()];
    u64_b[0] = bytes[start];
    u64_b[1] = bytes[start + 1];
    u64_b[2] = bytes[start + 2];
    u64_b[3] = bytes[start + 3];
    u64_b[4] = bytes[start + 4];
    u64_b[5] = bytes[start + 5];
    u64_b[6] = bytes[start + 6];
    u64_b[7] = bytes[start + 7];
    (u64::from_le_bytes(u64_b), start + mem::size_of::<u64>())
}
