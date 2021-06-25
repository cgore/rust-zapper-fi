use serde::{Serialize, Deserialize};
use std::string::ToString;
use std::time::Duration;
use strum::EnumCount;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter};
use ttl_cache::TtlCache;

pub const DEFAULT_API_KEY: &str = "96e0cc51-a62e-42ca-acee-910ea7d2a241";

pub struct Client {
    api_key: String,
    gas_price_cache: TtlCache<Network, GasPriceResponse>,
    http: reqwest::Client
}

impl Client {
    pub fn new() -> Client {
        Client {
            api_key: DEFAULT_API_KEY.to_string(),
            gas_price_cache: TtlCache::new(Network::COUNT),
            http: reqwest::Client::new()
        }
    }

    #[tokio::main]
    async fn get_gas_price(&self, network: Network) -> Result<GasPriceResponse, Box<dyn std::error::Error>> {
        let resp = self.http.get(API.to_owned() + "/gas-price")
            .query(&[("api_key", &self.api_key),
                     ("network", &network.to_string())])
            .send().await?
            .json::<GasPriceResponse>().await?;
        Ok(resp)
    }

    pub fn gas_price(&self, network: Network) -> GasPriceResponse {
        self.get_gas_price(network).unwrap()
    }
}

pub const API: &str = "http://api.zapper.fi/v1";

pub const API_CACHE_TIMEOUT: Duration = Duration::from_secs(60);

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

#[derive(Display, Debug, Eq, EnumCountMacro, EnumIter, Hash, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GasPriceResponse {
    pub fast: f32,
    pub instant: f32,
    pub standard: f32
}
