extern crate prometheus;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, Prometheus!");
    let mut reg = prometheus::Registry::new();
    let mut counter = prometheus::Counter::new("http_requests".to_string(), "Counter for HTTP Requests".to_string());
    reg.start();
    let interval = Duration::from_millis(500);
    loop {
        thread::sleep(interval);
        println!("Counter value: {}", counter.increment());
    }
}
