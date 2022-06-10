use std::fmt::Debug;

use crate::utf8;

// FIXME: Is this a good and fast implementation? We should probably change it to use an implementation of peeking like the Peekable iterator.
// TODO: Add a new feature called u8_char_processing that will disable iterating over unicode codepoints and just iterate over the bytes.
// TODO: Add a new feature called pre_char_processing that will read all the unicode chars before actually parsing.
// TODO: Add a new feature called dynamic_char_processing that will dynamically read the chars just like we do here. vvvvvvvvvv
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    slice: &'a str,
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(slice: &'a str) -> Cursor<'a> {
        Cursor { slice, index: 0 }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.index >= self.slice.len()
    }

    #[inline]
    unsafe fn next_code_point_u32(&mut self) -> Option<u32> {
        utf8::next_code_point(self.slice.as_bytes(), &mut self.index)
    }

    // TODO: Change this to use a Peekable-like implementation.
    #[inline]
    unsafe fn peek_code_point_u32(&self) -> Option<u32> {
        utf8::peek_code_point(self.slice.as_bytes(), self.index)
    }

    // TODO: Do something like the above TODO and move this into another smaller struct. We will then handle warnings about continuation characters here.
    #[inline]
    pub fn peek0(&self) -> Option<char> {
        unsafe {
            self.peek_code_point_u32()
                .map(|val| char::from_u32_unchecked(val))
        }
    }

    #[inline]
    pub fn peek_n(&self, n: usize) -> Option<&'a str> {
        let begin = self.index;
        let mut virtual_index = self.index;
        for _ in 0..n {
            unsafe {
                utf8::next_code_point(self.slice.as_bytes(), &mut virtual_index)?;
            }
        }
        Some(&self.slice[begin..virtual_index])
    }

    pub fn consume(&mut self, target: char) -> bool {
        let mut virtual_index = self.index;
        let c = unsafe { utf8::next_code_point(self.slice.as_bytes(), &mut virtual_index) };
        match c {
            Some(c) if unsafe { char::from_u32_unchecked(c) } == target => {
                self.index = virtual_index;
                true
            }
            _ => false,
        }
    }

    #[inline]
    pub fn advance(&mut self) -> Option<char> {
        unsafe { Some(char::from_u32_unchecked(self.next_code_point_u32()?)) }
    }

    pub fn advance_n(&mut self, n: usize) -> Option<&'a str> {
        let begin = self.index;
        for _ in 0..n {
            unsafe {
                utf8::next_code_point(self.slice.as_bytes(), &mut self.index)?;
            }
        }
        Some(&self.slice[begin..self.index])
    }

    pub fn advance_while(&mut self, mut pred: impl FnMut(char) -> bool) -> &'a str {
        let begin = self.index;

        let mut next_codepoint_index = self.index;
        loop {
            unsafe {
                match utf8::next_code_point(self.slice.as_bytes(), &mut next_codepoint_index) {
                    Some(code_point) if pred(char::from_u32_unchecked(code_point)) => {
                        self.index = next_codepoint_index
                    }
                    _ => break,
                }
            }
        }
        &self.slice[begin..self.index]
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn iter<'c>(&'c self) -> Iter<'a, 'c> {
        Iter(self.index, self)
    }
}

impl<'a> Debug for Cursor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct LimitDotDot<'a>(&'a str, usize);

        impl<'a> Debug for LimitDotDot<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0.len() > self.1 {
                    write!(
                        f,
                        "{:?} and {} more..",
                        &self.0[..self.1],
                        self.0.len() - self.1
                    )
                } else {
                    write!(f, "{:?}", self.0)
                }
            }
        }

        let slice = &self.slice[self.index..];
        f.debug_tuple("Cursor")
            .field(&LimitDotDot(slice, 16))
            .finish()
    }
}

pub struct Iter<'a, 'b>(usize, &'b Cursor<'a>);

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            utf8::next_code_point(self.1.slice.as_bytes(), &mut self.0)
                .map(|v| char::from_u32_unchecked(v))
        }
    }
}
