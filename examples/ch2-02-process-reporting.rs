use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_time(1);
                num_done.store(i + 1, std::sync::atomic::Ordering::Relaxed);
            }
        });

        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            println!("{n}");
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}

fn process_time(_: usize) {
    thread::sleep(Duration::from_millis(37));
}
