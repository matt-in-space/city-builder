# Building Spawning

**Priority:** P0
**Status:** Not Started
**Depends On:** Lot Subdivision (p0-lot-subdivision), Map Resources (complete)

---

## Overview

Place placeholder buildings (colored boxes) on available land along roads based on demand. This is the system that makes the city come alive — buildings appear organically where the economic logic says they should, oriented toward roads, sized appropriately for their purpose.

Demand is driven by **resources and road connectivity**. A road through timber country attracts a sawmill. Fertile land along a road attracts farms. People who work at those industries need housing, which clusters nearby. A general store appears where people congregate. Growth radiates outward from existing clusters along roads.

Initially, demand is derived from map resources and fabricated economic pressure so we can validate the placement system visually. The real economy (P1) replaces fabricated values with actual economic signals. The important thing now is that buildings appear in the right places, face the road, and create a settlement pattern that feels natural.

---

## Technical Details

**Building types (initial set, placeholder geometry):**

| Type | Footprint | Height | Color | Roof | Spawns When |
|------|-----------|--------|-------|------|-------------|
| Small house | ~30x25 ft | 15 ft | Warm tan/brown | Pitched (triangular prism) | Housing demand near jobs |
| Large house | ~50x35 ft | 20 ft | Warm cream | Pitched | Higher housing demand |
| Small shop | ~25x40 ft | 18 ft | Cool blue/teal | Flat with slight parapet | Commercial demand at intersections/clusters |
| General store | ~40x50 ft | 20 ft | Cool slate | Flat | Commercial demand along main roads |
| Workshop/small industrial | ~60x40 ft | 22 ft | Gray | Sawtooth | Near resources, road access |
| Factory | ~100x60 ft | 30 ft | Dark gray | Sawtooth | Strong resource + road access |
| Farm house | ~35x30 ft | 18 ft | Warm russet | Pitched | Road through fertile land |
| Barn/outbuilding | ~40x30 ft | 25 ft | Red-brown | Gambrel-ish (box approx) | Behind farmhouse, in farmland |

**Placement rules:**
- Buildings orient to face the road (front edge parallel to nearest road segment)
- Small setback from road for residential (front yard feel), minimal setback for commercial
- Building footprint must not overlap any existing lot, road, or water
- Terrain must be buildable (not too steep, not underwater)

**Resource-driven demand:**
- Map resources (timber, fertile land, etc.) create demand hotspots
- A road connecting to a resource creates opportunity: industry spawns to exploit the resource
- Industry creates jobs → housing demand → residential spawns nearby
- Population creates consumer demand → commercial spawns at high-traffic locations
- This chain is simplified/fabricated for P0 but follows the real economic logic

**Demand system (fabricated for P0):**
- `DemandPressure` resource with residential, commercial, industrial float values
- Resources connected to roads generate industrial demand
- Industrial buildings generate residential demand (workers need housing)
- Population clusters generate commercial demand
- Each spawn reduces demand slightly; demand regenerates over time
- This is a stopgap — P1 economy replaces it with real economic signals

**Placement selection:**
- When demand exists, find candidate locations along roads
- Score candidates by: proximity to demand source, road access quality, terrain flatness, clustering with similar buildings
- Select the best candidate and spawn the building
- Create a lot (spatial claim) for the building

**Clustering behavior:**
- New buildings prefer locations near existing buildings (gravity toward settlement)
- Growth radiates outward along roads from existing clusters
- Isolated spots far from other buildings are less desirable and develop later
- Commercial gravitates toward intersections and high-traffic road segments

**Agricultural buildings are special:**
- Farmhouses spawn on roads through fertile land
- They claim additional land behind them as "farmland" — a large area for production
- Farmland is visually distinct and reserves space from other development
- Barns/outbuildings spawn within the farmland area

---

## Implementation Checklist

- [ ] Define `Building` component: building type, footprint dimensions, height, construction state
- [ ] Define `BuildingType` enum: SmallHouse, LargeHouse, SmallShop, GeneralStore, Workshop, Factory, FarmHouse, Barn
- [ ] Define `DemandPressure` resource with residential, commercial, industrial float values
- [ ] Create placeholder mesh generation for each building type (boxes with roofs and colors)
- [ ] Implement building placement system: find candidates along roads, score, select best, spawn
- [ ] Orient buildings to face the nearest road
- [ ] Apply setback rules by building type
- [ ] Integrate with lot system: create spatial claim, check for overlaps before placing
- [ ] Implement resource-driven demand: resources + road connectivity → industrial demand → residential demand → commercial demand
- [ ] Implement clustering preference: candidates near existing buildings score higher
- [ ] Implement gradual growth: buildings spawn one at a time over game-ticks, not all at once
- [ ] Implement farmland claim: farmhouses reserve large area for agricultural use
- [ ] Place barns/outbuildings in farmland area
- [ ] Fabricated demand: initial values, slow regeneration, decrease on spawn

---

## Acceptance Criteria

- Buildings appear organically along roads over time
- Buildings near resources are industrial; housing clusters near industry; shops appear at intersections
- Buildings face the road with appropriate setbacks
- No buildings overlap each other, roads, or water
- Farms appear along roads through fertile land with large farmland claims
- Growth radiates outward from initial clusters along roads
- The settlement pattern looks like a natural small town forming along roads
- Growth is gradual — watching buildings fill in over game-minutes feels satisfying
