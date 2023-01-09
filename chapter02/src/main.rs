use std::{sync::atomic::AtomicI32, cmp::Ordering};

impl AtomicI32 {
    pub fn load(&self, ordering: Ordering) -> i32;
    pub fn store(&self, value: i32, ordering: Ordering);
}

fn main() {
    println!("Hello, world!");
}
