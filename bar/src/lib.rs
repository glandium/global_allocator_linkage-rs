use core::alloc::{GlobalAlloc, Layout};
use std::alloc::System;

#[no_mangle]
pub extern "C" fn bar() -> *const usize {
    Box::leak(Box::new(42))
}

struct Heap;

#[global_allocator]
static HEAP: Heap = Heap;

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::write(1, "bar\n".as_ptr() as *const libc::c_void, 4);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}
