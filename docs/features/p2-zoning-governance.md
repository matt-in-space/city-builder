# Zoning as Governance

**Priority:** P2
**Status:** Not Started
**Depends On:** Building Spawning (p0-building-spawning), Basic Economy (p1-basic-economy)

---

## Overview

Zoning is NOT how the city starts growing — organic growth driven by roads and economic opportunity handles that. Zoning is a governance tool the player unlocks as the city grows and land use conflicts emerge. It restricts and directs development rather than creating it.

Historically, zoning laws in America took off in the 1920s precisely because cities had grown enough that land-use conflicts were becoming serious. A tannery next to houses, a factory blocking a residential street. Zoning is the player's response to these organic conflicts.

Unzoned land continues to develop based on pure economic logic. Zoned land has constraints that filter what's allowed. Zones can conflict with economic pressure, creating gameplay tension — a developer wants to build a warehouse in your residential zone, do you rezone or hold the line?

---

## Technical Details

**Unlock mechanism:** Zoning becomes available at a population threshold or through a city council event (ties into event system). The idea is the player *earns* the ability to zone by growing enough that it's needed.

**Zone painting:** Brush-based freeform painting on terrain (no grid snap). Zone types: Residential, Commercial, Industrial, Mixed-Use. Zones restrict what building types can spawn in the area — they don't force buildings to appear.

**Zone enforcement:** When the building spawning system evaluates a lot in a zoned area, it filters by zone type. A lot in a residential zone won't get a factory even if the economic logic wants one there. This can create tension events (developer petitions for rezoning).

**Unzoned land:** Continues to develop with no restrictions. The simulation uses pure economic logic to decide what goes where.

---

## Implementation Checklist

- [ ] Implement zone unlock trigger (population threshold or event-based)
- [ ] Implement brush-based zone painting on terrain
- [ ] Define zone types: Residential, Commercial, Industrial, Mixed-Use
- [ ] Store zone data as spatial overlay
- [ ] Render zones as colored terrain overlays (toggleable)
- [ ] Building spawning respects zone restrictions when zones are present
- [ ] Unzoned areas continue to develop without restriction
- [ ] Allow zone erasing and rezoning
- [ ] Surface zone conflicts as notifications (economy wants X in a Y-zoned area)

---

## Acceptance Criteria

- Zoning is not available at game start — it unlocks as the town grows
- Painting a residential zone prevents industrial buildings from spawning there
- Unzoned areas continue to develop organically
- The player can see zones as an overlay on the terrain
- Zone conflicts generate player-facing notifications or events
