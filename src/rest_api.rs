pub const API: &str = "http://api.zapper.fi/v1";

pub const API_CACHE_TIMEOUT_SECONDS: i32 = 60;

pub const API_KEY: &str = "96e0cc51-a62e-42ca-acee-910ea7d2a241";

pub const VALID_PROTOCOLS: &'static [&str] = &[
    "aave", "aave-amm", "aave-v2", "alpha", "b-protocol", "badger", "balancer", "bancor",
    "barnbridge", "bitcoin", "compound", "cover", "cream", "curve", "defisaver",
    "derivadex", "dhedge", "dforce", "dodo", "dsd", "dydx", "esd", "futureswap", "idle",
    "harvest", "hegic", "keeper-dao", "linkswap", "loopring", "maker", "mooniswap",
    "1inch", "pancakeswap", "nft", "other", "pickle", "pooltogether", "quickswap",
    "rari", "realt", "reflexer", "saddle", "sfinance", "shell", "smoothy", "snowswap",
    "sushiswap", "swerve", "synthetix", "tokensets", "tokens", "uniswap", "uniswap-v2",
    "unit", "value", "vesper", "xsigma", "yearn"
];

pub const VALID_FARMS: &'static [&str] = &[
    "masterchef", "single-staking", "geyser", "gauge"
];

pub const VALID_POOLS: &'static [&str] = &[
    "balancer", "bancor", "curve", "loopring", "oneInch", "pancakeswap", "quickswap",
    "sfinance", "snowswap", "sushiswap", "uniswap", "linkswap", "dodo", "saddle", "xsigma"
];

pub const VALID_NETWORKS: &'static [&str] = &[
    "ethereum", "binance-smart-chain", "polygon", "xdai"
];


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GasPrice {
    fast: f32,
    instant: f32,
    standard: f32
}

#[tokio::main]
pub async fn ethereum_gas_price() -> Result<GasPrice, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(API.to_owned() + "/gas-price")
        .query(&[("api_key", API_KEY)])
        .send().await?
        .json::<GasPrice>().await?;
    Ok(resp)
}

pub fn ethereum_gas_price_fast() -> f32 {
    ethereum_gas_price().unwrap().fast
}
pub fn ethereum_gas_price_instant() -> f32 {
    ethereum_gas_price().unwrap().instant
}
pub fn ethereum_gas_price_standard() -> f32 {
    ethereum_gas_price().unwrap().standard
}
