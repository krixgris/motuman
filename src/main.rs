mod osc;

use std::env;
use std::process;

use motuman::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = motuman::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let client = osc::OscClient::new("192.168.1.43:8000").unwrap();
    let result = client.send("/test/whatelsegoesinhere");
    if let Err(e) = result {
        println!("Error sending OSC message: {}", e);
    }

    // let message = osc_packet::Message {
    //     address: "/test".to_string(),
    //     arguments: Some(vec![osc_packet::OscType::Int(42)]),
    // };
    // let bytes = message.to_packet().unwrap();
    // let result = client.send(&bytes);
    // println!("{:?}", result);
}
