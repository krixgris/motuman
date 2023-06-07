use reqwest::{blocking::{Client, Response}, header};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new();

    // Prepare the JSON payload
    let payload = json!({
        "fader": "1.0"
    });

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());


    // Send the POST request with JSON payload
    let response: Response = client
        .post("http://192.168.1.167/datastore/mix/chan/0/matrix")
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
