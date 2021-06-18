use zapper_fi::rest_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(rest_api::API.to_owned() + "/gas-price")
        .query(&[("api_key",rest_api::API_KEY)])
        .send()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
