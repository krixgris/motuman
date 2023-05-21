mod osc;
mod motu;


use std::env;
use std::process;

use motuman::Config;
// use motuman::motu::Motu;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let motu = motu::Motu::new("127.0.0.1", 8000).unwrap();

    if let Err(e) = motu.run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}