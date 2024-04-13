use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let a = thread::spawn(|| {
        A.store(true, Ordering::SeqCst);
        if !B.load(Ordering::SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
        if !A.load(Ordering::SeqCst) {
            unsafe { S.push('!') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();

    println!("{:?}", unsafe { S.clone() });
}
