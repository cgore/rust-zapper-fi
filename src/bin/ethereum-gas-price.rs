use zapper_fi::rest_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(rest_api::API.to_owned() + "/gas-price")
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
