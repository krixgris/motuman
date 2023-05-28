use std::env;
use std::process;

use motuman::config;
use motuman::motu;

fn main() {
    let config = config::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(&config.ip_address);

    let motu = motu::Motu::new(&config.ip_address).unwrap();

    if let Err(e) = motu.run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
