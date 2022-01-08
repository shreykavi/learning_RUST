use std::thread;
use std::time::Duration;

fn basic_thread_ex(){
    let v = vec![1, 2, 3];

    // Spawns new thread that runs the provided closure
    // the move keyword here makes sure thread takes ownership
    // of vals so that they exist 
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // sleeping a thread provides a gap in run so other threads can run
            thread::sleep(Duration::from_millis(1));

            println!("Here's a vector: {:?}", v);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // We cant do this because v is owned by the thread
    // drop(v); // oh no!

    // spawns return a JoinHandle; the following waits for all threads to finish
    // before continuing
    handle.join().unwrap();
}

pub fn run(){
    basic_thread_ex();
}
