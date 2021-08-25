//! Contains command line arguments and related helpers that are shared between the binaries.
use crate::{gas_price_estimation::GasEstimatorType, sources::BaselineSource};
use ethcontract::{H160, U256};
use std::{
    num::{NonZeroU64, ParseFloatError},
    time::Duration,
};
use url::Url;

#[derive(Debug, structopt::StructOpt)]
pub struct Arguments {
    #[structopt(
        long,
        env = "LOG_FILTER",
        default_value = "warn,orderbook=debug,solver=debug,shared=debug,shared::transport::http=info,archerapi=info"
    )]
    pub log_filter: String,

    /// The Ethereum node URL to connect to.
    #[structopt(long, env = "NODE_URL", default_value = "http://localhost:8545")]
    pub node_url: Url,

    /// Timeout for all http requests.
    #[structopt(
            long,
            default_value = "10",
            parse(try_from_str = duration_from_seconds),
        )]
    pub http_timeout: Duration,

    /// Which gas estimators to use. Multiple estimators are used in sequence if a previous one
    /// fails. Individual estimators support different networks.
    /// `EthGasStation`: supports mainnet.
    /// `GasNow`: supports mainnet.
    /// `GnosisSafe`: supports mainnet and rinkeby.
    /// `Web3`: supports every network.
    #[structopt(
        long,
        env = "GAS_ESTIMATORS",
        default_value = "Web3",
        possible_values = &GasEstimatorType::variants(),
        case_insensitive = true,
        use_delimiter = true
    )]
    pub gas_estimators: Vec<GasEstimatorType>,

    /// Base tokens used for finding multi-hop paths between multiple AMMs
    /// Should be the most liquid tokens of the given network.
    #[structopt(long, env = "BASE_TOKENS", use_delimiter = true)]
    pub base_tokens: Vec<H160>,

    /// Gas Fee Subsidy: 0 means no subsidy, 0.9 means 90% subsidized.
    #[structopt(long, env = "FEE_SUBSIDY_FACTOR", default_value = "0")]
    pub fee_subsidy_factor: f64,

    /// Which Liquidity sources to be used by Price Estimator.
    #[structopt(
        long,
        env = "BASELINE_SOURCES",
        default_value = "Uniswap,Sushiswap",
        possible_values = &BaselineSource::variants(),
        case_insensitive = true,
        use_delimiter = true
    )]
    pub baseline_sources: Vec<BaselineSource>,

    /// The number of blocks kept in the pool cache.
    #[structopt(long, env, default_value = "10")]
    pub pool_cache_blocks: NonZeroU64,

    /// The number of pairs that are automatically updated in the pool cache.
    #[structopt(long, env, default_value = "4")]
    pub pool_cache_maximum_recent_block_age: u64,

    /// How often to retry requests in the pool cache.
    #[structopt(long, env, default_value = "5")]
    pub pool_cache_maximum_retries: u32,

    /// How long to sleep between retries in the pool cache.
    #[structopt(long, env, default_value = "1", parse(try_from_str = duration_from_seconds))]
    pub pool_cache_delay_between_retries_seconds: Duration,

    /// How often we poll the node to check if the current block has changed.
    #[structopt(
        long,
        env,
        default_value = "5",
        parse(try_from_str = duration_from_seconds),
    )]
    pub block_stream_poll_interval_seconds: Duration,

    /// Default is one atom of native_token (i.e. price estimation uses spot prices).
    #[structopt(long, env, default_value = "1")]
    pub amount_to_estimate_prices_with: U256,
}

impl Arguments {
    pub fn validate(&self) {
        assert!(
            0f64 <= self.fee_subsidy_factor && self.fee_subsidy_factor <= 1f64,
            "Fee subsidy must be in the range [0, 1]"
        );
    }
}

pub fn duration_from_seconds(s: &str) -> Result<Duration, ParseFloatError> {
    Ok(Duration::from_secs_f32(s.parse()?))
}

pub fn wei_from_base_unit(s: &str) -> anyhow::Result<U256> {
    Ok(U256::from_dec_str(s)? * U256::exp10(18))
}

pub fn wei_from_gwei(s: &str) -> anyhow::Result<f64> {
    let in_gwei: f64 = s.parse()?;
    Ok(in_gwei * 10e9)
}
