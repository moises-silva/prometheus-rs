extern crate tiny_http;
use std::thread;
use std::sync::{Weak, Mutex};
use std::fmt;
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
        self.value()
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

impl fmt::Debug for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Counter {{ name: {}, value: {}}}", self.name, self.value)
    }
}

pub struct Registry {
    counters: Vec<Weak<Mutex<Counter>>>
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            counters: Vec::new()
        }
    }

    pub fn register(&mut self, counter: Weak<Mutex<Counter>>) {
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
