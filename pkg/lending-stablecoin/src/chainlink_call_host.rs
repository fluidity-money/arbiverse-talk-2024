use stylus_sdk::alloy_primitives::U256;

use crate::{error::Error, immutables::CHAINLINK_SCALING_FACTOR};

pub fn get_latest_price() -> Result<U256, Error> {
    Ok(CHAINLINK_SCALING_FACTOR) // I think this is one?
}
