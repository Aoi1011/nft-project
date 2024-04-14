use std::{time::Instant, sync::atomic::{AtomicU64, Ordering}, hint::black_box};

static A: AtomicU64 = AtomicU64::new(0);

fn main() {
    black_box(&A);
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Ordering::Relaxed));
    }

    println!("{:?}", start.elapsed());
}

pub fn add_ten(num: &mut i32) {
    *num += 10;
}
