# Agricultural Land Use

**Priority:** P2
**Status:** Not Started
**Depends On:** Starting Industries (p1-starting-industries), Building Spawning (p0-building-spawning)

---

## Overview

Agriculture has a fundamentally different spatial footprint than other development. A sawmill is a building on a lot. A farm is a building plus *acres* of cultivated land. Early game in fertile areas should produce the classic 1920s rural American landscape — roads cutting through open land with farmhouses spaced out along them, fields stretching behind, barns and fences.

As the town grows and land values rise, agricultural land faces economic pressure from development. Farmland near the center eventually gets sold and subdivided for housing — the real-world process of towns consuming their surrounding farmland. This should happen organically through the economic simulation.

---

## Technical Details

**Farm footprint:** Farmhouse sits on a standard lot facing the road. Behind it, a large rectangular "farmland" claim extends away from the road (several acres). Barns and outbuildings sit within this claim. The farmland area is visually distinct (cultivated rows, lighter green) and is reserved from other development.

**Farmland pressure:** As surrounding land values increase (more population, more demand), farms face economic pressure. The land is worth more as housing lots than as farmland. At some threshold, the farm may sell and the land gets subdivided. This is emergent, not scripted.

**Sprawl pattern:** Multiple farms along a road through fertile land create a sprawling agricultural corridor. This is visually distinct from the tighter clustering near the town center.

---

## Implementation Checklist

- [ ] Implement farmland claim system: farms reserve large areas behind the farmhouse
- [ ] Render farmland as visually distinct terrain overlay (cultivated appearance)
- [ ] Place outbuildings (barns, sheds) within farmland area
- [ ] Farms space themselves along roads through fertile land (not clustered tightly)
- [ ] Implement land value pressure: rising values around farms create sell/subdivide pressure
- [ ] When a farm is sold, farmland becomes available for lot subdivision and development
- [ ] Agricultural production ties into food supply for the economy

---

## Acceptance Criteria

- Roads through fertile land develop a sprawling agricultural character with spaced-out farms
- Farmland is visually distinct from undeveloped and developed land
- As the town grows toward farms, economic pressure causes some to sell and be replaced by housing
- The transition from rural to suburban development happens gradually and visibly
