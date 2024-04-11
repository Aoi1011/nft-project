use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(false, Ordering::Relaxed);
    background_thread.join().unwrap();
}

fn some_work() {
    thread::sleep(Duration::from_millis(100));
}
