use std::{
    cmp::Ordering,
    sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, AtomicU64},
    thread, time::Duration,
};
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    stop_flag();
}

pub fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

pub fn calculate_x() -> u64 {
    1
}

pub fn synchronization() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(
        |s| {
            // A background thread to process all 100 items
            s.spawn(|| {
                for i in 0..100 {
                    process_item(i);
                    num_done.store(i + 1, Relaxed);
                    main_thread.unpark();
                }
            });

            // The main thread shows status updates
            loop {
                let n = num_done.load(Relaxed);
                if n == 100 {break;}
                println!("Working.. {n}/100 done");
                thread::park_timeout(Duration::from_secs(1));
            }
        }
    );

    println!("Done!");
}

pub fn progress_reporting() {
     let num_done = AtomicUsize::new(0);

     thread::scope(|s| {
        // A background thread to process all 100 items
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed)
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {break;}
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
     });

     println!("Done!");
}


pub fn stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work
    let background_thread = thread::spawn(|| {
        while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
            some_work();
        }
    });

    // Use the main thread to listen for user input
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    // Inform the background thread it needs to stop
    STOP.store(true, std::sync::atomic::Ordering::Relaxed);

    // Wait until the background thread finishes
    background_thread.join().unwrap();
}

fn process_item(i: usize) {
    println!("")
}

fn some_work() {
    print!("");
}
