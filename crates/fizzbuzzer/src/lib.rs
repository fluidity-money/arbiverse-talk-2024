use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

extern crate alloc;

#[cfg(test)]
use proptest::prelude::*;

#[storage]
#[entrypoint]
struct Fizzbuzzer {
    pub counter: StorageU256,
}

#[public]
impl Fizzbuzzer {
    pub fn ctor(&mut self, c: U256) -> Result<(), Vec<u8>> {
        self.counter.set(c);
        Ok(())
    }

    pub fn is_fizzbuzz(&self) -> Result<bool, Vec<u8>> {
        let c = self.counter.get();
        Ok((c % U256::from(3)).is_zero() && (c % U256::from(5)).is_zero())
    }
}

#[cfg(test)]
proptest! {
    #[motsu::test]
    fn test_contract_fizzbuzzer(num in any::<u128>()) {
        // No fixtures so we can create this ourselves!
        let mut c = unsafe { <Fizzbuzzer as stylus_sdk::storage::StorageType>::new(U256::ZERO, 0) };
        let num = U256::from(num);
        c.ctor(num).unwrap();
        let is_fizz = num % U256::from(3) == U256::ZERO;
        let is_buzz = num % U256::from(5) == U256::ZERO;
        assert_eq!(c.is_fizzbuzz().unwrap(), is_fizz && is_buzz);
    }
}
