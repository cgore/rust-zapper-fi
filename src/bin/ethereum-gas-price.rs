use zapper_fi::rest_api;

fn main() {
    println!("{:#?}", rest_api::Network::Ethereum.standard_gas_price());
}
