#[cfg(feature = "span")]
use crate::Span;
use crate::{snapshot::Snapshot, spec, Result};

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self>;
}

/// A `Vec<char>` with utiliy functions and and a `cursor` field to keep track of the current index.
pub struct ParseStream {
    /// All the chars in the collection
    chars: Vec<char>,
    /// `cursor` represents the current index in `chars`. `chars[cursor]` is the same as calling `peek`.
    cursor: usize,
}

impl ParseStream {
    pub fn new(slice: &str) -> ParseStream {
        ParseStream {
            chars: slice.chars().collect(),
            cursor: 0,
        }
    }

    pub fn empty() -> ParseStream {
        ParseStream {
            chars: vec![],
            cursor: 0,
        }
    }

    pub fn eats(&mut self, c: char) -> bool {
        match self.peek() {
            Some(t) if c == t => {
                self.cursor += 1;
                true
            }
            _ => false,
        }
    }

    pub fn eats_of(&mut self, chars: &[char]) -> bool {
        match self.peek() {
            Some(c) => {
                for t in chars {
                    if c == *t {
                        self.cursor += 1;
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    pub fn eat_of(&mut self, chars: &[char]) -> Option<char> {
        match self.peek() {
            Some(c) => {
                for t in chars {
                    if c == *t {
                        self.cursor += 1;
                        return Some(*t);
                    }
                }
                None
            }
            None => None,
        }
    }

    pub fn string_while(&mut self, mut pred: impl FnMut(char) -> bool) -> String {
        let mut buf = String::new();
        // SAFETY: We know !is_empty so it will always succeed.
        while !self.is_empty() && pred(unsafe { self.unchecked_peek() }) {
            buf.push(unsafe { *self.chars.get_unchecked(self.cursor) });
            self.cursor += 1;
        }
        buf
    }

    #[inline]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot(self.cursor)
    }

    #[inline]
    #[cfg(feature = "span")]
    pub fn since(&self, snapshot: Snapshot) -> Span {
        Span {
            begin: snapshot.index(),
            end: self.cursor,
        }
    }

    #[inline]
    pub fn parse<P: Parse>(&mut self) -> Result<P> {
        P::parse(self)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cursor >= self.chars.len()
    }

    pub fn is_only_whitespaces(&self) -> bool {
        for c in self.chars.iter().skip(self.cursor) {
            if !spec::is_whitespace(*c) {
                return false;
            }
        }
        true
    }

    pub fn rewind(&mut self, snap: Snapshot) {
        self.cursor = snap.index();
    }

    #[inline]
    /// Returns the next character in the stream.
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }

    pub fn skip_all(&mut self, mut pred: impl FnMut(char) -> bool) -> bool {
        let mut has_moved = false;

        // SAFETY: We always check !is_empty before using unsafe.
        while !self.is_empty() && pred(unsafe { self.unchecked_peek() }) {
            has_moved = true;
            self.cursor += 1;
        }
        has_moved
    }

    #[inline]
    unsafe fn unchecked_peek(&mut self) -> char {
        *self.chars.get_unchecked(self.cursor)
    }

    #[must_use]
    /// Advances the iterator by one character and returns the current one. It is unidiomatic to ignore the result.
    pub fn advance(&mut self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            // SAFETY: We always check !is_empty before using unsafe.
            let c = unsafe { self.unchecked_peek() };
            self.cursor += 1;
            Some(c)
        }
    }

    #[must_use]
    pub fn advance_if(&mut self, pred: impl FnOnce(char) -> bool) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            let c = unsafe { self.unchecked_peek() };
            if pred(c) {
                self.cursor += 1;
                Some(c)
            } else {
                None
            }
        }
    }

    pub fn rewinds<T>(&mut self, fun: impl FnOnce(&mut ParseStream) -> Result<T>) -> Result<T> {
        let orig = self.cursor;
        let r = fun(self);
        match r {
            ok @ Ok(_) => ok,
            err @ Err(_) => {
                self.cursor = orig;
                err
            }
        }
    }
}
