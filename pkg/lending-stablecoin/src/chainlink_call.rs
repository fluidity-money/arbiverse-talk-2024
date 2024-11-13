
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use crate::chainlink_call_wasm::*;
    } else {
        pub use crate::chainlink_call_host::*;
    }
}
