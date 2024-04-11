use std::thread;

fn main() {
    // spawn tho threads that will both execute f  as their main function
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

   t1.join().unwrap();
   t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread is: {id:?}");
}
