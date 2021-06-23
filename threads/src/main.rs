use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || {   //move all variables into the closure env
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); value v can't be dropped because it was moved inside the closure

    handle.join().unwrap();

    message_passing();

}

fn message_passing(){
    let (tx, rx) = mpsc::channel();  //mpsc - multiple producer single consumer

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        println!("val is {}", val);  //Error: once val is send over channel, the channel becomes the owner or val.
    });

    let recived = rx.recv().unwrap();
    println!("Received message is {}", recived);

}