use std::io::Read;
use std::sync::{Mutex, Arc};
use std::sync::atomic::Ordering::Relaxed;
use std::time::Instant;
use std::{
    cmp::Ordering,
    sync::atomic::{AtomicBool, AtomicI32, AtomicU64, AtomicUsize},
    thread,
    time::Duration,
};

fn main() {
    statistics();
}

pub fn statistics() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        // The main thread shows status updates, every seconds
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {break;}
            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                println!("Working.. {n}/100 done, {:?} average, {:?} peak", total_time/n as u32, max_time);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!")
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

    thread::scope(|s| {
        // A background thread to process all 100 items
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Assuming this takes some time.
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); // Wake up the main thread.
            }
        });

        // The main thread shows status updates
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

pub fn progress_reporting() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 items.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }
        

        // The main thread shows status updates, every second
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {break;}
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(_i: usize) {
    print!("");
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



fn some_work() {
    print!("");
}
