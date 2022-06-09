pub struct Snapshot(pub usize);

impl From<usize> for Snapshot {
    fn from(index: usize) -> Self {
        Snapshot(index)
    }
}

impl Into<usize> for Snapshot {
    fn into(self) -> usize {
        self.0
    }
}

impl Snapshot {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}
