use std::process;
use motuman::{config, motu, args::Args};
use std::env;

fn main() {
    // Get the current working directory and print it to the console
    if let Ok(cwd) = env::current_dir() {
        println!("Current working directory: {:?}", cwd);
    } else {
        eprintln!("Error getting current working directory");
    }

    // Initialize the command line arguments
    let args = Args::init();
    println!("Args: {:?}", args);
  
    // Get the configuration file name, IP address, and MOTU commands from the command line arguments
    let config_file_name = args.config_file_name();
    let ip_address = args.ip_address();
    let motu_commands = args.motu_commands();
    dbg!(&config_file_name);
    dbg!(&ip_address);

    // Build the configuration object
    let config = config::Config::build(config_file_name, ip_address).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(&config.ip_address);

    // Create a new MOTU object and run the specified commands
    match motu::Motu::new(&config.ip_address, &config) {
        Ok(motu) => {
            if let Err(e) = motu.run(motu_commands) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error creating Motu object: {e}");
            process::exit(1);
        }
    }
}
