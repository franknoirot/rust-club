use serde::{Deserialize, Serialize};
use std::time::Duration;

const FILE_PATH: &'static str = "./urls.json";
const SERVER_URL: &'static str = "http://localhost:3000/ping/200";

#[derive(Deserialize, Debug, Serialize)]
struct UrlData {
    urls: Vec<SiteData>,
}

#[derive(Deserialize, Debug, Serialize)]
struct SiteData {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let text = std::fs::read_to_string(FILE_PATH)?;
    let data: UrlData = serde_json::from_str(&text)?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(5000))
        .build()
        .unwrap();

    // Create an iterator of reqwest futures
    // send a request to the server
    let result = client.post(SERVER_URL)
        .json(&data)
        .send()
        .await;

    match result {
        Ok(response) => {
            let text = response.text().await.unwrap();
            println!("{}", text);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
