#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;

#[macro_use]
extern crate log;

extern crate prometheus;
extern crate sys_info;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sys_info::loadavg;

use slog::drain::IntoLogger;

fn main() {
    let root = slog_term::stderr().into_logger(o!());
    slog_stdlog::set_logger(root).unwrap();

    let reg_arc = Arc::new(Mutex::new(prometheus::Registry::new("0.0.0.0".to_string(), 6780)));
    let counter_arc = Arc::new(Mutex::new(prometheus::Counter::new("sleep_count".to_string(),
                                                                   "Number of times we sleep".to_string())));
    let gauge_arc = Arc::new(Mutex::new(prometheus::Gauge::new("system_memory".to_string(),
                                                               "System memory".to_string())));
    {
        let mut regl = reg_arc.lock().unwrap();
        regl.register_counter(counter_arc.clone());
        regl.register_gauge(gauge_arc.clone());
    }
    let interval = Duration::from_millis(100);
    prometheus::Registry::start(&reg_arc);
    let counter_mutex = counter_arc.clone();
    let gauge_mutex = gauge_arc.clone();
    info!("Starting loop");
    for _ in 0..100 {
        counter_mutex.lock().unwrap().increment();
        let load = loadavg().unwrap();
        gauge_mutex.lock().unwrap().set(load.one);
        thread::sleep(interval);
    }
    debug!("Terminating!");
    prometheus::Registry::stop(reg_arc);
}
