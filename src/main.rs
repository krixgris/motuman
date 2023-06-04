use std::env;
// use std::fs::File;
// use std::io::Write;
use std::process;
// use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;

use motuman::config;
use motuman::motu;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    let config = config::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(&config.ip_address);

    let motu = motu::Motu::new(&config.ip_address, &config).unwrap();

    if let Err(e) = motu.run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // // Get the current time as a UNIX timestamp
    // let timestamp = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs();

    // // Create or open a file in the same directory as the executable
    // let mut file = File::create("output.txt").unwrap_or_else(|err| {
    //     eprintln!("Problem creating file: {err}");
    //     process::exit(1);
    // });

    // // Write the timestamp to the file
    // writeln!(file, "{}", timestamp).unwrap_or_else(|err| {
    //     eprintln!("Problem writing to file: {err}");
    //     process::exit(1);
    // });
}
