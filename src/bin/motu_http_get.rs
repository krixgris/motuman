use reqwest::{
    blocking::Client,
    blocking::Response,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ResponseData {
    fader: f64,
    // Add other fields as needed
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Create the headers
    let mut headers = HeaderMap::new();

    // Send the GET request
    let response = client
        .get("http://192.168.1.167/datastore/mix")
        .headers(headers.clone())
        .send()?;

    if let Some(etag) = response.headers().get("ETag") {
        // Store the ETag value for subsequent requests
        let etag_value = dbg!(etag.to_str()?.to_owned());

        // Add the ETag to the headers for future GET requests
        headers.insert("If-None-Match", HeaderValue::from_str(&etag_value)?);

        // Send subsequent GET requests with the ETag
        let response: Response = client
            .get("http://192.168.1.167/datastore/mix")
            .headers(headers)
            .send()?;

        // Process the response as desired
        if response.status().is_success() {
            let etag = response.headers().get("ETag").unwrap().clone();
            let body = response.text()?;
            println!("New ETag: {}", etag.to_str()?);
            let json_value: Value = serde_json::from_str(&body)?;
            for key in json_value.as_object().unwrap().keys() {
                println!("{}: {}", key, json_value[key]);
            }
            
            println!("Response body: {}", body);
        } else {
            println!("Request failed with status: {}", response.status());
        }
    } else {
        println!("ETag not found in the response headers");
    }
    // Parse the JSON response
    // let response_body = response.text()?;
    // let json_value: Value = serde_json::from_str(&response_body)?;

    // // Query the JSON using keys
    // if let Some(hpf_enable) = json_value.get("hpf/enable") {
    //     println!("hpf/enable: {}", hpf_enable);
    // }

    // if let Some(eq_highshelf_enable) = json_value.get("eq/highshelf/enable") {
    //     println!("eq/highshelf/enable: {}", eq_highshelf_enable);
    // }

    // for key in json_value.as_object().unwrap().keys() {
    //     println!("{}: {}", key, json_value[key]);
    // }

    // Query other keys as needed

    Ok(())
}
