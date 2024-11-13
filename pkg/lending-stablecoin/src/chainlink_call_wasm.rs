use stylus_sdk::{
    alloy_primitives::{I256, U256},
    call::RawCall,
};

use alloy_sol_types::SolCall;

use alloy_sol_macro::sol;

use crate::{error::Error, immutables::CHAINLINK_FEED_ADDR};

sol! {
    function latestRoundData();
}

/// Get the latest price data without taking into account when it was
/// collected. Uses CHAINLINK_FEED_ADDR in immutables.
/// If the return amount is negative (NOT very likely), we're going to error here.
pub fn get_latest_price() -> Result<U256, Error> {
    let w = unwrap_second_word(
        &RawCall::new()
            .call(CHAINLINK_FEED_ADDR, &latestRoundDataCall {}.abi_encode())
            .map_err(Error::ChainlinkError)?,
    )?;
    if w.is_negative() {
        return Err(Error::ChainlinkNegativeFeed);
    }
    Ok(w.into_raw())
}

// Chainlink's latest round for price returns 5 words. We're interested
// in the second.
fn unwrap_second_word(v: &[u8]) -> Result<I256, Error> {
    // Ordinarily this would be tested, but for our (lazy) workshop
    // sakes, that's not the case.
    I256::try_from_be_slice(&v[32..64]).ok_or(Error::ChainlinkUnpacking)
}
