# Save / Load

**Priority:** P2
**Status:** Not Started
**Depends On:** Core gameplay loop (building spawning, construction pipeline, basic economy)

---

## Overview

Persist and restore the full game state so the player can save their city and return to it later. This is essential for any city builder — sessions can be long and progress must be preserved.

Save/load needs to capture everything that defines the city: terrain (seed + config, not raw data), road network, buildings, construction state, economy, population, game time, and any event/narrative state. The format should be compact and forward-compatible as new systems are added.

---

## Technical Details

**What to save:**
- Terrain config + noise seed (regenerate heightmap deterministically rather than saving the full grid)
- Road network: all nodes, segments, spline data, road types, construction state
- Buildings: type, position, orientation, lot boundary, construction state, occupants/tenants
- Population: all resident data (or cohort data at large scale), employment, housing assignments, need states
- Economy: city budget, business data, wages, prices, supply/demand state
- Game time: current date, speed setting
- Event state: completed events, active event chains, cooldowns
- Camera position (nice-to-have for seamless resume)

**Save format:**
- Serialize with `serde` — Bevy's `Reflect` + scene serialization may work, or a custom struct
- RON or bincode format (RON for human-readable debugging, bincode for production)
- Versioned format with migration path for save compatibility across game updates

**Autosave:**
- Periodic autosave (configurable interval, e.g., every 5 game-months)
- Autosave on exit
- Keep last N autosaves as rolling backups

**Performance:**
- Save should not cause a noticeable frame hitch — consider async serialization or spreading work across frames
- Load can take a loading screen since it's a full state reconstruction

---

## Implementation Checklist

- [ ] Define a `SaveData` struct capturing all persistent game state
- [ ] Implement serialization of terrain config (seed-based, not raw heightmap)
- [ ] Implement serialization of road network
- [ ] Implement serialization of buildings and lots
- [ ] Implement serialization of population and economy state
- [ ] Implement serialization of game time and event state
- [ ] Add save-to-file system (manual save via UI or hotkey)
- [ ] Add load-from-file system (select save file, reconstruct game state)
- [ ] Add autosave system with configurable interval
- [ ] Add save file versioning and basic migration support
- [ ] Add save/load UI (save slot list, save name, timestamps)

---

## Acceptance Criteria

- Player can save the game and load it back to the exact same state
- All buildings, roads, population, and economic state are preserved
- Terrain regenerates identically from saved seed
- Autosave runs periodically without noticeable frame drops
- Loading a save reconstructs the city visually and functionally
- Save files are versioned so future game updates don't break old saves
