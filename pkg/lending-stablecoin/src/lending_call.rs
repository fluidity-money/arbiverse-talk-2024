
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use crate::lending_call_wasm::*;
    } else {
        pub use crate::lending_call_host::*;
    }
}
