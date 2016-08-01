extern crate prometheus;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut reg = prometheus::Registry::new("0.0.0.0".to_string(), 6780);
    let counter_arc = Arc::new(Mutex::new(prometheus::Counter::new("http_requests".to_string(), "Counter for HTTP Requests".to_string())));
    reg.register(Arc::downgrade(&counter_arc));
    let interval = Duration::from_millis(500);
    reg.start();
    let counter_mutex = counter_arc.clone();
    loop {
        thread::sleep(interval);
        println!("Counter: {}", counter_mutex.lock().unwrap().increment());
    }
}
