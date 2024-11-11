# Utilisation rate for a.
U_a = Borrowed_a / (Cash_Reserves_a + Borrowed_a)
# Interest rate for the market a. Set by the DAO.
I_a = 0.025 + U_a * 0.2
# Exchange rate for the ctoken of the asset.
E_a = (underlying_bal + total_borrow_bal_a + reserves_a) / ctoken_supply_a
# Interest rate index of what's accrued. This is needed to compound interest that's
# accumulated since the last index. Interest being the interest of this period (in the
# whitepaper, denominated as r * t).
Index_a[n] = Index_a[n-1] * (1 + Interest_a)
# Which then updates the total borrowing balance for the pool:
total_borrow_bal_a[n] = total_borrow_bal_a[n-1] * (1 + Interest_a)
# Some of the returns are set aside for reserves. Reserve factor is set by the DAO, and is
# a percentage.
reserves_a = reserves_a[n-1] + total_borrow_bal_a[n-1] * (Interest_a * reserve_factor)
