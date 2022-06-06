use std::{cell::RefCell, rc::Rc};

use crate::{Span, parse::ParseStream};

// FIXME: This is slow to construct. Maybe implement utf8.rs so that we can get the next unicode code point? It would be alot faster.
// TODO: Change Rc<RefCell<T>> to just a raw pointer or reference of some kind.
#[derive(Clone)]
pub struct Cursor<'a> {
    slice: &'a [char],
    index: usize,
    bound_spans: Rc<RefCell<Vec<Span>>>,
}

impl<'a> Cursor<'a> {
    pub fn new(slice: &'a [char], bound_spans: Rc<RefCell<Vec<Span>>>) -> Cursor<'a> {
        Cursor { slice, index: 0, bound_spans }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.index >= self.slice.len()
    }

    #[inline]
    pub fn peek0(&self) -> Option<char> {
        self.slice.get(self.index).copied()
    }

    #[inline]
    pub fn peek_range(&self, n: usize) -> Option<&'a [char]> {
        self.slice.get(self.index..n)
    }

    pub fn consume(&mut self, target: char) -> bool {
        match self.peek0() {
            Some(c) => c == target,
            _ => false,
        }
    }

    // FIXME: How would i make the spans update in real-time? This wouldn't be a problem when we remove the reference to the stream in virtual_parse
    pub fn advance(&mut self) -> Option<char> {
        let c = self.slice.get(self.index).copied();
        if c.is_some() {
            self.index += 1;
            for span in &mut *self.bound_spans.borrow_mut() {
                (*span).end = self.index;
            }
        }
        c
    }

    pub fn advance_n(&mut self, n: usize) -> Option<&'a [char]> {
        let slice = self.slice.get(self.index..n);
        if slice.is_some() {
            self.index += n;
        }
        slice
    }

    pub fn advance_while(&mut self, mut pred: impl FnMut(char) -> bool) -> &'a [char] {
        let begin = self.index;
        while !self.is_empty() && pred(self.peek0().unwrap()) {
            self.index += 1;
        }
        // NOTE: Is this correct?
        for span in &mut *self.bound_spans.borrow_mut() {
            (*span).end = self.index;
        }
        &self.slice[begin..self.index]
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn iter<'c>(&'c self) -> Iter<'a, 'c> {
        Iter(0, self)
    }
}

pub struct Iter<'a, 'b>(usize, &'b Cursor<'a>);

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.1.slice.get(self.0 + self.1.index).copied();
        self.0 += 1;
        c
    }
}
