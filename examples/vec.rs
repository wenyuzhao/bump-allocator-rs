extern crate bump_allocator;

#[global_allocator]
static GLOBAL: bump_allocator::BumpPointer = bump_allocator::BumpPointer;

fn main() {
    let sequence = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    let reversed_sequence = {
        let mut cloned = sequence.clone();
        cloned.reverse(); 
        cloned
    };
    assert_eq!(reversed_sequence, vec![34, 21, 13, 8, 5, 3, 2, 1, 1, 0]);
    println!("Sequence: {:?}", sequence);
    println!("Reversed: {:?}", reversed_sequence);
}

