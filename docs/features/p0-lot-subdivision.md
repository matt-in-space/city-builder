# Lot Subdivision

**Priority:** P0
**Status:** Not Started
**Depends On:** Road placement (Milestones 3-4, complete)

---

## Overview

Generate buildable lots along roads without requiring zones. This is the system that answers "where can buildings go?" in the absence of traditional zone painting. Since the game's early phase has no zoning laws — just organic growth — lots are generated purely based on road frontage.

The core idea: roads are economic arteries. Land adjacent to roads is desirable and developable. The lot subdivision system identifies buildable parcels along the road network, making them available for the building spawning system to place structures on.

This replaces the originally planned "zone painting → lot subdivision" flow. Zones become a later governance tool (P2) used to *restrict and direct* growth, not to initiate it.

---

## Technical Details

**Road frontage as the organizing principle.** For each road segment, the system projects potential lots outward from both sides of the road. Each lot gets a slice of road frontage and extends perpendicular to the road to a configurable depth.

**Lot generation algorithm:**
1. Walk each road segment, sampling at intervals along the spline
2. At each sample point, project outward perpendicular to the road on both sides
3. Generate rectangular-ish lot footprints (frontage width × depth)
4. Check terrain constraints: reject lots that are too steep, underwater, or overlapping existing structures
5. Check for overlap with other lots and resolve conflicts
6. Store lots as entities with spatial data and metadata

**Lot properties:**
- Polygon boundary (the actual shape of the lot)
- Road frontage length
- Total area
- Average slope (sampled from terrain heightmap)
- Distance to nearest road (should be near-zero for frontage lots)
- Occupied flag (whether a building exists here)
- Building type affinity (derived from context — see below)

**Context-sensitive lot sizing:**
- Near the founding point / town center: smaller lots (25-50 ft frontage, 80-120 ft depth) — denser, more urban
- Further out along roads: larger lots (60-100 ft frontage, 120-200 ft depth) — more spacious, rural feel
- Near industrial areas or resource sites: larger, more irregular lots for bigger structures
- Lot size could also be influenced by road type — dirt road generates larger rural lots, paved road generates smaller urban lots

**Building type affinity** (what kind of building is likely here):
- Lots at intersections / corners: weighted toward commercial (two-street frontage, foot traffic)
- Lots near resource sites: weighted toward industrial
- Lots in clusters near other residential: weighted toward residential
- Lots along main arteries: weighted toward commercial

**Road change handling:** When roads are added, removed, or modified, the lot system needs to regenerate affected lots. This should be event-driven (trigger on road network changes) rather than running every frame.

**Performance:** Lot generation only runs when roads change, not every tick. Lots are static entities until something triggers regeneration. The number of lots scales with road network size, which grows gradually.

---

## Implementation Checklist

- [ ] Define `Lot` component/entity: polygon boundary, area, frontage length, slope, road reference, occupied flag, building affinity
- [ ] Define `LotConfig` resource: default lot dimensions (frontage width, depth), slope threshold, minimum area
- [ ] Implement lot generation along a single road segment: walk the spline, project lots outward on both sides
- [ ] Handle lot depth — project perpendicular to road, clip to terrain constraints (water, excessive slope)
- [ ] Handle lot-to-lot overlap detection and resolution (lots from different road segments shouldn't overlap)
- [ ] Skip lot generation where existing buildings or other structures already occupy space
- [ ] Implement context-sensitive sizing: lot dimensions vary based on distance from town center and road type
- [ ] Assign building type affinity to lots based on location context (corner, near resources, along main road, etc.)
- [ ] Wire up lot regeneration to trigger when roads are added or modified
- [ ] Debug visualization: render lot boundaries as wireframe outlines on terrain (toggleable)
- [ ] Debug visualization: color-code lots by affinity (warm = residential, cool = commercial, gray = industrial)

---

## Acceptance Criteria

- When a road is placed, lots automatically generate along both sides of the road
- Lots have visible debug boundaries that show their shape and type affinity
- Lots don't appear on water, excessively steep terrain, or overlapping existing structures
- Lots near town center are smaller/denser than lots further out
- Corner lots at intersections are identifiable and weighted toward commercial
- Adding a new road generates new lots; lots update when roads change
- Lot data is queryable by the future building spawning system (can ask "give me available lots sorted by desirability")
