use std::sync::Once;

use libc::{sysconf, _SC_PAGESIZE};

#[inline]
pub fn granularity() -> usize {
    page_size()
}

pub fn page_size() -> usize {
    static mut PAGE_SIZE: usize = 0;
    // SAFETY: It's not like the granularity is gonna change while we are running the program, so data races won't be a problem.
    unsafe {
        if PAGE_SIZE == 0 {
            PAGE_SIZE = sysconf(_SC_PAGESIZE) as usize;
        }

        PAGE_SIZE
    }
}