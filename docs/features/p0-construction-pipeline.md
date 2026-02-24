# Construction Pipeline

**Priority:** P0
**Status:** Not Started
**Depends On:** Building Spawning (p0-building-spawning)

---

## Overview

Buildings and roads don't appear instantly — they go through a construction process. This is one of the game's core pillars: construction as a visible, time-consuming process that makes the city feel alive and grounded.

For the initial implementation, this is a visual state system: buildings transition from planned (translucent) to under construction (translucent with indicator) to complete (solid). Construction takes real game-time. Later (P1+), construction will tie into the economy — requiring materials and labor, stalling when supplies run out — but for now, it's purely time-based.

Roads placed by the player also go through this pipeline. The player plans a road, it shows as translucent/dashed, construction proceeds over time, and it becomes solid when complete. Buildings can only spawn on lots along completed roads.

---

## Technical Details

**Construction states:**
- `Planned` — the building or road has been queued but construction hasn't started. Visual: translucent/ghosted version of the final geometry.
- `UnderConstruction` — work is actively happening. Visual: translucent geometry with a color shift or simple scaffolding indicator (e.g., wireframe box slightly larger than the building, or a distinct construction color).
- `Complete` — fully built and functional. Visual: solid, opaque geometry with final colors.

**Time-based progression:**
- Each building/road has a `construction_duration` based on its type and size
- A timer ticks down each game-tick while in `UnderConstruction` state
- When timer reaches zero, state transitions to `Complete`
- Transition from `Planned` to `UnderConstruction` is immediate for now (later gated by material/labor availability)

**Construction duration guidelines (game-time):**
- Small house: ~2-4 weeks
- Large house: ~4-8 weeks
- Small shop: ~3-6 weeks
- Factory: ~2-4 months
- Road segment: ~1-2 weeks per unit length (scales with road length)

These are tuning values and should be easily adjustable.

**Interaction with building spawning:**
- When the building spawning system decides to place a building, it creates the entity in `Planned` state
- The construction system picks it up and transitions it through states
- The building only becomes functional (provides housing, jobs, etc.) when `Complete`
- Lots are marked as occupied as soon as a building is `Planned` (no double-booking)

**Interaction with roads:**
- Player-placed roads start as `Planned` (translucent) when the player confirms placement
- Construction proceeds automatically over time
- Lots only generate along `Complete` roads — buildings won't spawn along roads that are still under construction
- This creates a natural flow: player places road → waits for construction → buildings start appearing

**Rendering approach:**
- Use material alpha/opacity to distinguish states
- `Planned`: alpha ~0.3, possibly with a blueprint-blue tint
- `UnderConstruction`: alpha ~0.6, with a construction-orange or yellow tint
- `Complete`: alpha 1.0, normal building colors
- Later we can add scaffolding meshes, construction vehicles, etc. but for now, opacity and color tint are sufficient

---

## Implementation Checklist

- [ ] Define `ConstructionState` enum: Planned, UnderConstruction, Complete
- [ ] Define `ConstructionProgress` component: current state, elapsed time, total duration
- [ ] Assign default construction durations per building type and road type
- [ ] Create materials for each construction state (translucent blue for planned, translucent orange for under construction, solid for complete)
- [ ] Implement state transition system: Planned → UnderConstruction → Complete based on elapsed game time
- [ ] Apply construction states to spawned buildings: new buildings start as Planned
- [ ] Apply construction states to player-placed roads: new roads start as Planned
- [ ] Swap materials on entities when construction state changes
- [ ] Buildings only become functional (countable as housing, employable, etc.) when Complete
- [ ] Lots only generate along Complete roads (not Planned or UnderConstruction roads)
- [ ] Show construction state in the info panel when a building or road is clicked
- [ ] Show estimated completion date in info panel for in-progress construction

---

## Acceptance Criteria

- When a road is placed, it appears translucent/ghosted and gradually transitions to solid over game-time
- When buildings spawn, they appear translucent first, then shift to under-construction appearance, then become solid
- Construction takes a noticeable amount of game-time — the player can see the town being built, not just popping into existence
- Clicking a building or road under construction shows its state and estimated completion
- Buildings only appear along roads that have finished construction
- Multiple buildings can be under construction simultaneously
- Pausing the game pauses construction; fast-forwarding speeds it up proportionally
