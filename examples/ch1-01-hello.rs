use std::thread;

fn main() {
    // spawn tho threads that will both execute f  as their main function
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread is: {id:?}");
}
