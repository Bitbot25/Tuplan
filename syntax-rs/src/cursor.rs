// FIXME: This is slow to construct. Maybe implement utf8.rs so that we can get the next unicode code point? It would be alot faster.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    slice: &'a [char],
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(slice: &'a [char]) -> Cursor<'a> {
        Cursor { slice, index: 0 }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.index >= self.slice.len()
    }

    #[inline]
    pub fn peek0(&self) -> Option<char> {
        self.slice.get(self.index).copied()
    }

    #[inline]
    pub fn peek_range(&self, n: usize) -> Option<&'a [char]> {
        self.slice.get(self.index..n)
    }

    pub fn consume(&mut self, target: char) -> bool {
        match self.peek0() {
            Some(c) => c == target,
            _ => false,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.slice.get(self.index).copied();
        if c.is_some() {
            self.index += 1;
        }
        c
    }

    pub fn advance_n(&mut self, n: usize) -> Option<&'a [char]> {
        let slice = self.slice.get(self.index..n);
        if slice.is_some() {
            self.index += n;
        }
        slice
    }

    pub fn advance_while(&mut self, mut pred: impl FnMut(char) -> bool) -> &'a [char] {
        let begin = self.index;
        while !self.is_empty() && pred(self.peek0().unwrap()) {
            self.index += 1;
        }
        &self.slice[begin..self.index]
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn iter<'c>(&'c self) -> Iter<'a, 'c> {
        Iter(0, self)
    }
}

pub struct Iter<'a, 'b>(usize, &'b Cursor<'a>);

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.1.slice.get(self.0 + self.1.index).copied();
        self.0 += 1;
        c
    }
}
