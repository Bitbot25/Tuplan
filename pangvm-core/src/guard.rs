use std::{ops::Drop, rc::Rc, cell::{RefCell, Cell}};

use crate::Span;
use std::mem;

pub struct SpanGuard {
    spans: Rc<RefCell<Vec<Span>>>,
    popped: Cell<bool>,
}

impl SpanGuard {
    pub fn new(spans: Rc<RefCell<Vec<Span>>>) -> SpanGuard {
        SpanGuard { spans, popped: Cell::new(false) }
    }

    pub fn into_inner(self) -> Span {
        let value = self.spans.borrow_mut().pop().expect("All spans are gone?");
        self.popped.set(true);
        value
    }
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        if !self.popped.get() {
            self.spans.borrow_mut().pop();
        }
    }
}