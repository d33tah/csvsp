extern crate csvsp;

use std::env;
use std::process;

fn main() {
    if let Err(e) = csvsp::run(env::args().collect()) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
