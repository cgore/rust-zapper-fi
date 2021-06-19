use zapper_fi::rest_api;

fn main() {
    println!("{:#?}", rest_api::ethereum_gas_price_standard());
}
