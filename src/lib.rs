extern crate tiny_http;
use std::thread;
use std::sync::{Arc, Mutex};
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

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Debug for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Counter {{ name: {}, value: {}}}", self.name, self.value)
    }
}

pub struct Registry {
    address: String,
    port: u16,
    counters: Vec<Arc<Mutex<Counter>>>
}

impl Registry {
    pub fn new(address: String, port: u16) -> Registry {
        Registry {
            address: address,
            port: port,
            counters: Vec::new()
        }
    }

    pub fn register(&mut self, counter: Arc<Mutex<Counter>>) {
        self.counters.push(counter)
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn start(registry: &Arc<Mutex<Registry>>) {
        let bindaddr;
        {
            let reg = registry.lock().unwrap();
            bindaddr = format!("{}:{}", reg.address(), reg.port());
        }
        println!("Startings metrics http endpoint at addr {}", bindaddr);
        let regref = registry.clone();
        thread::spawn(move || {
            let server = Server::http(bindaddr.as_str()).unwrap();
            loop {
                let request = match server.recv() {
                    Ok(rq) => rq,
                    Err(e) => { println!("error: {}", e); break }
                };
                {
                    let reg = regref.lock().unwrap();
                    println!("Handling metrics request");
                    for rc in &reg.counters {
                        let counter = rc.lock().unwrap();
                        println!("{} {} ({})", counter.name(), counter.value(), counter.desc());
                    }
                }
                let response = Response::from_string("Prometheus Metrics".to_string());
                let _ = request.respond(response);
            }
        });
        //println!("Thread done: {}", handle.join().unwrap());
    }
}
