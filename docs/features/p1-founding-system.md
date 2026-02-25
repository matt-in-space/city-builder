# Founding System

**Priority:** P1
**Status:** Not Started
**Depends On:** Terrain (Milestone 2, complete), Map Resources (p0-map-resources)

---

## Overview

The founding system is how a new game begins. Instead of dropping into an empty map and painting zones, the player surveys the terrain, identifies a promising location, and establishes a founding point — a specific, named building that represents the economic reason this town exists.

The map has pre-existing connections to the outside world (initially a road leading off the map edge; rail and river connections come later). The player picks a spot near this connection and chooses a founding establishment based on what resources and geography are nearby. This choice shapes the early economy and the character of the town.

This replaces the traditional city builder start of "here's an empty map, start building." Instead, the player is making a strategic, contextual decision that has lasting consequences.

---

## Technical Details

**Pre-existing map features:**
- One or more roads leading off the map edge (representing regional connections)
- Resource deposits visible on terrain (timber, fertile land, coal, clay, stone)
- Geography: rivers, hills, flat areas, etc.

**Founding flow:**
1. Game starts in a "founding mode" — the player can pan around the map freely
2. Player clicks a location to evaluate it as a founding site
3. The game shows what's nearby: distance to nearest road connection, nearby resources (within a radius), terrain features
4. Based on what's available, the game presents 2-4 founding establishment options (contextual, not a fixed menu)
5. Player picks one; the founding building is placed, a name is generated, and the game clock starts

**Founding establishments (initial set):**

| Establishment | Requires Nearby | Workers | Economic Seed | Example Name |
|---|---|---|---|---|
| Trading Post / General Store | Road connection | 2-3 | Import/sell goods to area | "Henderson's Trading Post" |
| Lumber Camp Office | Timber + road | 8-12 | Harvest and export timber | "Pacific Lumber Company" |
| Quarry Office | Stone or clay + road | 8-12 | Harvest and export stone/brick materials | "Ridgemont Stone Works" |
| Mining Company Office | Coal + road | 15-25 | Extract and export coal | "Black Hill Coal Company" |
| Farmstead | Fertile land + road | 3-5 | Produce and export food/agriculture | "Whitfield Farm" |

**Name generation:** Procedurally generated from a pool of era-appropriate surnames, geographic references, and business suffixes. "Sullivan's Depot," "Creekside Trading Co.," "Appalachian Coal & Iron," etc.

**Post-founding immediate effects:**
- The founding building is placed and a few starter buildings spawn nearby within the first few game-months (a house or two for the workers, maybe a boarding house)
- The entry point for immigration is established at the map-edge road connection
- Initial residents arrive to staff the founding establishment
- The economic seed begins: the trading post starts importing goods, the lumber camp starts producing lumber, etc.

**Connection to the outside world:**
- The pre-existing road to the map edge is the immigration and trade artery
- Goods flow in and out along this road
- People arrive along this road
- The player can build additional roads from the founding point outward to open up more of the map

---

## Implementation Checklist

- [ ] Implement founding mode: game starts with free camera, no clock running, player exploring the map
- [ ] Define founding site evaluation: when player clicks, calculate nearby resources (within configurable radius), distance to road connection, terrain suitability
- [ ] Generate contextual founding options based on what's near the selected site
- [ ] Display founding options to the player with descriptions and expected economic effects
- [ ] Implement procedural name generation for founding establishments (surname + business type pools)
- [ ] Place founding building on confirmation, transition to normal gameplay, start game clock
- [ ] Spawn initial residents (workers for the founding establishment)
- [ ] Spawn 1-2 starter houses near the founding building for those initial residents
- [ ] Establish the map-edge road as the entry point for immigration and trade
- [ ] Ensure the founding building functions as an employer and economic entity from the start

---

## Acceptance Criteria

- Starting a new game presents the player with a map to explore, not an immediate build mode
- Clicking different locations shows different founding options based on nearby resources
- Clicking near timber and a road offers a lumber camp; clicking near coal offers a mining office; etc.
- The founding establishment has a generated name that feels era-appropriate
- After founding, a small cluster of buildings appears organically near the establishment
- The game clock starts and the town begins its life
- The player's first action after founding is building roads outward, and the town responds to those roads
