use tuplan_ir::{ByteStream, Inst, FromDiscriminant};
use std::io::Read;
use std::mem;

// TODO: We might aswell make this a union. We don't need the type at runtime.
#[derive(Debug)]
#[cfg(feature = "checked")]
pub enum Item {
    U64(u64),
    U32(u32),
}

#[cfg(feature = "checked")]
impl Item {
    #[inline]
    pub fn from_u64(val: u64) -> Item {
        Item::U64(val)
    }

    #[inline]
    pub fn from_u32(val: u32) -> Item {
        Item::U32(val)
    }

    #[inline]
    pub fn u64(&self) -> u64 {
        match self {
            Item::U64(val) => *val,
            _ => panic!("Expected U64"),
        }
    }

    #[inline]
    pub fn u32(&self) -> u32 {
        match self {
            Item::U32(val) => *val,
            _ => panic!("Expected U32"),
        }
    }
}

#[cfg(not(feature = "checked"))]
pub union Item {
    u64: u64,
    u32: u32,
}

#[cfg(not(feature = "checked"))]
impl Item {
    #[inline]
    pub fn from_u64(val: u64) -> Item {
        Item { u64: val }
    }
    
    #[inline]
    pub fn from_u32(val: u32) -> Item {
        Item { u32: val }
    }

    #[inline]
    pub unsafe fn u64(&self) -> u64 {
        self.u64
    }

    #[inline]
    pub unsafe fn u32(&self) -> u32 {
        self.u32
    }
}

// TODO: Make item not take up so much space without adding performance overhead
pub struct Vm {
    code: ByteStream,
    stack: Vec<Item>,
}

impl Vm {

    #[inline]
    #[cold]
    #[must_use]
    pub fn new(code: ByteStream) -> Vm {
        Vm { code, stack: Vec::new() }
    }

    #[cfg(feature = "checked")]
    pub fn run(&mut self) {
        todo!()
    }

    #[cfg(not(feature = "checked"))]
    #[allow(unused_must_use)]
    pub unsafe fn run(&mut self) {
        while let Some(header) = self.code.read_byte() {
            match Inst::from_discriminant(header).unwrap_unchecked() {
                Inst::PushU64 => {
                    let mut bytes = [0u8; 8];
                    self.code.read_into_const(&mut bytes);

                    let value = u64::from_le_bytes(bytes);
                    self.stack.push(Item::from_u64(value));
                },
                Inst::Pop => {
                    self.stack.pop();
                },
                Inst::Ret => {
                    let addr = self.stack.pop().unwrap_unchecked();
                    self.code.jump_unchecked(addr.u32() as usize);
                },
                Inst::Goto => {
                    let mut addr_bytes = [0u8; 4];
                    self.code.read_into_const(&mut addr_bytes);

                    let addr = u32::from_le_bytes(addr_bytes);
                    self.code.jump_unchecked(addr as usize);
                },
                Inst::AddU64 => {
                    let b = self.stack.pop().unwrap_unchecked().u64();
                    let a = self.stack.pop().unwrap_unchecked().u64();
                    self.stack.push(Item::from_u64(a + b));
                },
                Inst::PeekU64 => {
                    let val = self.stack.last().unwrap_unchecked().u64();
                    println!("{}", val)
                }
            }
        }
    }
}