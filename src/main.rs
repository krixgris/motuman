// use std::fs::File;
// use std::io::Write;
use std::process;
use std::sync::mpsc::channel;
// use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;

use motuman::config;
use motuman::motu;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "./config.toml")]
    config: String,
    #[arg(short, long)]
    monitor: Option<bool>,
    #[arg(short, long)]
    channel: Option<i32>,
    #[arg(short, long)]
    volume: Option<f32>,
    #[arg(long = "ip")]
    ip_address: Option<String>,
    #[arg(long = "port", default_value = "8000")]
    port: Option<u16>,
    #[arg(short, long)]
    list_channels: bool,
    #[arg(short, long)]
    aux_channel: Option<i32>,
    #[arg(short, long)]
    send_amount: Option<f32>,
}

fn main() {
    let args = Args::parse();
    println!("Args: {:?}", args);
    let mut motu_commands: Vec<motu::MotuCommand> = vec![];

    let monitor = args.monitor;
    match monitor {
        Some(true) => motu_commands.push(motu::MotuCommand::EnableMonitoring),
        Some(false) => motu_commands.push(motu::MotuCommand::DisableMonitoring),
        None => println!("No monitor mode"),
    }
    let channel = args.channel;
    let send_to_channel = args.aux_channel;
    let send_amount = args.send_amount;
    let volume = args.volume;
    match send_to_channel {
        Some(send_to_channel) => match channel {
            Some(channel) => motu_commands.push(motu::MotuCommand::Send(
                Some(motu::Channel::new(channel)),
                Some(motu::Channel::new(send_to_channel)),
                send_amount.unwrap_or(0.0),
            )),
            None => println!("No channel"),
        },
        None => println!("No send to channel"),
    }

    match volume {
        Some(volume) => motu_commands.push(motu::MotuCommand::Volume(
            Some(motu::Channel::new(args.channel.unwrap_or(0))),
            volume,
        )),
        None => println!("No volume"),
    }

    let list_channels = args.list_channels;
    if list_channels {
        motu_commands.push(motu::MotuCommand::PrintSettings);
    }

    // motu_commands.push(motu::MotuCommand::Volume(
    //     Some(motu::Channel::new(0)),
    //     1.0,
    // ));
    // motu_commands.push(motu::MotuCommand::Send(
    //     Some(motu::Channel::new(0)),
    //     Some(motu::Channel::new(0)),
    //     0.0,
    // ));
    // motu_commands.push(motu::MotuCommand::Send(
    //     Some(motu::Channel::new(0)),
    //     Some(motu::Channel::new(2)),
    //     0.0,
    // ));

    let ip_address = {
        match args.ip_address {
            Some(ip) => Some(if ip.contains(':') {
                ip
            } else {
                format!("{}:{}", ip, args.port.unwrap_or(8002))
            }),
            None => None,
        }
    };

    let config = config::Config::build(args.config, ip_address).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(&config.ip_address);

    let motu = motu::Motu::new(&config.ip_address, &config).unwrap();

    if let Err(e) = motu.run(motu_commands) {
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
