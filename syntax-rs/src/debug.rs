pub trait DebugTap<F> {
    fn debug_tap(self, fun: F) -> Self;
}

impl<T, F> DebugTap<F> for T
where
    F: Fn(&T),
{
    fn debug_tap(self, fun: F) -> Self {
        fun(&self);
        self
    }
}
