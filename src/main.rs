extern crate prometheus;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let reg_arc = Arc::new(Mutex::new(prometheus::Registry::new("0.0.0.0".to_string(), 6780)));
    let counter_arc = Arc::new(Mutex::new(prometheus::Counter::new("http_requests".to_string(),
                                                                   "Counter for HTTP Requests".to_string())));
    {
        reg_arc.lock().unwrap().register(counter_arc.clone());
    }
    let interval = Duration::from_millis(500);
    prometheus::Registry::start(&reg_arc);
    let counter_mutex = counter_arc.clone();
    loop {
        thread::sleep(interval);
        counter_mutex.lock().unwrap().increment();
    }
}
