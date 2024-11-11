#!/usr/bin/env python3

"""
Does not include fees for the bad debt!
"""

class MockOracle:
	@staticmethod
	def cur_price():
		return 1
	@staticmethod
	def value_of_asset(a):
		return MockOracle.cur_price() * a

class BadDebt(Exception):
	pass

class NotAbleToLiquidate(Exception):
	pass

class Timepoint:
	def __init__(self, time, collateral):
		self.time = time
		self.collateral = collateral

class Token:
	INTEREST_RATE = 0.005
	INTEREST_PER_SEC_RATE = INTEREST_RATE / (365 * 24 * 60 * 60)
	COLLATERAL_REQ = 1.10
	token_collateral = 0
	usd_collateral = 0
	oracle = MockOracle()
	borrows = {}
	cash = {}

	@staticmethod
	def utilisation_rate(borrowed, cash):
		return borrowed / cash

	def borrow(self, ticket, cur_time, ausd_amt, token):
		# Take the collateral from the user, record a time
		# point of their collateral, and give them back some
		# AUSD.
		usd_collateral = MockOracle.value_of_asset(token)
		if self.utilisation_rate(ausd_amt, usd_collateral) > self.COLLATERAL_REQ:
			raise BadDebt
		self.token_collateral += token
		self.usd_collateral += usd_collateral
		# We track the user's balances at each timepoint. We track the amount they
		# supplied at first, and gradually take amounts from them.
		self.borrows[ticket] = [Timepoint(cur_time, ausd_amt)]
		self.cash[ticket] = ausd_amt
		return ausd_amt

	def record_timepoint(self, ticket, cur_time):
		# Record the amount outstanding for the ticket since the last time we
		# checked. Don't do any fancy work with taking amounts, they can
		# call repayment for that.
		# Get the latest timepoint:
		timepoint_count = len(self.borrows[ticket])
		if timepoint_count == 0:
			return None
		last_timepoint = self.borrows[ticket][timepoint_count-1]
		# Now that we have the latest timepoint, we can look
		# at the collateral now, and check the interest
		# accumulated on it.
		interest = last_timepoint.collateral * (self.INTEREST_PER_SEC_RATE * (cur_time - last_timepoint.time))
		new_collateral = last_timepoint.collateral - interest
		new_timepoint = Timepoint(cur_time, max(new_collateral, 0))
		self.borrows[ticket].append(new_timepoint)
		return new_timepoint

	def liquidate(self, ticket, cur_time):
		timepoint = self.record_timepoint(ticket, cur_time)
		cash = self.cash[ticket]
		usd_collateral = MockOracle.value_of_asset(timepoint.collateral)
		if self.utilisation_rate(cash, usd_collateral) > self.COLLATERAL_REQ:
			self.borrows[ticket].append(Timepoint(0, cur_time))
			return ticket
		else:
			raise NotAbleToLiquidate

if __name__ == "__main__":
	token = Token()
	token.borrow("hello", 0, 100, 200)
	try:
		token.liquidate("hello", 0)
		raise Exception("Liquidate not working")
	except NotAbleToLiquidate:
		# Do nothing
		None
