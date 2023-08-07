use axum::{
    extract::Path,
    response::Json,
    routing::post,
    Router,
};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct UrlData {
    urls: Vec<SiteData>,
}

#[derive(Deserialize, Debug)]
struct SiteData {
    name: String,
    url: String,
}

#[derive(Serialize, Debug)]
struct SiteResponse {
    status_code: String,
    responses: Vec<String>,
}

async fn ping(Path(status_code): Path<String>, Json(data): Json<UrlData>) -> Json<Value> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(5000))
        .build()
        .unwrap();

    // Create an iterator of reqwest futures
    // based on the URLs in the dataset.
    let response_futures = data
        .urls
        .iter()
        .map(|site| async { client.get(&site.url).send().await });

    // Join up the futures and await them,
    // then print their output.
    let mut responses = join_all(response_futures).await.into_iter();

    if responses.all(|r| r.is_ok()) {
        let responses = responses
            .enumerate()
            .map(|(i, r)| match r {
                Ok(r) => data.urls[i].name.to_string() + ": " + r.status().as_str(),
                Err(e) => format!("Error: {}", e),
            })
            .collect::<Vec<String>>();

        Json(json!(SiteResponse {
            status_code,
            responses
        }))
    } else {
        Json(json!({
            "error": "One or more requests failed."
        }))
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping/:status_code", post(ping));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
