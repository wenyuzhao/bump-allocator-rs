use super::bump_pointer_local::LOCAL_ALLOCATOR;
use super::mem_zero;



#[no_mangle]
unsafe extern fn malloc(size: usize) -> *mut u8 {
    LOCAL_ALLOCATOR.alloc(size, size.next_power_of_two())
}

#[no_mangle]
unsafe extern fn calloc(num: usize, size: usize) -> *mut u8 {
    let ptr = LOCAL_ALLOCATOR.alloc(size * num, size.next_power_of_two());
    mem_zero(ptr, size);
    ptr
}

#[no_mangle]
unsafe extern fn free(_ptr: *mut u8) {}

#[no_mangle]
unsafe extern fn realloc(ptr: *mut u8, bytes: usize) -> *mut u8 {
    let new_ptr = malloc(bytes);

    if !ptr.is_null() && !new_ptr.is_null() {
        ::std::ptr::copy(ptr, new_ptr, bytes);
        free(ptr);
    }

    return new_ptr;
}

