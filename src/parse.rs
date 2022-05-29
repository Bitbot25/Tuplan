use std::cell::Cell;

use crate::{Result, token::Cursor};

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream<'_>) -> Result<Self>;
}

// TODO: Implement a no-copy version.
pub struct ParseStream<'a> {
    cursor: Cell<Cursor<'a>>,
}

impl<'a> ParseStream<'a> {
    pub fn new(input: &'a str) -> ParseStream<'a> {
        ParseStream { cursor: Cell::new(Cursor::new(input)) }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    // TODO: Rename to step_isolated maybe?
    pub fn try_step<R>(&self, parse_fn: impl FnOnce(Cursor<'a>) -> Result<(R, Cursor<'a>)>) -> Result<R> {
        let (node, left) = parse_fn(self.cursor.get())?;
        self.cursor.set(left);
        Ok(node)
    }
 }