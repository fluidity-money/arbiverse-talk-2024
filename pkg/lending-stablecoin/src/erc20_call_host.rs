use stylus_sdk::alloy_primitives::*;

use crate::error::Error;

pub fn initialise(_token: Address, _owner: Address) -> Result<(), Error> {
    Ok(())
}

pub fn transfer(_token: Address, _recipient: Address, _amount: U256) -> Result<(), Error> {
    Ok(())
}

pub fn transfer_from(
    _token: Address,
    _spender: Address,
    _recipient: Address,
    _amount: U256,
) -> Result<(), Error> {
    Ok(())
}

pub fn mint(_token: Address, _to: Address, _amount: U256) -> Result<(), Error> {
    Ok(())
}

pub fn burn(_token: Address, _amount: U256) -> Result<(), Error> {
    Ok(())
}
