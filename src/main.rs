extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest;

use serde_json::Value;

mod faker;
mod types;

use types::APIResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fetch_wave("5842041f4e65fad6a7708b39").await?;

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("current_timestamp {:?}", current_timestamp);

    Ok(())
}

fn human_timestamp(epoch: u64) {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}

#[test]
fn test_faker_loads() -> Result<(), serde_json::Error> {
    let text = faker::fake_response();
    let json: Value = serde_json::from_str(&text)?;
    assert_ne!(json["dog"], Value::Null);
    assert_eq!(json["dog"], "hi");
    Ok(())
}

async fn fetch_document(url: String) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let response = reqwest::get(&url).await?;

    println!("GET {}!", url);
    if response.status().is_success() {
        println!("URL: {:#?}, status: {:#?}", &url, response.status());
    } else {
        println!("Something bad happened...");
        return Err(Box::new(response.status()));
    }

    Ok(response)
}

async fn fetch_wave(spot_id: &str) -> Result<APIResponse, Box<dyn std::error::Error>> {
    let req_url = format!("https://services.surfline.com/kbyg/spots/forecasts/{type}?{params}",
    type = "wave",
    params = format!("spotId={}", &spot_id)
    );

    let resp = fetch_document(req_url.to_string()).await?;
    let text = &resp.text().await?;

    // let text = faker::fake_response();
    let json = serde_json::from_str(&text);

    let p: APIResponse = json.unwrap();
    println!("JSON: {:#?}", p);

    Ok(p)
}
