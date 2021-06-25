use zapper_fi::rest_api;
use zapper_fi::rest_api::Network;

fn main() {
    let mut client = rest_api::Client::new();
    println!("{:#?}", client.gas_price(Network::Ethereum).standard);
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("{:#?}", client.gas_price(Network::Ethereum).standard);
    std::thread::sleep(std::time::Duration::from_secs(60));
    println!("{:#?}", client.gas_price(Network::Ethereum).standard);
}
