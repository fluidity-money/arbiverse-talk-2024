use stylus_sdk::alloy_primitives::{address, Address, U256};

use array_concat::{concat_arrays};

/// Address of ARB on Arbitrum Mainnet.
pub const ARB_ADDR: Address = address!("912CE59144191C1204E64559FE8253a0e49E6548");

/// Scaling factor to use for any mul_div operation in lieu of decimal math.
//1e18
pub const SCALING_FACTOR: U256 = U256::from_limbs([1000000000000000000, 0, 0, 0]);

/// Scaled interest rate per second.
//0.005 / (365 * 24 * 60 * 60) = 158548960
pub const INTEREST_PER_SEC_RATE: U256 = U256::from_limbs([158548960, 0, 0, 0]);

/// Scaled collateral requirement (1.10).
pub const COLLATERAL_REQ: U256 = U256::from_limbs([1100000000000000000, 0, 0, 0]);

/// Scaled security deposit (0.005).
pub const SECURITY_DEPOSIT: U256 = U256::from_limbs([5000000000000000, 0, 0, 0]);

/// The scaling factor Chainlink uses.
//1e8
pub const CHAINLINK_SCALING_FACTOR: U256 = U256::from_limbs([100000000, 0, 0, 0]);

/// Hardcoded Arbitrum mainnet price feed for ARB.
pub const CHAINLINK_FEED_ADDR: Address = address!("b2A824043730FE05F3DA2efaFa1CBbe83fa548D6");

// Minimal viable proxy bytecode.
pub const NORMAL_PROXY_BYTECODE_1: [u8; 18] = [
    0x60, 0x2d, 0x5f, 0x81, 0x60, 0x09, 0x5f, 0x39, 0xf3, 0x5f, 0x5f, 0x36, 0x5f, 0x5f, 0x37, 0x36,
    0x5f, 0x73,
];

pub const NORMAL_PROXY_BYTECODE_2: [u8; 16] = [
    0x5a, 0xf4, 0x3d, 0x5f, 0x5f, 0x3e, 0x60, 0x29, 0x57, 0x3d, 0x5f, 0xfd, 0x5b, 0x3d, 0x5f, 0xf3,
];

pub fn create_proxy_bytecode(addr: Address) -> [u8; 54] {
    concat_arrays!(
        NORMAL_PROXY_BYTECODE_1,
        addr.into_array(),
        NORMAL_PROXY_BYTECODE_2
    )
}
