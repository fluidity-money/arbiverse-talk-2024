use stylus_sdk::{alloy_primitives::*, call::RawCall};

use alloy_sol_types::SolCall;

use alloy_sol_macro::sol;

use crate::error::Error;

sol! {
    function initialize(address initialOwner);
    function transfer(address recipient, uint256 amount);
    function transferFrom(address spender, address recipient, uint256 amount);
    function mint(address to, uint256 amount);
    function burn(uint256 amount);
}

pub fn initialise(token: Address, owner: Address) -> Result<(), Error> {
    RawCall::new()
        .call(
            token,
            &initializeCall {
                initialOwner: owner,
            }
            .abi_encode(),
        )
        .map_err(Error::ERC20Failed)?;
    Ok(())
}

pub fn transfer(token: Address, recipient: Address, amount: U256) -> Result<(), Error> {
    RawCall::new()
        .call(token, &transferCall { recipient, amount }.abi_encode())
        .map_err(Error::ERC20Failed)?;
    Ok(())
}

pub fn transfer_from(
    token: Address,
    spender: Address,
    recipient: Address,
    amount: U256,
) -> Result<(), Error> {
    RawCall::new()
        .call(
            token,
            &transferFromCall {
                spender,
                recipient,
                amount,
            }
            .abi_encode(),
        )
        .map_err(Error::ERC20Failed)?;
    Ok(())
}

pub fn mint(token: Address, to: Address, amount: U256) -> Result<(), Error> {
    RawCall::new()
        .call(token, &mintCall { to, amount }.abi_encode())
        .map_err(Error::ERC20Failed)?;
    Ok(())
}

pub fn burn(token: Address, amount: U256) -> Result<(), Error> {
    RawCall::new()
        .call(token, &burnCall { amount }.abi_encode())
        .map_err(Error::ERC20Failed)?;
    Ok(())
}
