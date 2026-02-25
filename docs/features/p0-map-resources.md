# Map Resources

**Priority:** P0
**Status:** Not Started
**Depends On:** Terrain (Milestone 2, complete)

---

## Overview

The map needs a resource layer that defines what raw materials are available and where. Resources are the economic foundation — they determine what industries can exist, what gets exported, and what still needs to be imported. They're visible on the terrain so the player can make informed decisions about where to build roads and where to found their town.

The initial resource set is small and grounded in the 1920s setting: timber, fertile land, coal, clay, and stone. Each is tied to terrain features and visually distinct on the map.

---

## Technical Details

**Resource types (initial set):**

| Resource | Terrain Association | Visual Indicator | Harvested By |
|---|---|---|---|
| Timber | Forested areas (mid-elevation, moderate slope) | Dense dark green tree clusters | Sawmill |
| Fertile Land | Flat, low-elevation areas near water | Rich brown/dark green ground tint | Farm |
| Coal | Underground deposits, hilly terrain | Dark patches on hillsides, exposed seams | Coal mine |
| Clay | Riverside areas, low terrain near water | Reddish-brown ground patches | Brick works |
| Stone | Rocky terrain, hillsides, exposed rock | Light gray rock outcroppings | Quarry |

**Data representation:**
- Resource data is a spatial layer on top of the terrain, similar to the biome/material layer
- Each cell in the resource grid has a resource type (or none) and a quantity/richness value
- Resources are generated procedurally during map generation based on terrain features
- Resources are finite but large — a timber stand has thousands of board-feet, a coal seam has years of extraction. Depletion is a long-term concern, not an immediate one.

**Resource generation rules:**
- Timber spawns in areas with moderate elevation, not too steep, away from water
- Fertile land spawns in flat, low areas — river valleys, plains
- Coal spawns in hilly terrain, often clustered in specific deposits
- Clay spawns near water (riverbanks, lake edges)
- Stone spawns on steep terrain, hillsides, elevated areas

**Visibility:** Resources should be clearly visible on the map. This could be through terrain coloring, small 3D indicators (tree clusters for timber, rock outcroppings for stone), or an overlay mode. The player needs to be able to look at the map and say "there's timber to the north, coal in the eastern hills, fertile land along the river."

**Accessibility:** A resource only becomes economically useful when connected to the road network. A timber stand with no road access is just scenery. Build a road to it and suddenly a sawmill can operate there.

---

## Implementation Checklist

- [ ] Define `ResourceType` enum: Timber, FertileLand, Coal, Clay, Stone
- [ ] Define resource data layer: spatial grid with resource type and quantity per cell
- [ ] Implement procedural resource generation during map creation based on terrain features
- [ ] Timber: place in forested elevation bands with moderate slope
- [ ] Fertile land: place in flat, low areas, especially near water
- [ ] Coal: place in clustered deposits in hilly terrain
- [ ] Clay: place near water features (rivers, lakes)
- [ ] Stone: place on steep/rocky terrain and hillsides
- [ ] Visual representation: adjust terrain coloring or add small indicator meshes for each resource type
- [ ] Ensure resources are clearly distinguishable from each other and from bare terrain
- [ ] Implement resource accessibility check: is this resource connected to the road network?
- [ ] Add resource info to the hover/click info panel (what resource is here, quantity, accessibility)
- [ ] Optional: add a resource overlay toggle that highlights all resources with colored regions

---

## Acceptance Criteria

- Generated maps have visually distinct resource deposits in logical terrain locations
- Timber appears in forested areas, fertile land in river valleys, coal in hills, etc.
- Resources are visible enough that the player can plan road routes toward them
- Clicking on a resource deposit shows its type and richness
- Different maps have different resource distributions, making each playthrough's economic opportunities unique
- Resources near roads are flagged as accessible; isolated resources are flagged as inaccessible
