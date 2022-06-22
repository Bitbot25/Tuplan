#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod win;

#[cfg(unix)]
pub use unix as sys;
#[cfg(windows)]
pub use win as sys;

fn ceil_pow_2(mut n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n += 1;
    n
}

pub fn round_cap(cap: usize) -> usize {
    let g = sys::granularity();
    let segments = (cap + g - 1) / g;
    let segments = ceil_pow_2(segments);
    segments * g
}

#[cfg(all(not(unix), not(windows)))]
compile_error!("Unsupported platform. Please use unix or windows.");
