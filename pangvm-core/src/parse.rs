use crate::{cursor::Cursor, spec, Result};

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self>;
}

// TODO: Implement a no-copy version.
pub struct ParseStream<'a> {
    cursor: Cursor<'a>,
}

impl<'a> ParseStream<'a> {
    pub fn new(slice: &'a [char]) -> ParseStream<'a> {
        ParseStream {
            cursor: Cursor::new(slice),
        }
    }

    pub fn empty() -> ParseStream<'a> {
        ParseStream {
            cursor: Cursor::new(&[]),
        }
    }

    pub fn parse<P: Parse>(&mut self) -> Result<P> {
        P::parse(self)
    }

    /// Parse until the cursor is empty or there are only whitespaces left.
    pub fn exhaustive_parse<P: Parse>(&mut self) -> Result<Vec<P>> {
        let mut results = Vec::new();
        while !self.is_empty() && !self.is_only_whitespaces() {
            results.push(P::parse(self)?);
        }
        Ok(results)
    }

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
        let original = self.cursor;
        match parse_fn(self) {
            ok @ Ok(_) => ok,
            e @ Err(_) => {
                self.cursor = original;
                e
            }
        }
    }
}
