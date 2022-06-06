use std::{sync::RwLock, cell::RefCell, rc::Rc};

use crate::{cursor::Cursor, spec, Result, Span, guard::SpanGuard};

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self>;
}

// TODO: Implement a no-copy version.
pub struct ParseStream<'a> {
    cursor: Cursor<'a>,
    bound_spans: Rc<RefCell<Vec<Span>>>,
}

impl<'a> ParseStream<'a> {
    pub fn new(slice: &'a [char]) -> ParseStream<'a> {
        let bound_spans = Rc::new(RefCell::new(Vec::new()));
        ParseStream {
            cursor: Cursor::new(slice, Rc::clone(&bound_spans)),
            bound_spans,
        }
    }

    pub fn empty() -> ParseStream<'a> {
        let bound_spans = Rc::new(RefCell::new(Vec::new()));
        ParseStream {
            cursor: Cursor::new(&[], Rc::clone(&bound_spans)),
            bound_spans,
        }
    }

    #[inline]
    pub fn parse<P: Parse>(&mut self) -> Result<P> {
        P::parse(self)
    }

    // TODO: Add SpanPopGuard
    #[inline]
    pub fn push_span(&mut self) -> SpanGuard {
        let index = self.cursor.index();
        self.bound_spans.borrow_mut().push(Span { begin: index, end: index });
        SpanGuard::new(Rc::clone(&self.bound_spans))
    }

    /*#[inline]
    pub fn pop_span(&mut self) -> Span {
        self.bound_spans.borrow_mut().pop().expect("There is no span left to pop.")
    }*/

    /// Parse until the cursor is empty or there are only whitespaces left.
    pub fn exhaustive_parse<P: Parse>(&mut self) -> Result<Vec<P>> {
        let mut results = Vec::new();
        while !self.is_empty() && !self.is_only_whitespaces() {
            results.push(self.parse()?);
            debug_assert_eq!(self.bound_spans.borrow().len(), 0, "Expected span stack to be of length 0.");
        }
        Ok(results)
    }

    // FIXME: Remove this function and just pass a cursor to virtual parse instead. You could avoid incrementing the spans otherwise.
    #[inline]
    pub fn cur(&mut self) -> &mut Cursor<'a> {
        &mut self.cursor
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cursor.is_empty()
    }

    pub fn is_only_whitespaces(&self) -> bool {
        for c in self.cursor.iter() {
            if !spec::is_whitespace(c) {
                return false;
            }
        }
        true
    }

    pub fn skip_all(&mut self, mut pred: impl FnMut(char) -> bool) -> bool {
        let mut has_moved = false;
        while !self.is_empty() && pred(self.cursor.peek0().unwrap()) {
            has_moved = true;
            self.cursor.advance();
        }
        has_moved
    } 

    /// Tries to parse something using the `parse_fn` parameter.
    /// On failure; the cursor is reset to it's original value.
    pub fn virtual_parse<R>(
        &mut self,
        parse_fn: impl FnOnce(&mut ParseStream<'a>) -> Result<R>,
    ) -> Result<R> {
        // TODO: Maybe we can do something clever here to avoid expensive cloning?
        let original = self.cursor.clone();
        match parse_fn(self) {
            ok @ Ok(_) => ok,
            e @ Err(_) => {
                self.cursor = original;
                e
            }
        }
    }
}
