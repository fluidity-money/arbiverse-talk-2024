#![cfg_attr(target_arch = "wasm32", no_main)]
#![no_std]

extern crate alloc;

#[macro_use]
pub mod error;

pub mod immutables;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod erc20_call_wasm;
        mod chainlink_call_wasm;
        mod lending_call_wasm;
    } else {
        mod erc20_call_host;
        mod chainlink_call_host;
        mod lending_call_host;
    }
}
mod chainlink_call;
mod erc20_call;
mod lending_call;

pub mod chainlink;

mod contract_factory;
mod contract_lender;

#[cfg(feature = "contract-factory")]
pub use contract_factory::*;

pub use contract_factory::StorageFactory;

#[cfg(feature = "contract-lending")]
pub use contract_lender::*;

pub use contract_lender::StorageLender;

#[cfg(not(any(
    feature = "testing",
    not(target_arch = "wasm32"),
    feature = "contract-factory",
    feature = "contract-lending",
)))]
compile_error!(
    "contract-factory or contract-lending feature must be enabled"
);
