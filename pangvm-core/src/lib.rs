#![feature(allocator_api, str_internals)]

use std::ops::{Deref, DerefMut};

pub mod compiler;
pub mod cursor;
pub mod parse;
pub mod spec;
pub mod macros;
pub mod guard;

#[derive(Debug, Default, Hash, Copy, Clone)]
pub struct Span {
    pub begin: usize,
    pub end: usize,
}

pub trait Spanned {
    fn span(&self) -> Span;
    fn span_ref_mut(&mut self) -> &mut Span;
}

pub type Result<T> = std::result::Result<T, &'static str>;