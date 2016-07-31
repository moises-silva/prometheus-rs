extern crate tiny_http;
use tiny_http::{Server, Response};

pub struct Counter {
    name: String,
    desc: String,
    value: i64
}

impl Counter {
    pub fn increment(&mut self) -> i64 {
        self.value + 1
    }
}

pub struct Registry {
    counters: Vec<Counter>
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            counters: Vec::new()
        }
    }

    pub fn counter(&mut self, name: String, desc: String) -> &mut Counter {
        let c = Counter {
            name: name,
            desc: desc,
            value: 0
        };
        self.counters.push(c);
        self.counters.last_mut().unwrap()
    }

    pub fn start(&self) {
        println!("Startings metrics http endpoint");
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
    }
}

