use futures::future::join_all;
use itertools::Itertools;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct UrlData {
    urls: Box<[SiteData]>,
}

#[derive(Deserialize, Debug)]
struct SiteData {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let text = std::fs::read_to_string("./urls.json")?;
    let data: UrlData = serde_json::from_str(&text)?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(5000))
        .build()
        .unwrap();

    // Create an iterator of reqwest futures
    // based on the URLs in the dataset.
    let response_futures = data.urls.iter().map(|site| client.get(&site.url).send());

    // Join up the futures and await them,
    // then print their output.
    join_all(response_futures)
        .await
        .into_iter()
        .filter_map_ok(|r| Some(r.status()))
        .enumerate()
        .for_each(|(i, r)| {
            println!(
                "Pinging {} at {}: {}",
                data.urls[i].name,
                data.urls[i].url,
                // Account for timeout error
                match r {
                    Ok(r) => r.to_string(),
                    Err(e) => format!("Error: {}", e),
                }
            )
        });

    Ok(())
}
