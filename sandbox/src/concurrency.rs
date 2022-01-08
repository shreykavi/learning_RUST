use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

fn channel_comms(){
    // multiple producer, single consumer :: channel
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // This would cause err since the channel now owns var
        // println!("val is {}", val);
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // .recv blocks and waits for comms, when tx closes this will error
    // alternatively could use .try_recv() for non blocking
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // can also treat rx as a iterator for all messages in channel
    // iteration ends once tx closes channel
    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn run(){
    // basic_thread_ex();
    channel_comms();
}
