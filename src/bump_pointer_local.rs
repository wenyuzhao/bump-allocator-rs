use libc;



#[thread_local]
pub static mut LOCAL_ALLOCATOR: BumpPointerLocal = BumpPointerLocal::new();

pub struct BumpPointerLocal {
    cursor: *mut u8,
    limit: *mut u8,
}

impl BumpPointerLocal {
    const LOG_BLOCK_SIZE: usize = 15;
    const BLOCK_SIZE: usize = 1 << Self::LOG_BLOCK_SIZE; // 32KB
    const BLOCK_MASK: usize = Self::BLOCK_SIZE - 1;

    pub const fn new() -> Self {
        Self { cursor: 0 as _, limit: 0 as _ }
    }

    #[inline(always)]
    fn align_allocation(cursor: *mut u8, align: usize) -> *mut u8 {
        debug_assert!(align.is_power_of_two());
        let mask = align - 1;
        (((cursor as usize) + mask) & !mask) as _
    }

    #[inline(always)]
    pub fn alloc(&mut self, bytes: usize, align: usize) -> *mut u8 {
        debug_assert!(align != 0);
        let start = Self::align_allocation(self.cursor, align);
        let new_cursor = unsafe { start.add(bytes) };
        if new_cursor <= self.limit {
            self.cursor = new_cursor;
            start
        } else {
            self.alloc_slow(bytes, align)
        }
    }

    #[inline(always)]
    fn alloc_slow_inline(&mut self, bytes: usize, align: usize) -> *mut u8 {
        let size = (bytes + Self::BLOCK_MASK) & !Self::BLOCK_MASK;
        self.cursor = unsafe { libc::memalign(Self::BLOCK_SIZE, size) as _ };
        assert!(self.cursor != 0 as _);
        self.limit = unsafe { self.cursor.add(size) };
        self.alloc(bytes, align)
    }

    #[inline(never)]
    fn alloc_slow(&mut self, bytes: usize, align: usize) -> *mut u8 {
        self.alloc_slow_inline(bytes, align)
    }
}
