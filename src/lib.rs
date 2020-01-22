#![feature(thread_local)]

extern crate libc;
mod bump_pointer_local;

use std::alloc::{Layout, GlobalAlloc};
use bump_pointer_local::LOCAL_ALLOCATOR;



pub struct BumpPointer;

unsafe impl GlobalAlloc for BumpPointer {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        LOCAL_ALLOCATOR.alloc(layout.size(), layout.align())
    }

    #[inline(always)]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

