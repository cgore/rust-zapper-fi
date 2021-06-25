use zapper_fi::rest_api;
use zapper_fi::rest_api::Network;

fn main() {
    let client = rest_api::Client::new();
    println!("{:#?}", client.gas_price(Network::Ethereum).standard);
}
