extern crate prometheus;

fn main() {
    println!("Hello, Prometheus!");
    let mut reg = prometheus::Registry::new();
    {
        let counter = reg.counter("http_requests".to_string(), "Counter for HTTP Requests".to_string());
        counter.increment();
    }
    reg.start()
}
