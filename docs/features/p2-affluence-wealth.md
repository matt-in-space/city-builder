# Affluence & Wealth System

**Priority:** P2
**Status:** Not Started
**Depends On:** Population & Immigration (p1-population-immigration), Basic Economy (p1-basic-economy)

---

## Overview

Residents have varying wealth levels that emerge from their income and savings. Wealth determines what housing they can afford, which creates organic neighborhood stratification — wealthier residents gravitate toward desirable locations (hilltops, waterfronts, quiet streets) while workers cluster near factories and affordable areas.

Affluence also affects political influence. Wealthy neighborhoods have more power to resist unwanted development (a tannery near mansions triggers a crisis; near worker cottages, it's a minor grumble). This drives realistic governance dynamics where the player must weigh economic growth against the interests of different classes.

---

## Technical Details

**Wealth tiers:** Working class, middle class, upper class — derived from income and savings, not assigned. Wealth affects: housing quality sought, spending patterns, political influence weight, tolerance for nearby nuisances (noise, pollution).

**Housing quality:** Buildings have a quality level. Higher quality = larger, more expensive, on better land. Residents seek housing matching their wealth tier. Mismatch creates pressure — a wealthy resident in a modest home looks to upgrade; a struggling resident in expensive housing risks eviction.

**Neighborhood emergence:** Wealth clusters geographically as residents self-sort by what they can afford. Desirable locations (views, parks, quiet) attract wealth. Locations near industry attract workers. This happens organically from the simulation, not from player designation.

**Political influence:** When land-use conflicts arise (industry near homes), the affected residents' wealth tier determines the severity of the political response. This feeds into the event system.

---

## Implementation Checklist

- [ ] Add wealth/savings tracking to residents and cohorts
- [ ] Define housing quality levels tied to building type and location desirability
- [ ] Residents seek housing matching their wealth tier
- [ ] Implement location desirability scoring (elevation, water proximity, distance from industry, noise)
- [ ] Wealth tier affects political influence weight in events and conflict resolution
- [ ] Visual indicator of neighborhood wealth (housing size/quality variation)
- [ ] Implement upward/downward mobility: residents can change wealth tier over time based on economic conditions

---

## Acceptance Criteria

- Wealthier residents cluster in desirable locations; workers cluster near jobs
- Neighborhoods develop distinct economic character without player designation
- Land-use conflicts near wealthy areas generate stronger political responses than near working-class areas
- Housing quality visibly varies across the town (larger vs. smaller buildings in different areas)
