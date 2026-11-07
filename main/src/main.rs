// use std::sync::{Arc, Mutex};
// use std::thread;
//
// fn main() {
//     let value = Arc::new(Mutex::new(42));
//
//     let value1 = Arc::clone(&value);
//     let value2 = Arc::clone(&value);
//     let value3 = Arc::clone(&value);
//
//     let t1 = thread::spawn(move || {
//         let mut value = value1.lock().unwrap();
//         *value += 10;
//     });
//
//     let t2 = thread::spawn(move || {
//         let mut value = value2.lock().unwrap();
//         *value += 20;
//     });
//     let t3 = thread::spawn(move || {
//         let mut value = value3.lock().unwrap();
//         *value += 30;
//     });
//
//     println!("{}", *value.lock().unwrap());
//     t1.join().unwrap();
//     println!("{}", *value.lock().unwrap());
//     t2.join().unwrap();
//     println!("{}", *value.lock().unwrap());
//     t3.join().unwrap();
//     println!("{}", *value.lock().unwrap());
// }
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // Consumer thread
    let consumer_pair = Arc::clone(&pair);

    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*consumer_pair;

        let mut ready = lock.lock().unwrap();

        // Wait until the condition becomes true
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }

        println!("Consumer: Data is ready!");
    });

    // Producer thread
    let (lock, cvar) = &*pair;

    thread::sleep(Duration::from_secs(2));

    let mut ready = lock.lock().unwrap();
    *ready = true;

    println!("Producer: Data produced!");

    cvar.notify_one();

    drop(ready);

    consumer.join().unwrap();
}
