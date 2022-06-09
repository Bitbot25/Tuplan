pub struct Snapshot(pub usize);

impl From<usize> for Snapshot {
    fn from(index: usize) -> Self {
        Snapshot(index)
    }
}

impl From<Snapshot> for usize {
    fn from(snap: Snapshot) -> Self {
        snap.0
    }
}

impl Snapshot {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}
