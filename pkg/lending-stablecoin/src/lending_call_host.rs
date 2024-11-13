use crate::error::Error;

use stylus_sdk::alloy_primitives::Address;

pub fn ctor(_lending: Address, _token: Address) -> Result<(), Error> {
    Ok(())
}
