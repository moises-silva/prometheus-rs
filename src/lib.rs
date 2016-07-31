extern crate tiny_http;
use std::thread;
use tiny_http::{Server, Response};

pub struct Counter {
    name: String,
    desc: String,
    value: i64
}

impl Counter {
    pub fn new(name: String, desc: String) -> Counter {
        Counter {
            name: name,
            desc: desc,
            value: 0
        }
    }
    pub fn increment(&mut self) -> i64 {
        self.value += 1 as i64;
        self.value
    }
}

pub struct Registry<'a> {
    counters: Vec<&'a Counter>

}

impl<'a> Registry<'a> {
    pub fn new() -> Registry<'a> {
        Registry {
            counters: Vec::new()
        }
    }

    pub fn register(&mut self, counter: &'a Counter) {
        self.counters.push(counter)
    }

    pub fn start(&mut self) {
        println!("Startings metrics http endpoint");
        let handle = thread::spawn(|| {
            let server = Server::http("0.0.0.0:6780").unwrap();
            loop {
                let request = match server.recv() {
                    Ok(rq) => rq,
                    Err(e) => { println!("error: {}", e); break }
                };
                println!("Handling metrics request");
                let response = Response::from_string("Prometheus Metrics".to_string());
                let _ = request.respond(response);
            }
        });
        //println!("Thread done: {}", handle.join().unwrap());
    }
}
