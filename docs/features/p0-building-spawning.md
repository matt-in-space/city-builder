# Building Spawning

**Priority:** P0
**Status:** Not Started
**Depends On:** Lot Subdivision (p0-lot-subdivision)

---

## Overview

Place placeholder buildings (colored boxes) on available lots based on demand. This is the system that makes the city come alive — buildings appear organically where the economic logic says they should, oriented toward roads, sized appropriately for their lots.

Initially, demand is fabricated (simple pressure values) so we can validate the placement system visually. The real economy replaces this fabricated demand later (P1). The important thing now is that buildings appear in the right places, look reasonable, face the road, and create a settlement pattern that feels natural rather than grid-like.

Buildings should cluster near the founding point / town center, thin out along roads, and fill in organically over time — frontage lots first, back lots later. The result should look like a small settlement forming along a road, not a uniform grid of boxes.

---

## Technical Details

**Building types (initial set, placeholder geometry):**

| Type | Footprint | Height | Color | Roof | Typical Lot |
|------|-----------|--------|-------|------|-------------|
| Small house | ~30×25 ft | 15 ft | Warm tan/brown | Pitched (triangular prism) | Standard residential |
| Large house | ~50×35 ft | 20 ft | Warm cream | Pitched | Large or merged lot |
| Small shop | ~25×40 ft | 18 ft | Cool blue/teal | Flat with slight parapet | Corner lot or main road frontage |
| General store | ~40×50 ft | 20 ft | Cool slate | Flat | Main road frontage |
| Workshop/small industrial | ~60×40 ft | 22 ft | Gray | Sawtooth | Near resources |
| Factory | ~100×60 ft | 30 ft | Dark gray | Sawtooth | Large lot near resources |
| Farm house | ~35×30 ft | 18 ft | Warm russet | Pitched | Along road near fertile land |
| Barn/outbuilding | ~40×30 ft | 25 ft | Red-brown | Gambrel-ish (box approx) | Behind farmhouse |

**Placement rules:**
- Buildings orient to face the road (front edge parallel to the nearest road segment)
- Small setback from road for residential (front yard feel), minimal setback for commercial (right at the sidewalk)
- Building footprint must fit within the lot boundary
- Buildings don't overlap each other
- One primary building per lot (outbuildings like barns are secondary and placed behind the primary)

**Demand system (fabricated for now):**
- A simple `DemandPressure` resource with values for residential, commercial, and industrial demand
- Each game tick, if demand > 0, the system looks for available lots matching that demand type
- Selects the most desirable available lot (closest to town center, best road frontage, flattest terrain)
- Spawns an appropriately typed building
- Demand decreases slightly after each spawn (so growth is gradual, not instant)
- Demand slowly regenerates over time (simulating ongoing immigration/economic pressure)
- This is a stopgap — the real economy (P1) will replace these fabricated values with actual economic signals

**Agricultural buildings are special:**
- Farmhouses spawn on lots near fertile land
- They claim additional land behind them as "farmland" — a large rectangular area extending away from the road
- Farmland is visually distinct (lighter green, possibly with row textures later) and reserves the space from other development
- Barns and outbuildings spawn in the farmland area near the farmhouse, not on separate lots

**Clustering behavior:**
- New buildings prefer lots near existing buildings (gravity toward existing settlement)
- First buildings cluster near the founding point / entry point
- Growth radiates outward along roads from existing clusters
- Isolated lots far from other buildings are less desirable and develop later

---

## Implementation Checklist

- [ ] Define `Building` component: building type, footprint dimensions, height, construction state
- [ ] Define `BuildingType` enum with initial types: SmallHouse, LargeHouse, SmallShop, GeneralStore, Workshop, Factory, FarmHouse, Barn
- [ ] Define `DemandPressure` resource with residential, commercial, industrial float values
- [ ] Create placeholder mesh generation for each building type (boxes with appropriate roofs and colors)
- [ ] Implement building placement system: query available lots, select best candidate, spawn building entity
- [ ] Orient buildings to face the nearest road (front edge parallel to road tangent at nearest point)
- [ ] Apply setback rules: residential gets a small front yard, commercial sits at road edge
- [ ] Verify building footprint fits within lot boundary before placing
- [ ] Mark lot as occupied when building is placed
- [ ] Implement clustering preference: lots near existing buildings score higher in desirability
- [ ] Implement gradual growth: buildings spawn one at a time over game-ticks, not all at once
- [ ] Implement farmland claim: farmhouses reserve additional land behind them, rendered as a distinct ground overlay
- [ ] Place barns/outbuildings in farmland area near the farmhouse
- [ ] Fabricated demand: configure initial demand values, slow regeneration, decrease on spawn
- [ ] Ensure buildings are visible and identifiable from the default camera view

---

## Acceptance Criteria

- Buildings appear organically along roads over time without player intervention (beyond having built roads)
- Buildings face the road and have appropriate setbacks by type
- Residential buildings cluster in groups that feel like a small neighborhood
- Commercial buildings appear at intersections and along main roads
- Farms appear along roads near fertile land, with visible farmland extending behind them
- Growth radiates outward from the founding area along roads
- No buildings appear on water, steep slopes, or overlapping other buildings
- The settlement pattern looks like a natural small town, not a grid
- Growth is gradual — watching the town fill in over several game-minutes feels satisfying
- Demand can be tweaked via the `DemandPressure` resource for testing
