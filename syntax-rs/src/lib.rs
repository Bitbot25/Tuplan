// TODO: Make different presets for languages with a preset-<language> feature.
// TODO: Add no_std feature.
// TODO: Add some benchmarks.

use parse::{Parse, ParseStream};

pub mod compiler;
pub mod cursor;
pub mod debug;
pub mod macros;
pub mod parse;
pub mod ringbuf;
pub mod snapshot;
pub mod spec;
mod utf8;

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

// TODO: Make the new method private.
#[inline]
pub fn parse_stream(input: &str) -> ParseStream {
    ParseStream::new(input)
}

#[inline]
pub fn parse<T: Parse>(input: &str) -> Result<T> {
    T::parse(&mut parse_stream(input))
}

/// Parses until the stream is empty or there are only whitespaces left.
pub fn exhaustive_parse<T: Parse>(input: &str) -> Result<Vec<T>> {
    let mut stream = parse_stream(input);
    let mut results = Vec::new();
    while !stream.is_empty() && !stream.is_only_whitespaces() {
        results.push(stream.parse::<T>()?);
    }
    Ok(results)
}
