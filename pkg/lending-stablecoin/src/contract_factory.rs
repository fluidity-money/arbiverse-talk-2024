use stylus_sdk::{alloy_primitives::*, deploy::RawDeploy, prelude::*};

use crate::{erc20_call, error::Error, immutables::create_proxy_bytecode, lending_call};

#[storage]
#[cfg_attr(feature = "contract-factory", entrypoint)]
pub struct StorageFactory {}

#[public]
impl StorageFactory {
    pub fn deploy(erc20_impl: Address, lending_impl: Address) -> Result<(Address, Address), Error> {
        // Deploy the ERC20 proxy, and set it's proxy later to point to the Lending impl.
        let erc20_addr = unsafe {
            RawDeploy::new()
                .deploy(&create_proxy_bytecode(erc20_impl), U256::ZERO)
                .map_err(Error::DeployFailure)?
        };
        let lending_addr = unsafe {
            RawDeploy::new()
                .deploy(&create_proxy_bytecode(lending_impl), U256::ZERO)
                .map_err(Error::DeployFailure)?
        };
        erc20_call::initialise(erc20_addr, lending_addr)?;
        lending_call::ctor(lending_addr, erc20_addr)?;
        Ok((lending_addr, erc20_addr))
    }
}
