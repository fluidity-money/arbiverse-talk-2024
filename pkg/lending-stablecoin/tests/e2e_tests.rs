#![cfg(feature = "testing")]

use liblendingstablecoin::{
    immutables::{SCALING_FACTOR, SECURITY_DEPOSIT},
    StorageLender,
};

use stylus_sdk::alloy_primitives::*;

use proptest::prelude::*;

proptest! {
    #[motsu::test]
    fn test_part_3(ausd_amt in 1..u128::MAX, secs_since in 1..u64::MAX) {
        let mut c = unsafe { <StorageLender as stylus_sdk::storage::StorageType>::new(U256::ZERO, 0) };
        let ausd_amt = U256::from(ausd_amt);
        let mut scaled_ausd_amt = ausd_amt * SCALING_FACTOR;
        let mut scaled_collateral =
            scaled_ausd_amt * U256::from(15000000000000000000_u128);
        let collateral = scaled_collateral / SCALING_FACTOR;
        let ticket = c.borrow_testing(0, ausd_amt, collateral).unwrap();
        scaled_ausd_amt -= scaled_ausd_amt * SECURITY_DEPOSIT;
        let ausd_amt = scaled_ausd_amt / SCALING_FACTOR;
        scaled_collateral -= scaled_collateral * SECURITY_DEPOSIT;
        assert_eq!(ausd_amt, c.debt.getter(ticket).get());
        c.record_timepoint_testing(ticket, secs_since).unwrap();
    }
}
