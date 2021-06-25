use zapper_fi::rest_api;

fn main() {
    let _client = rest_api::Client::new();
    println!("{:#?}", rest_api::Network::Ethereum.standard_gas_price());
}
