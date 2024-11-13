
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use crate::erc20_call_wasm::*;
    } else {
        pub use crate::erc20_call_host::*;
    }
}
