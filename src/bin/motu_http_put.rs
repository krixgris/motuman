use motuman::motu::channel::Channel;
use motuman::motu::{channel::ChannelType, MotuCommand};
use reqwest::{
    blocking::{Client, Response},
    header,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new();

    let motu_commands = vec![
        MotuCommand::Send {
            channel: Channel::new(1, ChannelType::Chan),
            aux_channel: Channel::new(1, ChannelType::Aux),
            value: 1.0,
        },
        MotuCommand::Send {
            channel: Channel::new(2, ChannelType::Chan),
            aux_channel: Channel::new(1, ChannelType::Aux),
            value: 1.0,
        },
        MotuCommand::Volume {
            channel: Channel::new(1, ChannelType::Chan),
            volume: 1.0,
        },
    ];

    // iter through motu_commands and filter out the .hash_map().is_some(), and then create one long string with comma separated key: value pairs
    let payload = "";

    println!("{}", payload);

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // Send the POST request with JSON payload
    let response: Response = client
        .post("http://192.168.1.167/datastore/mix")
        .headers(headers)
        .body(format!("json={}", payload))
        .send()?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Request successful!");
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
