#![feature(thread_local)]

extern crate libc;
mod bump_pointer_local;
mod malloc_api;

use std::alloc::{Layout, GlobalAlloc};
use bump_pointer_local::LOCAL_ALLOCATOR;



pub struct BumpPointer;

unsafe impl GlobalAlloc for BumpPointer {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (bytes, align) = (layout.size(), layout.align());
        let ptr = LOCAL_ALLOCATOR.alloc(bytes, align);
        mem_zero(ptr, bytes);
        ptr
    }

    #[inline(always)]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}


#[inline(always)] 
unsafe fn mem_zero(ptr: *mut u8, bytes: usize) {
    let mut cursor = ptr;
    let limit = ptr.add(bytes);
    while cursor < limit {
        cursor.write(0);
        cursor = cursor.add(1);
    }
}
