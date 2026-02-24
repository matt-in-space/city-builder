# Starting Industries

**Priority:** P1
**Status:** Not Started
**Depends On:** Map Resources (p1-map-resources), Building Spawning (p0-building-spawning), Lot Subdivision (p0-lot-subdivision)

---

## Overview

Industries are the economic engine of the town. Unlike residential and commercial buildings that spawn based on population demand, industries spawn based on resource availability and economic opportunity. Each industry type ties to a specific map resource, employs workers, produces goods, and contributes to the local economy.

The initial set is small — roughly 5 industry types — but each is a *specific* business, not a generic building. You won't see 20 identical factories. A second sawmill only appears if lumber demand exceeds what the first can produce. Every industry exists because the economy needs it.

Industries are what differentiate one town from another. A town near timber becomes a lumber town. A town near coal becomes a mining town. A town in fertile plains becomes an agricultural hub. The player influences this through road placement (connecting to resources) but the economy decides what opens and when.

---

## Technical Details

**Initial industry types:**

| Industry | Resource Required | Workers | Produces | Building Size |
|---|---|---|---|---|
| Sawmill | Timber (road access to forest) | 10-15 | Lumber | Medium industrial |
| Brick Works | Clay (road access to clay deposits) | 10-15 | Brick | Medium industrial |
| Coal Mine | Coal (road access to coal deposit) | 30-50 | Coal (fuel, export) | Large industrial |
| Quarry | Stone (road access to rock) | 10-20 | Stone (construction material) | Medium-large industrial |
| Farm | Fertile Land (along road near fertile area) | 3-5 per farm | Food, agricultural goods | Farmhouse + farmland |

**How industries spawn:**
- The simulation periodically evaluates economic opportunities: is there an accessible resource that could support an industry?
- "Accessible" means connected to the road network and within reasonable distance of the settlement
- Spawn probability increases with demand for the industry's output (e.g., high lumber demand from construction activity increases sawmill spawn chance)
- Industries spawn on appropriate lots — typically larger lots near the resource they harvest
- Each industry is a named entity ("Ridgemont Sawmill," "Valley Brick & Tile") — generated procedurally

**Industry economics:**
- Industries pay wages to workers (pulls from business revenue)
- Industries sell their goods locally (reducing import costs) and can export surplus
- Local production is cheaper than imports — this is a major economic incentive
- Industries consume the resource over time (timber stands shrink, coal seams deplete — slowly)
- Industries need equipment and supplies, some of which may be imported

**Farms are special:**
- Farms have a large land-use footprint (farmhouse + fields)
- Multiple farms can develop along a road through fertile land, creating a sprawling agricultural area
- Farms produce food, which is a basic need for all residents
- Agricultural land is visually distinct from undeveloped terrain

**Industry placement:**
- Near their resource (sawmill near the forest edge, quarry at the rock face)
- Along roads (need transport for goods)
- Set back from residential areas when possible (noise, pollution)
- The simulation should prefer locations that minimize conflict with existing residential buildings

---

## Implementation Checklist

- [ ] Define `Industry` component: industry type, workers employed (current/max), production rate, resource consumed, goods produced
- [ ] Define `IndustryType` enum: Sawmill, BrickWorks, CoalMine, Quarry, Farm
- [ ] Implement industry opportunity evaluation: periodically check for accessible resources that could support a new industry
- [ ] Implement industry spawning on appropriate lots near resources
- [ ] Generate procedural names for industries (surname + industry type)
- [ ] Industries employ residents — residents seek jobs at industries based on proximity and wages
- [ ] Industries produce goods that enter the local economy (ties into Goods & Import system when available; stubbed for now)
- [ ] Industries consume their associated resource over time (reduce resource quantity)
- [ ] Farm-specific: claim farmland area, render as distinct terrain overlay, place farmhouse + outbuildings
- [ ] Ensure industries don't spawn redundantly (no second sawmill if the first meets demand)
- [ ] Placeholder visuals: industrial building types (medium/large gray boxes with sawtooth roofs) placed near resources

---

## Acceptance Criteria

- Building a road to a timber stand eventually causes a sawmill to appear near the forest edge
- Building a road through fertile land causes farms to develop along it with visible farmland
- Industries employ residents and are visible employers in the economy
- Each industry has a generated name and feels like a specific business
- Different maps with different resources lead to different industry compositions
- Industries don't over-duplicate — the simulation doesn't spawn more than demand justifies
- The player's road placement decisions directly influence which industries develop
