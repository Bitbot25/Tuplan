// TODO: Make different presets for languages with a preset-<language> feature.
// TODO: Add no_std feature.

pub mod compiler;
pub mod cursor;
pub mod macros;
pub mod parse;
pub mod snapshot;
pub mod spec;

#[cfg(feature = "span")]
#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Span {
    pub begin: usize,
    pub end: usize,
}

#[cfg(feature = "span")]
pub trait Spanned {
    fn span(&self) -> Span;
    fn span_ref_mut(&mut self) -> &mut Span;
}

pub type Result<T> = std::result::Result<T, &'static str>;
