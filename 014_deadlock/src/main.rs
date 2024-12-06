use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let d1 = Arc::new(Mutex::new(0));
    let d2 = Arc::new(Mutex::new(0));

    let d1_c = Arc::clone(&d1);
    let d2_c = Arc::clone(&d2);
    let h1 = thread::spawn(move || {
        let d1 = d1_c.lock().unwrap();
        thread::sleep(Duration::from_nanos(1));
        let d2 = d2_c.lock().unwrap();
        println!("t1: {}-{}", d1, d2)
    });

    let h2 = thread::spawn(move || {
        let d2 = d2.lock().unwrap();
        thread::sleep(Duration::from_nanos(1));
        let d1 = d1.lock().unwrap();
        println!("t2: {}-{}", d1, d2)
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
