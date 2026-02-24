# Goods & Import System

**Priority:** P2
**Status:** Not Started
**Depends On:** Basic Economy (p1-basic-economy), Starting Industries (p1-starting-industries)

---

## Overview

Replace abstract "spending" with physical goods that flow through supply chains. Every construction project needs specific materials (lumber, brick, stone). Residents need food and consumer goods. Goods are either imported through the town's external connection (expensive) or produced locally by industries (cheaper). When supply can't meet demand, prices rise, construction stalls, and needs go unmet.

This is what makes the economy feel real rather than being a facade of top-level stats.

---

## Technical Details

**Initial goods:** Lumber, Brick, Stone, Coal, Food, Consumer Goods

Each good has: base import cost, transport markup, local supply rate, local demand rate, current local price, and inventory level.

**Import system:** Goods flow in through the map-edge connection at base cost + markup. Import capacity is limited by the connection throughput (a single dirt road can only carry so much freight). This creates natural bottlenecks that pressure the player to develop local production or improve transport infrastructure.

**Local production:** Industries produce goods that enter local inventory. Local goods skip the transport markup, making them cheaper. A sawmill producing lumber reduces the town's dependence on imported lumber.

**Consumption:** Construction projects consume materials (a house needs lumber and brick). Residents consume food and goods. When inventory runs low, construction stalls and needs go unfulfilled.

---

## Implementation Checklist

- [ ] Define `Good` enum and `GoodsInventory` resource tracking supply, demand, price, and stock level per good
- [ ] Define import pipeline: goods flow in each tick based on demand, limited by connection capacity
- [ ] Construction projects declare material requirements; construction stalls if materials unavailable
- [ ] Industries add to local supply of their produced good
- [ ] Residents consume food and goods from local inventory
- [ ] Implement basic price calculation: price = base cost × (demand / supply) with smoothing
- [ ] Display goods availability, prices, and import/local split in a UI panel
- [ ] Implement import capacity limit tied to the quality/type of map-edge connection

---

## Acceptance Criteria

- Construction visibly stalls when materials are unavailable
- Local industry reduces import costs for the goods it produces
- Goods prices fluctuate based on supply and demand
- The player can see which goods are scarce and which are abundant
- Import bottlenecks create pressure to develop local production
