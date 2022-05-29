#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    left: &'a str,
    generation: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor { left: input, generation: 0 }
    }

    #[inline(always)]
    fn is_eof(&self) -> bool {
        self.left.is_empty()
    }

    #[inline]
    pub fn getc(&mut self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            unsafe { Some(self.getc_unchecked()) }
        }
    }

    #[inline]
    pub unsafe fn getc_unchecked(&mut self) -> char {
        let bytes = self.left.as_bytes();
        self.left = std::mem::transmute(bytes.get_unchecked(1..));
        self.generation += 1;
        bytes[0] as char
    }

    #[inline]
    unsafe fn getc_unchecked_lookahead(&mut self, len: usize) -> char {
        self.left.as_bytes()[len] as char
    }

    pub fn getc_while<R>(&mut self, mut pred: impl FnMut(char) -> bool, user: impl FnOnce(&'a str) -> R) -> R {
        let mut end = 0;
        while self.left.len() > end && pred(unsafe { self.getc_unchecked_lookahead(end) }) {
            end += 1;
        }
        self.generation += end;
        let result = user(unsafe { self.left.get_unchecked(..end) });
        self.left = unsafe { self.left.get_unchecked(end..) };
        result
    }
}

impl<'a> IntoIterator for Cursor<'a> {
    type Item = char;

    type IntoIter = CursorIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CursorIntoIter { c: self }
    }
}

pub struct CursorIntoIter<'a> {
    c: Cursor<'a>
}

impl<'a> Iterator for CursorIntoIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.c.getc()
    }
}