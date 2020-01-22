# bump-allocator-rs

A high performance `#[global_allocator]` implementation using the bump-pointer allocation algorithm

## Usage

```rust
extern crate bump_allocator;

#[global_allocator]
static GLOBAL: bump_allocator::BumpPointer = bump_allocator::BumpPointer;

fn main() {
    // Heap allocations here...
    let _boxed = Box::new(233);
}
```
