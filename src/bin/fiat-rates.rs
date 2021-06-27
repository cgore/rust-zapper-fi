use zapper_fi::rest_api;

fn main() {
    let mut client = rest_api::Client::new();
    println!("BTC fiat rate: {:#?}", client.fiat_rate("BTC".to_string()));
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("BTC fiat rate: {:#?}", client.fiat_rate("BTC".to_string()));
    std::thread::sleep(std::time::Duration::from_secs(60));
    println!("BTC fiat rate: {:#?}", client.fiat_rate("BTC".to_string()));
}
