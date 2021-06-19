use zapper_fi::rest_api;

fn main() {
    let resp = rest_api::ethereum_gas_price_standard();
    println!("{:#?}", resp.unwrap());
}
