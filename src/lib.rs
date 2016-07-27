extern crate tiny_http;
use tiny_http::{Server, Response};

pub fn handler() {
    println!("Handling metrics request!");
    let server = Server::http("0.0.0.0:6780").unwrap();
    loop {
        let request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => { println!("error: {}", e); break }
        };
        let response = Response::from_string("Prometheus Metrics".to_string());
        let _ = request.respond(response);
    }
}


