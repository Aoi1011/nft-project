use std::{sync::atomic::AtomicUsize, thread, time::Duration};

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_time(1);
                num_done.store(i + 1, std::sync::atomic::Ordering::Relaxed);
                main_thread.unpark()
            }
        });

        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");

            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done");
}

fn process_time(_: usize) {
    thread::sleep(Duration::from_millis(37));
}
