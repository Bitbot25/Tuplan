use tuplan_ir::{ByteStream, INST_ADD_U64, INST_DUMP_U64, INST_PUSH_U64, INST_POP};

// TODO: We might aswell make this a union. We don't need the type at runtime.
#[derive(Debug)]
pub enum Item {
    U64(u64),
}

impl AsRef<u64> for Item {
    fn as_ref(&self) -> &u64 {
        match self {
            Item::U64(val) => val,
            _ => panic!("{:?} cannot dereference to a u64.", self)
        }
    }
}

pub struct Vm {
    code: ByteStream,
    stack: Vec<Item>,
}

use std::io::Read;

mod instristic {
    use super::Item;

    pub fn add_u(a: Item, b: Item) -> Item {
        let a: u64 = *a.as_ref();
        let b: u64 = *b.as_ref();
        Item::U64(a + b)
    }

    pub fn dump_u(item: Item) {
        let u: u64 = *item.as_ref();
        println!("{}", u);
    }
}

impl Vm {
    pub fn new(code: ByteStream) -> Vm {
        Vm { code, stack: Vec::new() }
    }

    pub unsafe fn run(&mut self) {
        let mut inst = [0; 1];
        while let Ok(_) = self.code.read_exact(&mut inst) {
            let inst = inst[0];
            match inst {
                INST_PUSH_U64 => {
                    let mut bytes = [0; 8];
                    self.code.read_exact(&mut bytes).unwrap_unchecked();
                    let value = u64::from_le_bytes(bytes);
                    self.stack.push(Item::U64(value));
                },
                INST_POP => {
                    self.stack.pop();
                },
                INST_ADD_U64 => {
                    let b = self.stack.pop().unwrap_unchecked();
                    let a = self.stack.pop().unwrap_unchecked();
                    let result = instristic::add_u(a, b);
                    self.stack.push(result);
                },
                INST_DUMP_U64 => {
                    let item = self.stack.pop().unwrap_unchecked();
                    instristic::dump_u(item);
                },
                _ => panic!("Unknown instruction: {}", inst)
            }
        }
    }
}