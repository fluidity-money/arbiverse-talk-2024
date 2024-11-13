use stylus_sdk::{alloy_primitives::*, block, contract, msg, prelude::*, storage::*};

use alloc::{vec, vec::Vec};

use crate::{
    chainlink, erc20_call,
    error::Error,
    immutables::{
        ARB_ADDR, COLLATERAL_REQ, INTEREST_PER_SEC_RATE, SCALING_FACTOR, SECURITY_DEPOSIT,
    },
};

#[storage]
pub struct StorageTimepoint {
    /// The latest timestamp.
    pub time: StorageU256,
    /// Interest accumulated, scaled by the scaling factor.
    pub interest: StorageU256,
}

#[storage]
#[cfg_attr(feature = "contract-lending", entrypoint)]
pub struct StorageLender {
    /// Was this proxy set up properly?
    created: StorageU256,

    /// Token that we're associated with.
    token_addr: StorageAddress,

    /// The collateral that's okay to use for redemption.
    token_for_redemption: StorageU256,

    /// Supply of the connected token, stored here to avoid an excessive call.
    pub cash_supply: StorageU256,

    /// Borrow count for the ticket increment.
    pub borrow_count: StorageU256,

    /// Borrow timepoints made against the identifier.
    pub borrows: StorageMap<U256, StorageVec<StorageTimepoint>>,

    /// Owners of the tickets floating around in the system.
    pub ticket_owners: StorageMap<U256, StorageAddress>,

    /// Collateral supplied by the identifier.
    pub collateral: StorageMap<U256, StorageU256>,

    /// Debt minted by the identifier given.
    pub debt: StorageMap<U256, StorageU256>,

    /// Amount of cash set aside for a bad debt situation.
    pub security_deposits: StorageU256,
}

#[public]
impl StorageLender {
    pub fn ctor(&mut self, token: Address) -> Result<(), Vec<u8>> {
        assert_or!(self.created.get().is_zero(), Error::AlreadyInitialised);
        self.token_addr.set(token);
        self.created.set(U256::from(1));
        Ok(())
    }

    /// Return the utilisation rate, scaled by the scaling factor.
    pub fn utilisation_rate(borrowed: U256, cash: U256) -> U256 {
        borrowed / cash
    }

    fn borrow_internal(
        &mut self,
        cur_time: u64,
        ausd_amt: U256,
        token_collateral: U256,
    ) -> Result<U256, Error> {
        let ticket = self.borrow_count.get();
        self.borrow_count.set(ticket + U256::from(1));
        let mut scaled_ausd_amt = ausd_amt * SCALING_FACTOR;
        let mut scaled_token_collateral = ausd_amt * SCALING_FACTOR;
        let scaled_usd_collateral = chainlink::value_of_asset(token_collateral)? * SCALING_FACTOR;
        let is_underutilised =
            Self::utilisation_rate(scaled_ausd_amt, scaled_usd_collateral) < COLLATERAL_REQ;
        assert_or!(is_underutilised, Error::BadBorrowAttempt);
        let mut scaled_redemption_amt = scaled_token_collateral
            .checked_mul(
                scaled_ausd_amt
                    .checked_div(scaled_usd_collateral)
                    .ok_or(Error::CheckedDiv)?,
            )
            .ok_or(Error::CheckedMul)?;
        scaled_redemption_amt = scaled_redemption_amt
            .checked_sub(scaled_redemption_amt * SECURITY_DEPOSIT)
            .ok_or(Error::CheckedSub)?;
        let redemption_amt = scaled_redemption_amt / SCALING_FACTOR;
        self.token_for_redemption.set(
            self.token_for_redemption
                .get()
                .checked_add(redemption_amt)
                .ok_or(Error::CheckedAdd)?,
        );
        let scaled_ausd_security_deposit = scaled_ausd_amt * SECURITY_DEPOSIT;
        let ausd_security_deposit = scaled_ausd_security_deposit / SCALING_FACTOR;
        let scaled_token_security_deposit = scaled_token_collateral * SECURITY_DEPOSIT;
        scaled_ausd_amt -= scaled_ausd_security_deposit;
        scaled_token_collateral -= scaled_token_security_deposit;
        self.security_deposits.set(
            self.security_deposits
                .get()
                .checked_add(ausd_security_deposit)
                .ok_or(Error::CheckedAdd)?,
        );
        // We have to scale this again so we correctly get the amount
        // adjusted for the security deposit.
        let ausd_amt = scaled_ausd_amt / SCALING_FACTOR;
        self.cash_supply.set(
            self.cash_supply
                .get()
                .checked_add(ausd_amt)
                .ok_or(Error::CheckedAdd)?,
        );
        let mut borrows = self.borrows.setter(ticket);
        let mut timepoint = borrows.grow();
        timepoint.time.set(U256::from(cur_time));
        // Interest in the timepoint should be set to 0 by default.
        let token_collateral = scaled_token_collateral / SCALING_FACTOR;
        self.collateral.setter(ticket).set(token_collateral);
        self.debt.setter(ticket).set(ausd_amt);
        Ok(ticket)
    }

    pub fn borrow(
        &mut self,
        ausd_amt: U256,
        token_collateral: U256,
        recipient: Address,
    ) -> Result<U256, Error> {
        erc20_call::transfer_from(
            ARB_ADDR,
            msg::sender(),
            contract::address(),
            token_collateral,
        )?;
        let ticket = self.borrow_internal(block::timestamp(), ausd_amt, token_collateral)?;
        self.ticket_owners.setter(ticket).set(recipient);
        erc20_call::mint(self.token_addr.get(), recipient, ausd_amt)?;
        Ok(ticket)
    }

    pub fn borrow_testing(
        &mut self,
        _cur_time: u64,
        _ausd_amt: U256,
        _token_collateral: U256,
    ) -> Result<U256, Vec<u8>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "testing")] {
                Ok(self.borrow_internal(_cur_time, _ausd_amt, _token_collateral)?)
            } else {
                // Return nothing, so hopefully the caller gets a clue
                // that they shouldn't have invoked this.
                Err(vec![])
            }
        }
    }

    /// Record a new timepoint, assuming that it exists. Returns a handy
    /// Timepoint structure for internal use.
    fn record_timepoint_internal(
        &mut self,
        ticket: U256,
        cur_time: u64,
    ) -> Result<(U256, U256), Error> {
        let cur_time = U256::from(cur_time);
        let borrows = self.borrows.getter(ticket);
        let last_timepoint = borrows
            .getter(self.borrows.getter(ticket).len() - 1)
            .unwrap();
        if last_timepoint.time.get() == U256::from(cur_time) {
            return Ok((last_timepoint.time.get(), last_timepoint.interest.get()));
        }
        let scaled_outstanding_debt =
            (self.debt.getter(ticket).get() + last_timepoint.interest.get()) * SCALING_FACTOR;
        let scaled_interest_pct_incr =
            INTEREST_PER_SEC_RATE * (cur_time - last_timepoint.time.get());
        let scaled_interest = scaled_outstanding_debt * scaled_interest_pct_incr;
        let mut borrows = self.borrows.setter(ticket);
        let mut timepoint = borrows.grow();
        timepoint.time.set(cur_time);
        let interest = scaled_interest / SCALING_FACTOR;
        timepoint.interest.set(interest);
        Ok((cur_time, interest))
    }

    pub fn record_timepoint_testing(
        &mut self,
        _ticket: U256,
        _cur_time: u64,
    ) -> Result<(U256, U256), Vec<u8>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "testing")] {
                Ok(self.record_timepoint_internal(_ticket, _cur_time)?)
            } else {
                // Return nothing, so hopefully the caller gets a clue
                // that they shouldn't have invoked this.
                Err(vec![])
            }
        }
    }

    pub fn debt_outstanding(&mut self, ticket: U256, cur_time: u64) -> Result<U256, Error> {
        let (_, timepoint_interest) = self.record_timepoint_internal(ticket, cur_time)?;
        Ok(self.debt.getter(ticket).get() + timepoint_interest)
    }

    fn liquidate_internal(&mut self, ticket: U256, cur_time: u64) -> Result<(), Error> {
        let (_, timepoint_interest) = self.record_timepoint_internal(ticket, cur_time)?;
        let debt_outstanding = self.debt.getter(ticket).get() + timepoint_interest;
        let scaled_debt_outstanding = debt_outstanding * SCALING_FACTOR;
        let token_collateral = self.collateral.getter(ticket).get();
        let usd_collateral = chainlink::value_of_asset(token_collateral)?;
        let scaled_usd_collateral = usd_collateral * SCALING_FACTOR;
        let utilisation = Self::utilisation_rate(scaled_debt_outstanding, scaled_usd_collateral);
        assert_or!(utilisation <= COLLATERAL_REQ, Error::NotAbleToLiquidate);
        self.debt.setter(ticket).set(U256::ZERO);
        let collateral_diff = usd_collateral - debt_outstanding;
        self.security_deposits
            .set(if collateral_diff > self.security_deposits.get() {
                U256::ZERO
            } else {
                usd_collateral - debt_outstanding
            });
        self.collateral.setter(ticket).set(U256::ZERO);
        self.debt.setter(ticket).set(U256::ZERO);
        let tickets_len = self.borrows.getter(ticket).len();
        // Override the timepoint we just set using the grow function.
        self.borrows
            .setter(ticket)
            .setter(tickets_len - 1)
            .unwrap()
            .interest
            .set(U256::ZERO);
        Ok(())
    }

    pub fn liquidate(&mut self, ticket: U256) -> Result<(), Error> {
        self.liquidate_internal(ticket, block::timestamp())
    }

    pub fn liquidate_testing(&mut self, _ticket: U256, _cur_time: u64) -> Result<(), Vec<u8>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "testing")] {
                Ok(self.liquidate_internal(_ticket, _cur_time)?)
            } else {
                // Return nothing, so hopefully the caller gets a clue
                // that they shouldn't have invoked this.
                Err(vec![])
            }
        }
    }

    fn repay_internal(
        &mut self,
        ticket: U256,
        cur_time: u64,
        token_repay: U256,
    ) -> Result<(), Error> {
        let (_, timepoint_interest) = self.record_timepoint_internal(ticket, cur_time)?;
        let usd_repay = chainlink::value_of_asset(token_repay)?;
        let borrows_len = self.borrows.getter(ticket).len();
        let mut leftover = usd_repay;
        if timepoint_interest > leftover {
            self.borrows
                .setter(ticket)
                .setter(borrows_len - 1)
                .unwrap()
                .interest
                .set(timepoint_interest - leftover);
            return Ok(());
        } else {
            self.borrows
                .setter(ticket)
                .setter(borrows_len - 1)
                .unwrap()
                .interest
                .set(U256::ZERO);
            leftover -= timepoint_interest;
        };
        let outstanding_debt = self.debt.getter(ticket).get();
        if outstanding_debt > leftover {
            self.debt.setter(ticket).set(outstanding_debt - leftover);
        } else {
            self.debt.setter(ticket).set(U256::ZERO)
        };
        Ok(())
    }

    pub fn repay(&mut self, ticket: U256, token_repay: U256) -> Result<(), Error> {
        // TODO: return the unused amounts to the user.
        assert_or!(
            self.ticket_owners.getter(ticket).get() == msg::sender(),
            Error::NotOwner
        );
        erc20_call::transfer_from(ARB_ADDR, msg::sender(), contract::address(), token_repay)?;
        self.repay_internal(ticket, block::timestamp(), token_repay)
    }

    pub fn repay_testing(
        &mut self,
        _ticket: U256,
        _cur_time: u64,
        _token_repay: U256,
    ) -> Result<(), Vec<u8>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "testing")] {
                Ok(self.repay_internal(_ticket, _cur_time, _token_repay)?)
            } else {
                // Return nothing, so hopefully the caller gets a clue
                // that they shouldn't have invoked this.
                Err(vec![])
            }
        }
    }

    fn redeem_internal(&mut self, cash: U256) -> Result<U256, Error> {
        let scaled_token_for_redemption = self.token_for_redemption.get() * SCALING_FACTOR;
        let scaled_cash = cash * SCALING_FACTOR;
        let scaled_cash_supply = self.cash_supply.get() * SCALING_FACTOR;
        let token_amt = scaled_token_for_redemption * (scaled_cash / scaled_cash_supply);
        Ok(token_amt / SCALING_FACTOR)
    }

    pub fn redeem(&mut self, cash: U256, recipient: Address) -> Result<U256, Error> {
        // TODO: modify the ERC20 to do this in one go.
        erc20_call::transfer_from(
            self.token_addr.get(),
            msg::sender(),
            contract::address(),
            cash,
        )?;
        erc20_call::burn(self.token_addr.get(), cash)?;
        let redeemed = self.redeem_internal(cash)?;
        erc20_call::transfer(ARB_ADDR, recipient, redeemed)?;
        Ok(redeemed)
    }

    pub fn redeem_testing(&mut self, _cash: U256) -> Result<U256, Vec<u8>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "testing")] {
                Ok(self.redeem_internal(_cash)?)
            } else {
                // Return nothing, so hopefully the caller gets a clue
                // that they shouldn't have invoked this.
                Err(vec![])
            }
        }
    }
}
