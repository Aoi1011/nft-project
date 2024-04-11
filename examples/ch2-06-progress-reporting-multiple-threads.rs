use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_time(t * 25 + i);
                    num_done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
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
