# Market Pricing

**Priority:** P2
**Status:** Not Started
**Depends On:** Goods & Imports (p2-goods-imports)

---

## Overview

Prices for goods respond dynamically to supply and demand, creating feedback loops throughout the economy. High lumber prices make construction expensive, which slows growth, which reduces demand, which eventually brings prices back down. Low food prices from abundant local farms attract more residents, increasing demand, which raises prices until a new farm opens. The economy breathes.

---

## Technical Details

**Price formula:** price = base_cost × (demand / supply), with temporal smoothing to prevent wild oscillations. Prices update monthly.

**Feedback loops:**
- High material prices → expensive construction → slower growth → less demand → prices ease
- Low food prices → more immigration → more demand → prices rise → more farms needed
- Local production entering the market → supply increase → price drop → import substitution

**Substitution:** When one material is expensive, construction can substitute where possible (brick instead of lumber, stone instead of brick). This adds resilience and interesting tradeoffs.

---

## Implementation Checklist

- [ ] Implement supply/demand price calculation with smoothing
- [ ] Construction costs dynamically reflect current material prices
- [ ] Residents adjust spending based on prices (reduce consumption of expensive goods)
- [ ] Implement material substitution in construction (prefer cheaper materials when possible)
- [ ] Display price trends in UI (current price per good, trend arrows)
- [ ] Price spikes trigger economic events or notifications

---

## Acceptance Criteria

- Prices visibly respond to supply/demand changes
- Construction costs fluctuate with material prices
- Local production measurably reduces prices for those goods
- The economy self-corrects through price signals (no runaway inflation or deflation under normal conditions)
