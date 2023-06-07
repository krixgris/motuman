use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ResponseData {
    fader: f64,
    // Add other fields as needed
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Send the GET request
    let response = client
        .get("http://192.168.1.167/datastore/mix/chan/0/matrix")
        .send()?;

    // let response_data: ResponseData = response.json()?;

    // println!("Name: {:?}", response_data.fader);

    // Parse the JSON response
    let response_body = response.text()?;
    let json_value: Value = serde_json::from_str(&response_body)?;

    // // Query the JSON using keys
    // if let Some(hpf_enable) = json_value.get("hpf/enable") {
    //     println!("hpf/enable: {}", hpf_enable);
    // }

    // if let Some(eq_highshelf_enable) = json_value.get("eq/highshelf/enable") {
    //     println!("eq/highshelf/enable: {}", eq_highshelf_enable);
    // }

    for key in json_value.as_object().unwrap().keys() {
        println!("{}: {}", key, json_value[key]);
    }

    // Query other keys as needed

    Ok(())
}
