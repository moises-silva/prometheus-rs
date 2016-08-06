// TODO:
// - Add tags to metrics
// - Hide Arc/Mutex?
// - Validate request Accept header (should be compatible with what we support)
// - Validate endpoint /metrics?
// - Extend tiny http to allow getting headers by name
#[macro_use]
extern crate log;

extern crate tiny_http;
extern crate time;
use std::str::FromStr;
use std::ops::Deref;
use std::thread;
use std::sync::{Arc, Mutex};
use tiny_http::{Server, Response, Header, StatusCode};

#[derive(Debug)]
pub struct Counter {
    name: String,
    desc: String,
    value: f64
}

#[derive(Debug)]
pub struct Gauge {
    name: String,
    desc: String,
    value: f64
}

impl Counter {
    pub fn new(name: String, desc: String) -> Counter {
        Counter {
            name: name,
            desc: desc,
            value: 0.0
        }
    }

    pub fn increment(&mut self) -> f64 {
        self.value += 1 as f64;
        self.value()
    }

    pub fn increment_by(&mut self, val: f64) -> f64 {
        self.value += val;
        self.value()
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Gauge {
    pub fn new(name: String, desc: String) -> Gauge {
        Gauge {
            name: name,
            desc: desc,
            value: 0.0
        }
    }

    pub fn set(&mut self, val: f64) -> f64 {
        self.value = val;
        self.value()
    }

    pub fn increment(&mut self) -> f64 {
        self.value += 1 as f64;
        self.value()
    }

    pub fn increment_by(&mut self, val: f64) -> f64 {
        self.value += val;
        self.value()
    }

    pub fn decrement(&mut self) -> f64 {
        self.value -= 1 as f64;
        self.value()
    }

    pub fn decrement_by(&mut self, val: f64) -> f64 {
        self.value -= val;
        self.value()
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn desc(&self) -> String {
        self.desc.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Registry {
    address: String,
    port: u16,
    counters: Vec<Arc<Mutex<Counter>>>,
    gauges: Vec<Arc<Mutex<Gauge>>>
}

impl Registry {
    pub fn new(address: String, port: u16) -> Registry {
        Registry {
            address: address,
            port: port,
            counters: Vec::new(),
            gauges: Vec::new()
        }
    }

    pub fn register_counter(&mut self, counter: Arc<Mutex<Counter>>) {
        self.counters.push(counter)
    }

    pub fn register_gauge(&mut self, gauge: Arc<Mutex<Gauge>>) {
        self.gauges.push(gauge)
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
        info!("Startings metrics http endpoint at addr {}", bindaddr);
        let regref = registry.clone();
        thread::spawn(move || {
            let server = Server::http(bindaddr.as_str()).unwrap();
            loop {
                let request = match server.recv() {
                    Ok(rq) => rq,
                    Err(e) => { error!("error: {}", e); break }
                };
                debug!("Handling metrics request (method={:?}, url: {:?}, headers: {:?})",
                       request.method(), request.url(), request.headers());
                let time = time::now().to_timespec();
                let msnow = (time.sec * 1000) + (time.nsec as i64 / 1000000);
                let mut payload = String::new();
                // Locked
                {
                    let reg = regref.lock().unwrap();
                    for rc in &reg.counters {
                        let counter = rc.lock().unwrap();
                        debug!("{:?}", counter.deref());
                        payload.push_str(&format!("# HELP {} {}\n", counter.name(), counter.desc()));
                        payload.push_str(&format!("{} {} {}\n", counter.name(), counter.value(), msnow));
                    }
                    for rc in &reg.gauges {
                        let gauge = rc.lock().unwrap();
                        debug!("{:?}", gauge.deref());
                        payload.push_str(&format!("# HELP {} {}\n", gauge.name(), gauge.desc()));
                        payload.push_str(&format!("{} {} {}\n", gauge.name(), gauge.value(), msnow));
                    }
                    payload.push('\n');
                    debug!("{}", payload);
                    let headers = vec![Header::from_str("Content-Type: text/plain; version=0.0.4").unwrap()];
                    let rsp = Response::new(StatusCode::from(200), headers, payload.as_bytes(), Some(payload.len()), None);
                    let _ = request.respond(rsp);
                }
            }
        });
    }
}
