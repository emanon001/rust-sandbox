use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let v: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();
    for a in 1..=5 {
        let v = Arc::clone(&v);
        let thread = spawn(move || {
            let mut mutex = v.lock().unwrap();
            mutex.push(a);
        });
        threads.push(thread);
    }

    let _ = threads.into_iter().for_each(|t| t.join().unwrap());
    println!("{:?}", v.lock().unwrap());
}
