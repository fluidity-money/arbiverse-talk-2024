#!/usr/bin/env python3

"""
Reference implementation of a Compound-like stablecoin lending protocol.
"""

import unittest

class MockOracle:
	price = 1
	def update_price(self, new_price):
		self.price = new_price
	def cur_price(self):
		return self.price
	def value_of_asset(self, a):
		return self.cur_price() * a

class BadDebt(Exception):
	pass

class NotAbleToLiquidate(Exception):
	pass

class NotEnoughCash(Exception):
	pass

class Timepoint:
	def __init__(self, time, interest):
		self.time = time
		self.interest = interest
	def __repr__(self):
		return f"Timepoint({self.time},{self.interest})"

class Token:
	INTEREST_RATE = 0.005
	INTEREST_PER_SEC_RATE = INTEREST_RATE / (365 * 24 * 60 * 60)
	COLLATERAL_REQ = 1.10
	SECURITY_DEPOSIT_RATE = 0.005
	token_for_redemptions = 0
	cash_supply = 0
	oracle = None
	borrows = {}
	collateral = {}
	debt = {}
	security_deposits = 0

	def __init__(self, oracle):
		self.oracle = oracle

	@staticmethod
	def utilisation_rate(borrowed, cash):
		return borrowed / cash

	def borrow(self, ticket, cur_time, ausd_amt, token_collateral):
		usd_collateral = self.oracle.value_of_asset(token_collateral)
		if self.utilisation_rate(ausd_amt, usd_collateral) > self.COLLATERAL_REQ:
			raise BadDebt
		redemption_amt = token_collateral * (ausd_amt / usd_collateral)
		security_deposit = redemption_amt * self.SECURITY_DEPOSIT_RATE
		token_collateral -= security_deposit
		redemption_amt -= security_deposit
		self.token_for_redemptions += redemption_amt
		self.security_deposits += security_deposit
		self.cash_supply += ausd_amt
		self.borrows[ticket] = [Timepoint(cur_time, 0)]
		self.collateral[ticket] = token_collateral
		self.debt[ticket] = ausd_amt
		return ausd_amt

	def record_timepoint(self, ticket, cur_time):
		last_timepoint = self.borrows[ticket][-1]
		# We don't dupe the last timepoint if the time is the same.
		if last_timepoint.time == cur_time:
			return last_timepoint
		# We make sure to increase the interest that's accrued since last time by
		# auto compounding their debt.
		interest = (self.debt[ticket] + last_timepoint.interest) * (self.INTEREST_PER_SEC_RATE * (cur_time - last_timepoint.time))
		new_timepoint = Timepoint(cur_time, interest)
		self.borrows[ticket].append(new_timepoint)
		return new_timepoint

	def debt_outstanding(self, ticket, cur_time):
		timepoint = self.record_timepoint(ticket, cur_time)
		debt = self.debt[ticket] + timepoint.interest
		return debt

	def liquidate(self, ticket, cur_time):
		timepoint = self.record_timepoint(ticket, cur_time)
		debt_outstanding = self.debt[ticket] + timepoint.interest
		token_collateral = self.collateral[ticket]
		usd_collateral = self.oracle.value_of_asset(token_collateral)
		if self.utilisation_rate(debt_outstanding, usd_collateral) <= self.COLLATERAL_REQ:
			raise NotAbleToLiquidate
		self.debt[ticket] = 0
		self.token_collateral -= token_collateral
		# The amount that's needed to plug this shortfall. Assuming we have it.
		# We don't have a emergency mode.
		collateral_diff = usd_collateral - debt_outstanding
		self.security_deposits -= max(collateral_diff, self.security_deposits - collateral_diff)
		self.collateral[ticket] = 0
		self.debt[ticket] = 0
		self.borrows[ticket][-1].interest = 0
		return ticket

	def repay(self, ticket, cur_time, token_repay):
		# Repay a ticket's debt by paying down the interest on the right side, and
		# if we can pay down the principal debt, that too.
		timepoint = self.record_timepoint(ticket, cur_time)
		debt_outstanding = self.debt[ticket] + timepoint.interest
		usd_repay = self.oracle.value_of_asset(token_repay)
		interest = timepoint.interest
		interest_leftover = usd_repay - interest
		self.borrows[ticket][-1].interest = max(0, interest - usd_repay)
		usd_leftover = max(0, interest_leftover - self.debt[ticket])
		self.debt[ticket] -= interest_leftover
		return usd_leftover

	def redeem(self, cash):
		#= Take the share of the token that's held by this
		# contract, using the percentage of ownership of the USD
		# token relative to everyone else.
		token_amt = self.token_for_redemptions * (cash / self.cash_supply)
		self.token_for_redemptions -= token_amt
		return token_amt

class TestToken(unittest.TestCase):
	def setUp(self):
		self.oracle = MockOracle()
		self.token = Token(self.oracle)
	def test_e2e_1(self):
		"""
		Alex goes to create $100 of token with collateral worth $200. Only $100 is
		needed to match the AUSD deposit. 2 months pass, and the collateral is
		still worth $200. He redeems $99.5 AUSD. His outstanding
		debt is 100 + (100 * (INTEREST_PER_SEC_RATE * (2 * 30 * 24 * 60 * 60))).
		Alex goes to repay $90 on the same time, leaving him with
		10.08219178082192.
		"""
		self.token.borrow("Alex", 0, 100, 200)
		# 2 months from now.
		cur_time = 2 * 30 * 24 * 60 * 60
		self.assertEqual(99.0025, self.token.redeem(99.5))
		self.assertEqual(
			100.08219178082192,
			self.token.debt_outstanding("Alex", cur_time))
		self.assertEqual(
			0,
			self.token.repay("Alex", cur_time, 80))
		self.assertEqual(
			20.082191780821915,
			self.token.debt_outstanding("Alex", cur_time))

if __name__ == "__main__":
	unittest.main()
