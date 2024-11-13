use crate::{chainlink_call, error::Error, immutables::CHAINLINK_SCALING_FACTOR};

use stylus_sdk::alloy_primitives::U256;

/// Look up the price of ARB according to Chainlink, and use the scaling
/// math to figure out the value of the amount we were given in USD.
/// TODO maybe sus.
pub fn value_of_asset(amt: U256) -> Result<U256, Error> {
    Ok((chainlink_call::get_latest_price()? * amt) / CHAINLINK_SCALING_FACTOR)
}
