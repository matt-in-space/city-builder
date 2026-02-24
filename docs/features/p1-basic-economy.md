# Basic Economy

**Priority:** P1
**Status:** Not Started
**Depends On:** Population & Immigration (p1-population-immigration), Starting Industries (p1-starting-industries)

---

## Overview

The economy is the central nervous system of the simulation. Money circulates between residents, businesses, and the city government. Residents earn wages, spend them to fulfill needs, and pay taxes. Businesses earn revenue, pay employees, and pay taxes. The city collects taxes and spends on infrastructure.

This feature covers the basic money-flow loop. Dynamic pricing and physical goods/supply chains are separate features (P2) that layer on top of this foundation. For now, the economy works with abstract "spending" rather than tracking specific goods — residents spend money to fulfill needs, and that money goes to businesses.

---

## Technical Details

**City budget:**
- `CityBudget` resource: current funds, tax rates (residential income tax, commercial/industrial tax), monthly income, monthly expenses
- Starting funds are modest — enough to build a few roads but not much more
- Revenue: taxes collected monthly from employed residents and operating businesses
- Expenses: road construction costs, building maintenance (future), municipal services (future)

**Money flow per game-month:**
1. Employed residents earn wages from their employer
2. Residents spend a portion of wages on needs (food, goods, entertainment) — money goes to commercial businesses
3. Residents pay income tax — money goes to city budget
4. Businesses pay commercial tax — money goes to city budget
5. Businesses pay wages from revenue (if revenue < wages, business struggles)
6. Industries sell goods (locally or export) to earn revenue

**Wage and price levels:**
- Different job types pay different wages (factory worker < shop clerk < skilled tradesman)
- Wages are initially fixed per job type but later influenced by labor supply/demand
- Cost of living is determined by how much residents spend to fulfill needs
- If spending > income, residents deplete savings and eventually become unhappy

**Business viability:**
- Each commercial/industrial building tracks revenue and expenses
- If a business can't cover wages for multiple months, it fails (closes, building becomes vacant)
- Business failure frees up the lot for potential redevelopment
- This is how the economy self-corrects — too many shops for the population means some close

**Construction costs:**
- Road and building construction deducts from city budget
- Costs are currently flat per type (later influenced by material prices in P2)
- If city budget hits zero, no new city-funded construction can begin (in-progress construction continues)
- Private buildings (homes, shops, factories) are funded by the economy, not the city budget

---

## Implementation Checklist

- [ ] Define `CityBudget` resource: funds, tax rates, income/expense tracking
- [ ] Define `Business` component: revenue, expenses, employee count, viability status
- [ ] Implement monthly wage payment: employers pay wages to employed residents
- [ ] Implement resident spending: residents spend portion of income to fulfill needs, money flows to local businesses
- [ ] Implement tax collection: income tax from residents, commercial tax from businesses, deposited to city budget
- [ ] Implement construction cost deduction from city budget for player-placed infrastructure
- [ ] Private building construction is economy-funded (not deducted from city budget)
- [ ] Implement business viability check: businesses that can't cover expenses for multiple months close
- [ ] Vacant buildings from closed businesses are available for new businesses
- [ ] Display budget summary in HUD: current funds, monthly income, monthly expenses, trend indicator
- [ ] Display basic economic stats: average wage, employment rate, number of businesses
- [ ] Implement budget warning when funds are low
- [ ] Block new city-funded construction when budget is zero

---

## Acceptance Criteria

- Money visibly circulates: taxes come in monthly, construction costs go out
- The city budget rises when the economy is healthy (more taxpayers, more businesses)
- The city budget falls when the economy struggles (unemployment, business closures)
- Businesses that can't sustain themselves close, leaving vacant buildings
- The player can see the economic health of their town through the HUD
- Running out of money prevents new city construction but doesn't crash the simulation
- The economy can grow or shrink based on conditions — it's not a one-way ratchet
