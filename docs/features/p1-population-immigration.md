# Population & Immigration

**Priority:** P1
**Status:** Not Started
**Depends On:** Building Spawning (p0-building-spawning), Construction Pipeline (p0-construction-pipeline)

---

## Overview

People are the lifeblood of the city. Residents arrive from the outside world, seek housing and employment, fulfill their needs through the local economy, and leave if conditions are unacceptable. At small scale, each resident is a discrete simulated entity. The transition to statistical modeling at larger populations is a separate future concern.

Immigration happens through the town's connection to the outside world — the road leading off the map edge. People arrive at this entry point and look for a place in the town. The rate of immigration is driven by the town's attractiveness: available jobs, available housing, wage levels, and general quality of life. No one moves to a town with no jobs and no houses.

---

## Technical Details

**Resident entity components:**
- Name (procedurally generated, era-appropriate)
- Household ID (group of residents living together)
- Employment: employer entity reference, job type, wage
- Housing: building entity reference
- Needs: food (0-100), shelter (0-100), goods (0-100) — decay over time, replenished by access to services
- Happiness: derived from need fulfillment, employment, housing quality
- Wealth/savings: accumulated wages minus spending (simple float for now)

**Immigration mechanics:**
- Immigration pressure = f(available jobs, available housing, average wages, town reputation)
- Each game-tick, a probability check determines if new residents arrive
- New residents appear at the map-edge entry point
- They seek housing first, then employment
- If they can't find both within a grace period (e.g., 2-3 game-months), they leave
- Arrivals come in small groups (1-4 people, representing individuals or families)

**Resident behavior loop (per game-tick or periodic):**
1. If unemployed, seek job (nearest available employer with open positions)
2. If employed, earn wages
3. Spend wages to fulfill needs (food from shops, goods from stores)
4. Need fulfillment decays over time
5. Evaluate happiness — if persistently low, consider leaving

**Departure:** Residents leave if happiness stays below a threshold for too long. They vacate their housing (freeing the building) and their job (opening a position). Departure should be visible — the house is now empty, the employer has an unfilled position.

**Entry point:** Initially just the one road connection to the map edge. Later, additional connections (new roads to map edge, rail depots, etc.) add more entry points and increase immigration capacity.

---

## Implementation Checklist

- [ ] Define `Resident` entity with components: name, household, employment, housing, needs, happiness, wealth
- [ ] Implement procedural name generation (era-appropriate first and last names)
- [ ] Define `Household` component to group residents (families, roommates)
- [ ] Implement immigration system: calculate attractiveness, probabilistic arrival each tick
- [ ] New residents spawn at map-edge entry point
- [ ] Implement housing search: new residents look for unoccupied residential buildings
- [ ] Implement job search: residents seek employment at commercial/industrial buildings with openings
- [ ] Implement need decay: food, shelter, goods fulfillment decrease over time
- [ ] Implement need fulfillment: residents "visit" commercial buildings to replenish needs (abstracted, not pathfinding)
- [ ] Calculate happiness from need fulfillment and employment status
- [ ] Implement departure: residents with persistently low happiness leave, freeing housing and jobs
- [ ] Update HUD population counter to reflect actual resident count
- [ ] Show resident info when clicking on a residential building (who lives here, their employment, happiness)

---

## Acceptance Criteria

- Residents arrive over time when jobs and housing are available
- Immigration stops when there are no available jobs or housing
- Residents have visible names, jobs, and needs when inspected
- Residents leave if they can't find work or housing, or if happiness drops too low
- Population count in the HUD accurately reflects the number of residents
- A town with more jobs and housing attracts more immigrants than one with fewer
- Empty houses and unfilled jobs are visible signals that something is wrong or that there's room to grow
