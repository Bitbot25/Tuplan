use crate::{cursor::Cursor, spec, Result, Span, snapshot::Snapshot};

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self>;
    fn parse_from_str(input: &str) -> Result<Self> {
        // FIXME: This could be alot cleaner if the cursor just "iterated" over utf-8 codepoints on a str.
        Self::parse(&mut ParseStream::new(input.chars().collect::<Vec<char>>().as_slice()))
    }
}

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

    #[inline]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot(self.cursor.index())
    }

    #[inline]
    pub fn since(&self, snapshot: Snapshot) -> Span {
        Span { begin: snapshot.index(), end: self.cursor.index() }
    } 

    #[inline]
    pub fn parse<P: Parse>(&mut self) -> Result<P> {
        P::parse(self)
    }
    
    /// Parse until the cursor is empty or there are only whitespaces left.
    pub fn exhaustive_parse<P: Parse>(&mut self) -> Result<Vec<P>> {
        let mut results = Vec::new();
        while !self.is_empty() && !self.is_only_whitespaces() {
            results.push(self.parse()?);
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
    /// On failure; the cursor is reset to it's original value.<br>
    /// **NOTE: This function is generally much more expensive than just doing a simple check before parsing.**
    pub fn try_parse<R>(
        &mut self,
        parse_fn: impl FnOnce(&mut ParseStream<'a>) -> Result<R>,
    ) -> Result<R> {
        // FIXME: Do i need to reset the spans on error?
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
