use stylus_sdk::{alloy_primitives::*, call::RawCall};

use alloy_sol_types::SolCall;

use alloy_sol_macro::sol;

use crate::error::Error;

sol! {
    function ctor(address token);
}

pub fn ctor(lending: Address, token: Address) -> Result<(), Error> {
    RawCall::new()
        .call(lending, &ctorCall { token }.abi_encode())
        .map_err(Error::CtorFailed)?;
    Ok(())
}
