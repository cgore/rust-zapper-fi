#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://api.zapper.fi/v1".to_owned() + "/gas-price")
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
