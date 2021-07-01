use zapper_fi::rest_api;

fn main() {
    let client = rest_api::Client::new();
    println!("Zapper.fi health {:#?}", client.health());
}
