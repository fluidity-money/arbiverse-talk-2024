"""
aToken design:
"""
# The current liquidity rate (with U_t_a being utilisation at the
# current point in time) - a function of the borrow rate and the
# utilisation rate.
LR_t = R_t * U_t
# Cumulated liquidity index: the interest accumulated during the time
# interval, updated whenever borrow, deposit, repay, redeem, swap, and
# liquidation events occur.
LI_t = ((LR_t * ▲ * T_year) + 1) / LI[t-1]
LI_0 = 1 * (10 ^ 27)
# The ongoing interest accumulated by the reserve.
NI_t = (LR_t * ▲ * T_year + 1) / LI[t-1]
# Scaled balance function, a part of the aToken value calculation. A
# ratio including the principal amount supplied by the holder.
# On deposit:
ScB_t(x) = ScB[t-1](x) + (m / NI_t)
# Withdrawals:
ScB_t(x) = Scb[t-1](x) + (m / NI_t)
"""
Debt Tokenisation:
"""
# The total supply of the debt token, including what's accrued per
# second. ScB(i) is the amount borrowed by each user i.
dS_t = sum(|i| ScB_t(i), Δ)
# The total debt on an asset at time t, with SD being the stable debt,
# and VD being the variable debt.
D_t_a = SD_t + VD_t
"""
Functions for variable debt:
"""
# The cumulative variable borrow index. Interest accumulated by the
# variable borrows VB during a time T, at variable rate VR, at variable
# rate VR, updated during updates.
VI_t_a = ((1 + (VR_t / T_year)^ΔT) * VI[t-1]
# The user cumulated variable borrow index. Variable borrow index of the
# specific user, stored when a user opens a variable borrow position.
VI(x) = VI[t(x)]
# The user's principal borrow balance.
VN_t = ((1 + (VR_t / T_year)) ^ ΔT) * VI[i-1]
# Normalised variable debt function:
VN_t = ((1 + (VR_t / T_year)) ^ ΔT) * VI[t-1]
