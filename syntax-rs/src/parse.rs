#[cfg(feature = "span")]
use crate::Span;
use crate::{cursor::Cursor, snapshot::Snapshot, spec, Result};

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self>;
}

pub struct ParseStream<'a> {
    cursor: Cursor<'a>,
}

impl<'a> ParseStream<'a> {
    pub fn new(slice: &'a str) -> ParseStream<'a> {
        ParseStream {
            cursor: Cursor::new(slice),
        }
    }

    pub fn empty() -> ParseStream<'a> {
        ParseStream {
            cursor: Cursor::new(""),
        }
    }

    #[inline]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot(self.cursor.index())
    }

    #[inline]
    #[cfg(feature = "span")]
    pub fn since(&self, snapshot: Snapshot) -> Span {
        Span {
            begin: snapshot.index(),
            end: self.cursor.index(),
        }
    }

    #[inline]
    #[cfg(feature = "debug")]
    pub fn parse<P: Parse>(&mut self) -> Result<P> {
        P::parse(self).debug_tap(|result| {
            if result.is_ok() {
                eprintln!(
                    "[P] at {}:{}    | Sucessfully parsed item {}.",
                    file!(),
                    line!(),
                    std::any::type_name::<P>()
                )
            } else {
                eprintln!(
                    "[P] at {}:{}    | Failed to parse item {}.",
                    file!(),
                    line!(),
                    std::any::type_name::<P>()
                )
            }
        })
    }

    #[inline]
    #[cfg(not(feature = "debug"))]
    pub fn parse<P: Parse>(&mut self) -> Result<P> {
        P::parse(self)
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
            ok @ Ok(_) => {
                #[cfg(feature = "debug")]
                eprintln!(
                    "[P&R] at {}:{} | Successfully parsed item {}.",
                    file!(),
                    line!(),
                    std::any::type_name::<R>()
                );
                ok
            }
            e @ Err(_) => {
                #[cfg(feature = "debug")]
                eprintln!(
                    "[P&R] at {}:{} | Failed to parse item {}. Reversed the cursor to {:?}",
                    file!(),
                    line!(),
                    std::any::type_name::<R>(),
                    self.cursor
                );
                self.cursor = original;
                e
            }
        }
    }
}
