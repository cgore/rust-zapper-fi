use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::string::ToString;
use std::time::Duration;
use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter};
use ttl_cache::TtlCache;

const DEFAULT_API_CACHE_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_API_KEY: &str = "96e0cc51-a62e-42ca-acee-910ea7d2a241";
const DEFAULT_API_URL: &str = "http://api.zapper.fi/v1";

pub struct Client {
    api_cache_timeout: Duration,
    api_key: String,
    api_url: String,
    gas_price_cache: TtlCache<Network, GasPriceResponse>,
    http: reqwest::Client
}

impl Client {
    pub fn new() -> Client {
        Client {
            api_cache_timeout: DEFAULT_API_CACHE_TIMEOUT,
            api_key: DEFAULT_API_KEY.to_string(),
            api_url: DEFAULT_API_URL.to_string(),
            gas_price_cache: TtlCache::new(Network::COUNT),
            http: reqwest::Client::new()
        }
    }

    #[tokio::main]
    async fn get_fiat_rates(&self) -> Result<HashMap<String, Decimal>, Box<dyn std::error::Error>> {
        let resp = self.http.get(self.api_url.to_owned() + "/fiat-rates")
            .query(&[("api_key", &self.api_key)])
            .send().await?
            .json::<HashMap<String, Decimal>>().await?;
        Ok(resp)
    }

    #[tokio::main]
    async fn get_gas_price(&self, network: Network) -> Result<GasPriceResponse, Box<dyn std::error::Error>> {
        let resp = self.http.get(self.api_url.to_owned() + "/gas-price")
            .query(&[("api_key", &self.api_key),
                     ("network", &network.to_string())])
            .send().await?
            .json::<GasPriceResponse>().await?;
        Ok(resp)
    }

    fn update_gas_price(&mut self, network: Network) -> GasPriceResponse {
        println!("updating the gas price");
        let result = self.get_gas_price(network).unwrap();
        self.gas_price_cache.insert(network, result, self.api_cache_timeout);
        result
    }

    pub fn fiat_rate(&mut self, fiat_symbol: String) -> Decimal {
        self.get_fiat_rates().unwrap()[&fiat_symbol]
    }

    pub fn gas_price(&mut self, network: Network) -> GasPriceResponse {
        match self.gas_price_cache.get(&network) {
            Some(result) => *result,
            None => self.update_gas_price(network)
        }
    }
}

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

#[derive(Copy, Clone, Display, Debug, Eq, EnumCountMacro, EnumIter, Hash, PartialEq)]
pub enum Network {
    #[strum(serialize = "binance-smart-chain")]
    BinanceSmartChain,
    #[strum(serialize = "ethereum")]
    Ethereum,
    #[strum(serialize = "polygon")]
    Polygon,
    #[strum(serialize = "xdai")]
    XDAI
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct GasPriceResponse {
    pub fast: Decimal,
    pub instant: Decimal,
    pub standard: Decimal
}
