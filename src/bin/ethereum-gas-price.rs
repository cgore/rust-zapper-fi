use serde_json;
use zapper_fi::rest_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp: serde_json::Value = client.get(rest_api::API.to_owned() + "/gas-price")
        .query(&[("api_key",rest_api::API_KEY)])
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", resp["standard"]);
    Ok(())
}
