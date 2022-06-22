use syntax_rs::ringbuf;

fn main() {
    let cap = ringbuf::round_cap(40);
    unsafe { ringbuf::sys::ringbuf_alloc_uninit::<u8>(cap); }
}
