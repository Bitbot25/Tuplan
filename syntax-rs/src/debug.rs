#[cfg(feature = "debug")]
pub trait DebugTap<F> {
    fn debug_tap(self, fun: F) -> Self;
}

#[cfg(feature = "debug")]
impl<T, F> DebugTap<F> for T
where
    F: Fn(&T),
{
    fn debug_tap(self, fun: F) -> Self {
        fun(&self);
        self
    }
}
