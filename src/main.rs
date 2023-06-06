use clap::Parser;
use std::process;

use motuman::config;
use motuman::motu;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "./motu_config.toml")]
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
    #[arg(short, long)]
    init: bool,
}

fn main() {
    let args = Args::parse();
    println!("Args: {:?}", args);
    let mut motu_commands: Vec<motu::MotuCommand> = vec![];

    let init = args.init;
    if init {
        motu_commands.push(motu::MotuCommand::Init);
    }

    if let Some(true) = args.monitor {
        motu_commands.push(motu::MotuCommand::EnableMonitoring);
    } else if let Some(false) = args.monitor {
        motu_commands.push(motu::MotuCommand::DisableMonitoring);
    }
    let channel = args.channel;
    let send_to_channel = args.aux_channel;
    let send_amount = args.send_amount;
    let volume = args.volume;

    if let (Some(channel), Some(send_to_channel)) = (channel, send_to_channel) {
        motu_commands.push(motu::MotuCommand::Send(
            motu::Channel::new(channel, motu::ChannelType::Chan),
            motu::Channel::new(send_to_channel, motu::ChannelType::Aux),
            send_amount.unwrap_or(0.0),
        ))
    }

    if let (Some(channel), Some(volume)) = (channel, volume) {
        motu_commands.push(motu::MotuCommand::Volume(
            motu::Channel::new(channel, motu::ChannelType::Chan),
            volume,
        ))
    }

    let list_channels = args.list_channels;
    if list_channels {
        motu_commands.push(motu::MotuCommand::PrintSettings);
    }

    let ip_address = {
        match args.ip_address {
            Some(ip) => Some(if ip.contains(':') {
                ip
            } else {
                format!("{}:{}", ip, args.port.unwrap_or(8000))
            }),
            None => None,
        }
    };

    let config = config::Config::build(args.config, ip_address).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(&config.ip_address);

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
