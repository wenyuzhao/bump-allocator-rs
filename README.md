# bump-allocator-rs

A high performance `#[global_allocator]` implementation using the bump-pointer allocation algorithm

[![Latest version](https://img.shields.io/crates/v/bump-allocator.svg)](https://crates.io/crates/bump-allocator)
[![License](https://img.shields.io/crates/l/bump-allocator.svg)](https://github.com/wenyuzhao/bump-allocator-rs/LISENCE)

## Usage

As a rust [custom global allocaor](https://doc.rust-lang.org/beta/std/alloc/trait.GlobalAlloc.html):

```rust
extern crate bump_allocator;

#[global_allocator]
static GLOBAL: bump_allocator::BumpPointer = bump_allocator::BumpPointer;

fn main() {
    // Heap allocations here...
    let _boxed = Box::new(233);
}
```

As a `malloc()` replacement:

```bash
cargo build --release
LD_PRELOAD=target/release/libbump_allocator.so your_program
# e.g.
#   LD_PRELOAD=target/release/libbump_allocator.so ls ~
```
