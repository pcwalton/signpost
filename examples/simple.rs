// signpost/examples/simple.rs

extern crate signpost;

use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_millis(250));
    signpost::trace_function(1, &[2, 3, 4, 5], || thread::sleep(Duration::from_secs(1)));
    thread::sleep(Duration::from_millis(250));
    signpost::trace(6, &[7, 8, 9, 10]);
    thread::sleep(Duration::from_millis(250));
}

