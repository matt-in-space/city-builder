# Lot Subdivision (Spatial Claims)

**Priority:** P0
**Status:** Not Started
**Depends On:** Road placement (complete), Map Resources (co-developed)

---

## Overview

Buildings need to claim space and not overlap each other. A "lot" is the spatial footprint a building occupies — its property boundary. This is not a pre-subdivision system that carves roads into parcels ahead of time. Instead, lots are created *when buildings are placed* as the space they claim.

The building spawning system decides *where* to place buildings (based on road access, resource proximity, demand, and terrain). This system ensures that once a building is placed, its footprint is reserved and nothing else can overlap it.

Lot size is determined by what the building needs:
- A small house claims a modest residential footprint
- A farm claims a large area because it's producing goods from the land
- A shop claims a narrow deep lot to maximize road frontage
- A factory claims a large footprint for production space

As the city densifies, lots become a more important concept. A one-story house might get demolished and replaced with a two-story building that serves more people on a smaller footprint. Lot boundaries can shrink, merge, or split as the city evolves. But all of this is emergent from economic pressure, not pre-planned.

There is no "town center" that gets placed. Dense commercial areas emerge naturally where demand concentrates — near intersections, along busy roads, close to services. Lot sizing responds to density and intended use, not distance from an arbitrary center point.

---

## Technical Details

**Lot as spatial claim:**
- When a building is spawned, it creates a `Lot` entity representing the claimed space
- The lot is a polygon boundary (initially rectangular, aligned to road orientation)
- Other buildings cannot overlap existing lots
- Lots are tied to their building — if the building is demolished, the lot is freed

**Lot sizing by building type:**
- Residential: modest footprint (house + small yard). Size varies by building scale.
- Commercial: narrow frontage, deeper lot. Right at the road edge.
- Industrial: large footprint with staging area.
- Agricultural: very large — farmhouse lot + cultivated land extending away from road. Farms are the biggest space consumers.

**Overlap prevention:**
- Before placing a building, check that the proposed lot boundary doesn't intersect any existing lot
- Simple approach: axis-aligned bounding boxes for fast rejection, polygon intersection for precise check
- Also check against roads (buildings don't overlap roads) and water

**Lot evolution (future):**
- As demand increases in an area, economic pressure can trigger demolition of low-density buildings
- Replacement buildings may have different (often smaller but taller) footprints
- Lot boundaries update when buildings are replaced
- Multiple small lots could merge for a larger building
- This is P2+ behavior but the data model should not prevent it

**Queryable spatial index:**
- Need efficient spatial queries: "is this area free?", "what lots are near this point?", "find the nearest available space along this road"
- A simple spatial grid or quadtree over lot bounding boxes
- The building spawning system uses this to find placement candidates

---

## Implementation Checklist

- [ ] Define `Lot` component: polygon boundary, area, building reference, road reference
- [ ] Implement lot creation when a building is placed (sized appropriately for building type)
- [ ] Implement lot overlap detection: check proposed lot against all existing lots
- [ ] Implement lot-road overlap check: buildings don't overlap roads
- [ ] Implement lot-water overlap check: buildings don't go in water
- [ ] Implement lot terrain check: reject lots on excessively steep terrain
- [ ] Free lot when building is demolished
- [ ] Basic spatial index for efficient "is this area free?" queries
- [ ] Debug visualization: render lot boundaries as wireframe outlines on terrain (toggleable)

---

## Acceptance Criteria

- Every building has an associated lot that represents its claimed space
- No two buildings overlap — lots enforce spatial separation
- Lots are appropriately sized for their building type (farms are large, houses are modest)
- Attempting to place a building where a lot already exists is rejected
- Demolishing a building frees its lot for future development
- Lot boundaries are visible in debug mode
- The spatial query system can efficiently answer "where is there free space near this road?"
