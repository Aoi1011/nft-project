use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
    time::Duration,
};

static mut TEMP_DATA: u64 = 0;
static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        unsafe { TEMP_DATA = 123 };
        READY.store(true, Ordering::Relaxed);
    });

    while !READY.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }

    println!("{}", unsafe { TEMP_DATA });
}

pub fn release_acquire() {
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });

    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_secs(1));
        println!("waiting...");
    }

    println!("{}", DATA.load(Ordering::Relaxed));
}
