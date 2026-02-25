# Manually Placed Buildings

**Priority:** P3
**Status:** Not Started
**Depends On:** Building spawning, construction pipeline, basic economy, road network

---

## Overview

Player-placed civic and special buildings — hospitals, police stations, fire departments, schools, parks, city hall. These are the buildings the player directly controls, as opposed to the organically spawning residential, commercial, and industrial buildings.

These structures provide services that improve quality of life, attract population, and affect property values in their radius. They represent the player's most direct tool for shaping the city beyond infrastructure and policy.

---

## Technical Details

**Placement flow:**
- Player selects a building type from a construction menu
- Ghost/preview of the building follows the cursor on terrain
- Game validates placement: sufficient flat terrain, minimum lot size, road access
- If placed away from a road, the construction plan automatically includes a connector road to the nearest existing road (factored into cost and time)
- Placement enters the standard construction pipeline (Planned → Under Construction → Complete)

**Service radius:**
- Each civic building has an effect radius
- Properties and residents within the radius receive the service benefit
- Overlapping radii don't double-stack (diminishing returns or no stacking)
- Service coverage gaps are visible in an overlay toggle

**Initial building set:**
- **Fire Station** — reduces fire risk in radius, responds to fire events
- **Police Station** — reduces crime in radius, responds to crime events
- **Hospital/Clinic** — provides healthcare, improves happiness and population retention
- **School** — educates children, improves workforce quality over time
- **Park/Green Space** — improves happiness, increases nearby property values
- **City Hall** — administrative center, unlocks governance features

**Cost and upkeep:**
- Construction cost deducted from city budget (uses materials from economy)
- Monthly upkeep cost for staffing and maintenance
- Understaffed buildings (not enough workers in town) operate at reduced effectiveness

---

## Implementation Checklist

- [ ] Define civic building types with properties: footprint, cost, upkeep, service radius, staffing needs
- [ ] Implement placement tool: ghost preview, validation, road-access check
- [ ] Auto-generate connector road if placed away from road network
- [ ] Integrate with construction pipeline (Planned → Under Construction → Complete)
- [ ] Implement service radius system: buildings provide benefits within their radius
- [ ] Add service coverage overlay (toggleable) showing covered vs uncovered areas
- [ ] Deduct construction cost from city budget; apply monthly upkeep
- [ ] Staffing system: civic buildings need workers, reduced effectiveness if understaffed
- [ ] Placeholder geometry for each civic building type (distinct from organic buildings)

---

## Acceptance Criteria

- Player can select a civic building and place it on valid terrain
- Ghost preview shows before placement with valid/invalid indicators
- If placed away from a road, a connector road is automatically planned
- Civic buildings enter the construction pipeline and take time to build
- Completed civic buildings provide measurable benefits in their service radius
- Service overlay clearly shows coverage gaps
- Budget reflects construction costs and ongoing upkeep
