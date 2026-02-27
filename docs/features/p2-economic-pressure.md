# Economic Pressure & Industry Growth

**Priority:** P2
**Status:** Not Started
**Depends On:** Basic Economy (p1-basic-economy), Starting Industries (p1-starting-industries)

---

## Overview

The current economy uses hard thresholds to decide whether an industry can exist — a resource cell either passes a richness check or it's invisible. Real economies don't work this way. A desperate entrepreneur will set up a small logging operation on marginal land if demand is strong enough, while a rich timber stand might go untouched if nobody needs lumber.

This feature replaces binary viability with a continuous model of **economic pressure**: the gap between demand and supply creates opportunity, and industries respond to that opportunity with varying degrees of ambition and competence. Growth isn't automatic — it requires sustained demand *and* sufficient resources. The result is an economy that feels organic rather than mechanical.

---

## Technical Details

### Soft resource thresholds

Currently `is_producer_viable` rejects any cell with `richness <= 0.2`. Instead:

- **All resource cells are candidates**, regardless of richness
- Richness affects **production capacity**, not existence — a camp at 0.1 richness produces little, a camp at 0.9 produces a lot
- Low-richness sites get poor scores in candidate evaluation, so they're only chosen when better options are exhausted
- The scoring function should weight richness heavily but not absolutely — a 0.15 site near an established road beats a 0.8 site with no infrastructure

### Economic pressure as the spawn driver

Rather than checking "is this resource type viable?" → "find candidates" → "spawn best", the loop should be:

- Compute **demand** for each good (e.g., timber demand from construction activity, population growth)
- Compute **current supply** from existing industries
- The **pressure** = demand - supply
- High pressure lowers the bar for what's considered a viable site
- Zero or negative pressure means no new industry spawns, regardless of available resources

### Growth vs. spawning

Separate the concepts:

- **Spawning** = a new business opens. Small initial capacity. Driven by economic pressure.
- **Growth** = an existing business expands capacity. Requires sustained demand + available resources (richness headroom).
- A camp at richness 0.1 can spawn but has almost no growth potential — it's a small operation that stays small.
- A camp at richness 0.8 can grow substantially if demand supports it.
- Growth could mean more workers, higher output, or physically larger footprint (later milestone).

### Competition and diminishing returns

- Multiple industries extracting the same resource in the same area split the effective richness
- The 1st logging camp in a timber zone gets full benefit; the 3rd is fighting for scraps
- Industries further from the resource center get diminishing returns
- This naturally limits clustering without hard caps

### Simulating imperfection

Not every economic decision should be optimal:

- Add noise to candidate scoring so "good enough" sites sometimes win over optimal ones
- This models bounded rationality — real people don't have perfect information
- Magnitude should be tunable: maybe ±10-20% score variation
- Occasionally an industry picks a mediocre site, giving the economy character and variety

### Open questions

- **Money for growth:** Does expanding an industry require capital (savings from profit)? Or is growth purely demand + resource driven? Capital adds realism but also complexity. Could start simple (demand + resources) and add capital later.
- **Supply chains:** Timber → lumber → construction. Does a sawmill need to exist before timber demand creates building construction? How deep do chains go at this stage?
- **Failure and bankruptcy:** If an industry on a marginal site can't cover operating costs, does it shut down? This creates natural churn — marginal operations open in booms, close in busts.
- **Individual-to-statistical transition:** At < 500 pop, industries are individual businesses with named owners. Above 500, do they become statistical aggregates? Or do industries always stay individual (since there are few of them)?
- **Export as pressure release:** If local demand is low but the town has road/rail connections, can industries produce for export? This would let resource-rich towns develop industries beyond local need.

---

## Implementation Checklist

- [ ] Replace hard richness threshold with continuous scoring (richness as weight, not gate)
- [ ] Define demand/supply model: what generates demand for each resource/good, how supply is tracked
- [ ] Compute economic pressure per good type each evaluation tick
- [ ] Pressure-based spawn threshold: higher pressure → lower minimum score for viable candidates
- [ ] Add score noise to candidate evaluation (tunable imperfection factor)
- [ ] Implement industry growth: separate system that evaluates existing industries for expansion
- [ ] Growth gated by sustained demand (not just instantaneous) + resource headroom
- [ ] Competition model: nearby same-type industries reduce effective richness for each other
- [ ] Business failure: industries on marginal sites close when pressure drops

---

## Acceptance Criteria

- Industries spawn on marginal resource sites when demand is high, not just on the richest deposits
- A logging camp at low richness stays small; one at high richness grows over time with demand
- Multiple industries in the same resource zone compete — returns diminish naturally
- When demand drops, marginal industries fail before established ones
- Industry placement has visible variety — not every camp is in the mathematically optimal spot
- The economy feels responsive: building a road to new resources creates visible economic activity proportional to both the resource quality and the current demand
