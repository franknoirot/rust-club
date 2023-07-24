#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let resp = client.get("https://api.kittycad.io");
    let resp_future = resp.send();
    
    match resp_future.await {
        Ok(data) => {
            match data.text().await {
                Ok(b) => print_json(&b),
                Err(e) => exit(&format!("Could not parse response: {e}")),
            };
        },
        Err(e) => exit(&format!("Could not curl KittyCAD API: {e}")),
    }
}

fn exit(s: &str) -> ! {
    eprintln!("{s}");
    std::process::exit(1)
}

#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    components: Components,
}

#[derive(serde::Deserialize, Debug)]
struct Components {
    responses: Responses,
}

#[derive(serde::Deserialize, Debug)]
struct Responses {
    error: serde_json::Value,
}

fn print_json(body: &str) {
    let json_root: ApiResponse = match serde_json::from_str(body) {
        Ok(d) => d,
        Err(e) => exit(&format!("Could not parse as JSON: {e}")),
    };

    println!("{:?}", &json_root);
    println!("error is: {}", json_root.components.responses.error);
}