use std::mem;
use std::ptr;
use std::ptr::NonNull;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::memoryapi::FILE_MAP_ALL_ACCESS;
use winapi::um::memoryapi::MapViewOfFileEx;
use winapi::um::memoryapi::UnmapViewOfFile;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::memoryapi::VirtualFree;
use winapi::um::winnt::MEM_RELEASE;
use winapi::um::winnt::MEM_RESERVE;
use winapi::um::winnt::PAGE_NOACCESS;
use winapi::{
    shared::minwindef::DWORD,
    um::{
        memoryapi::{CreateFileMappingW},
        sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
        winnt::PAGE_READWRITE,
    },
};

pub fn granularity() -> usize {
    static mut GRANULARITY: usize = 0;

    unsafe {
        if GRANULARITY == 0 {
            let mut sysinfo: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut sysinfo);
            GRANULARITY = sysinfo.dwAllocationGranularity as usize;
        }
        GRANULARITY
    }
}

pub fn page_size() -> usize {
    static mut PAGE_SIZE: usize = 0;
    unsafe {
        if PAGE_SIZE == 0 {
            let mut sysinfo: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut sysinfo);
            PAGE_SIZE = sysinfo.dwPageSize as usize;
        }
        PAGE_SIZE
    }
}

pub unsafe fn ringbuf_alloc_uninit<T>(cap: usize) -> NonNull<T> {
    println!("cap={}", cap);
    let mapping = CreateFileMappingW(
        INVALID_HANDLE_VALUE,
        ptr::null_mut(),
        PAGE_READWRITE,
        ((cap >> 31) >> 1) as DWORD,
        (cap & 0xffffffff) as DWORD,
        ptr::null(),
    );

    if mapping.is_null() {
        eprintln!("error code={}", GetLastError());
        panic!("Failed to create virtual mapping for ringbuffer.");
    }

    // MapViewOfFileEx(hFileMappingObject, dwDesiredAccess, dwFileOffsetHigh, dwFileOffsetLow, dwNumberOfBytesToMap, lpBaseAddress)

    // Find an address where two mappings can fit.
    let free_addr = VirtualAlloc(ptr::null_mut(), 2*cap, MEM_RESERVE, PAGE_NOACCESS);
    if free_addr.is_null() {
        CloseHandle(mapping);
        panic!("Out of memory while allocating ringbuffer.");
    }

    if VirtualFree(free_addr, 0, MEM_RELEASE) == 0 {
        CloseHandle(mapping);
        panic!("Cannot free tested memory for further use?");
    }

    let base = MapViewOfFileEx(mapping, FILE_MAP_ALL_ACCESS, 0, 0, cap, free_addr);

    if base.is_null() {
        CloseHandle(mapping);
        panic!("Failed to map memory in ringbuffer.");
    }
    assert_eq!(base, free_addr);
    println!("base={:?}", base);

    if MapViewOfFileEx(mapping, FILE_MAP_ALL_ACCESS, 0, 0, cap, base.cast::<T>().add(cap).cast()).is_null() {
        UnmapViewOfFile(base);
        CloseHandle(mapping);
        eprintln!("error code={}", GetLastError());
        panic!("Cannot map second region of memory for ringbuffer.");
    }

    NonNull::new_unchecked(base.cast())
}

#[inline]
pub unsafe fn ringbuf_destroy<T>(base: NonNull<T>, cap: usize) {
    UnmapViewOfFile(base.as_ptr().cast());
    UnmapViewOfFile(base.as_ptr().add(cap).cast());
}
