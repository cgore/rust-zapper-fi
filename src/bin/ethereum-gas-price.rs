use zapper_fi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(zapper_fi::API.to_owned() + "/gas-price")
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
