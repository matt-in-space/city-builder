# Economic Simulation Design

## Overview

The economic simulation is the foundational system of the game. It replaces the classic city builder approach of abstract RCI demand meters with a causally grounded, supply-chain-driven model of urban growth. Cities do not grow because a pressure valve releases — they grow because rational economic actors respond to real local opportunities created by infrastructure, resources, and existing demand.

This document defines the core tenets of the economic model, how goods flow through the city, how buildings are spawned and upgraded, and how the system scales from individual buildings to district-level abstraction.

---

## Core Tenet: Economic Causality Over Abstract Demand

Traditional city builders (SimCity, Cities: Skylines) simulate city growth by maintaining global demand variables for residential, commercial, and industrial zones. When demand is high enough, buildings spawn. This is a compression of economic reality that feels arbitrary because it is arbitrary — there is no underlying reason why a factory appears, only that a meter reached a threshold.

This game replaces that model entirely. Every building that appears in the city does so because a specific economic case for it exists at that location. The questions the simulation asks are:

- Is there a resource here that can be profitably extracted given current infrastructure?
- Is there a workforce nearby to staff this operation?
- Are input goods available in sufficient quantity to support this process?
- Is there downstream demand for what this building produces?
- Can the goods physically travel the road network to reach buyers?

If the answer to the relevant questions is yes, a building becomes a viable investment and will eventually be constructed. The player's role is to create the conditions that make these answers yes — not to place the buildings themselves.

---

## Resource Viability: Where Growth Begins

City growth always originates from a resource opportunity made accessible by infrastructure. The sequence is:

1. A resource deposit exists on the map (timber stand, coal seam, fertile agricultural land, stone outcrop, etc.)
2. The player builds a road connecting that resource to the existing road network
3. If that road network connects to a market access point (the edge of the map, a rail depot, a trading post), the resource becomes viable for extraction
4. An extraction industry spawns at or near the resource — not because industrial demand went up, but because that specific resource is now reachable and sellable

This means the player is always making a meaningful geographic decision when building roads. Every road placement is a potential economic catalyst. A road to nowhere does nothing. A road to a timber stand that connects to a rail line creates a logging economy.

---

## The Goods System

### Goods Are Concrete and Countable

All economic exchange in the city is mediated by goods. Goods are discrete units produced and consumed per month, tracked to one decimal place. This allows small-scale production (a backyard garden producing 0.1 food per month) to exist alongside industrial-scale output without requiring a different system.

There is no abstraction layer above goods. If a building needs timber and no timber is available within its service radius, it is simply unsupported. Scarcity is emergent and natural.

### The Starting Goods List

The initial goods set covers one full industrial vertical (timber to furniture) and one agricultural vertical (farming to food), plus the secondary goods needed to support a functioning residential population:

| Good | Tier | Description |
|---|---|---|
| Timber | Raw Material | Harvested from forested resource tiles |
| Coal | Raw Material | Extracted from coal deposit tiles |
| Grain | Raw Material | Grown on fertile agricultural land |
| Lumber | Processed Good | Produced by sawmill from Timber |
| Flour | Processed Good | Produced by grist mill from Grain |
| Food | Consumer Good | Produced by bakery/general store from Flour |
| Furniture | Luxury Good | Produced by furniture workshop from Lumber |
| Dry Goods | Consumer Good | General merchandise; imported or produced locally |
| Fuel | Processed Good | Produced from Coal; used by industry and heating |
| Labor | Special | Not a physical good; represents available workforce from nearby housing |

This 10-good list is enough to simulate: a logging economy → sawmill → furniture production → luxury retail; a farming economy → milling → food retail; and a coal economy feeding industrial fuel needs. It provides a complete loop from resource extraction through consumer goods to residential support.

### Three-Tier Supply Chain Structure

**Tier 1 — Raw Materials**: Timber, Coal, Grain. Extracted directly from resource tiles. Cannot be produced; only harvested. Require extraction buildings (logging camp, coal mine, farm).

**Tier 2 — Processed Goods**: Lumber, Flour, Fuel. Produced by processing industries from raw materials. Require both a building and a supply of the relevant raw material within service radius.

**Tier 3 — Consumer and Luxury Goods**: Food, Furniture, Dry Goods. Required by residential buildings to sustain and grow population. Consumer goods are basic needs; luxury goods raise quality of life and affluence levels.

Labor is a special cross-cutting resource. It is not shipped like a physical good but is instead a measure of available workforce. Residential buildings produce labor; industrial and commercial buildings consume it. If labor is unavailable within travel distance, buildings cannot operate at full capacity.

### Building Production Profiles

Every building has a production profile: a list of goods it consumes and produces per month, and the number of labor slots it requires. Example profiles:

**Logging Camp**
- Requires: Road access to timber resource tile, 4 labor
- Produces: 3.0 Timber/month
- Service radius: 8 road-minutes

**Sawmill**
- Requires: 2.0 Timber/month input, 6 labor
- Produces: 2.0 Lumber/month
- Service radius: 10 road-minutes

**Furniture Workshop**
- Requires: 1.0 Lumber/month input, 4 labor
- Produces: 1.5 Furniture/month
- Service radius: 8 road-minutes

**Worker Housing (basic)**
- Requires: 1.0 Food/month
- Produces: 4.0 Labor
- Service radius: N/A (labor radiates outward from building location)

**General Store**
- Requires: 2.0 Dry Goods/month, 2 labor
- Produces: Retail access (satisfies Dry Goods need for nearby residential)
- Service radius: 6 road-minutes

These profiles are data-driven and defined externally, not hardcoded. Adding a new building type requires only a new data entry, not changes to core simulation logic.

---

## Service Radius: Road Network Distance, Not Flat Circles

A building does not serve everything within a fixed pixel radius. It serves everything reachable within a maximum travel cost along the road network. This is a critical distinction that ties infrastructure investment directly to economic outcomes.

### Travel Cost

Travel cost is calculated as the time to traverse each road segment, which depends on road type:

| Road Type | Travel Cost Modifier |
|---|---|
| Dirt track | 1.0 (baseline) |
| Gravel road | 0.7 |
| Paved road | 0.4 |
| (Future) Rail spur | 0.1 for bulk goods |

A logging camp with a service radius of 8 road-minutes can supply a sawmill 12 tiles away if connected by paved road (cost ~4.8), but not one 12 tiles away on dirt track (cost 12.0). The player extending and upgrading the road network is directly expanding the economic reach of every building on it.

### Implications for Gameplay

- Building a paved road through a district is not just cosmetic — it genuinely integrates more buildings into each other's service areas
- A warehouse or depot building can act as a relay point, extending effective supply chains beyond what direct road connections would allow
- Future delivery services (a special building or upgrade) can explicitly increase service radius for buildings it serves, representing the shift from workers walking to work to goods being actively distributed

---

## Demand Signals: Local and Additive

Each building emits signals representing its unmet needs. These signals are not global counters — they are spatial and local.

- One building needing food creates a food demand signal of 1.0 at its location
- Ten buildings each needing food creates demand signals totaling 10.0 across their locations
- A general store evaluating whether to open looks at total food/dry goods demand within its service radius and weighs it against the supply already available

This means commercial buildings naturally gravitate toward population centers. A general store will open where many homes are clustered, not where a global commercial meter happened to spike.

The simulation tick evaluates each unoccupied viable tile periodically and asks: given the overlapping demand signals here, and the supply signals already present, is there a building type whose economic case is now positive? If yes, that building type enters a queue to be constructed. Construction takes in-game time.

---

## Building Growth and Upgrades

Buildings do not upgrade on arbitrary timers or happiness scores. They upgrade when economic conditions organically support expansion.

### Upgrade Conditions

A logging camp can expand from 4 workers / 3.0 timber to 8 workers / 6.0 timber when:
- Sufficient additional housing exists within labor travel distance to supply 4 more workers
- Sufficient food supply exists to support those additional workers
- Downstream demand for timber exceeds current production (i.e., a sawmill is supply-constrained)

If the player has built housing near the camp, connected a food supply, and the sawmill is running at capacity, the camp will eventually upgrade without the player pressing a button. The player created the conditions; the economy responded.

This applies at every tier. A sawmill expands when timber input is sufficient and lumber demand from downstream workshops exceeds output. A general store upgrades to a larger storefront when residential density around it grows. Building upgrades are a natural consequence of a healthy supply chain, not a reward for player score.

---

## Residential Growth and Labor Supply

Residential buildings are not placed by the player. They are the organic response to labor demand from industry.

When a logging camp and sawmill both have unfilled labor slots, a labor demand signal radiates outward from those buildings along the road network. When that signal reaches a tile with good connectivity and no current use, a small worker housing unit becomes a viable investment and will spawn.

As that housing provides workers, it also generates its own consumption needs — food, dry goods, eventually luxury goods as affluence grows. This consumption demand is what drives commercial development near residential clusters.

### Affluence

Residential buildings have an affluence level that determines what goods they demand and what quality of commercial services they attract. A basic worker cottage demands food and dry goods. A higher-affluence household demands furniture, finer goods, and may generate pressure for different neighborhood character. Affluence rises when residents have consistent access to consumer and luxury goods; it falls when basic needs go unmet.

Affluence is not a global variable. It is tracked per residential building and creates natural neighborhood differentiation — a workers' district near the mill will look and function differently from a middle-class neighborhood near the commercial district, because the economic conditions there are genuinely different.

---

## Scaling: From Buildings to Districts

The building-level simulation is the ground truth of the economy. However, as a city grows to hundreds of buildings, recalculating every supply chain every tick becomes expensive.

### Near-Term: Building-Level

In early development, every building recalculates its supply situation on a staggered update schedule. Not every building updates every tick — they are offset to distribute the computational load. A building that has been stable for many months updates less frequently than one that recently changed neighbors.

### Medium-Term: Neighborhood Aggregation

As the city grows, buildings can be grouped into economic neighborhoods — clusters of buildings that share a contiguous road network segment. The neighborhood is treated as a single economic unit that:
- Sums its total production of each good type
- Sums its total consumption of each good type
- Exports surplus to adjacent neighborhoods via road connections
- Imports deficit from adjacent neighborhoods if supply is available

Individual building-level simulation continues for buildings near change boundaries (new construction, destroyed buildings, new roads). Interior stable buildings operate at the neighborhood level until something changes.

This mirrors how real urban economics actually works: a neighborhood produces and consumes collectively, trading with other neighborhoods through transport corridors.

---

## Integration with Gameplay Systems

### Infrastructure as Economic Policy

The player's primary tool is infrastructure. Every road built is a decision about which economic opportunities to enable and which areas to connect. Road quality determines the effective range of every supply chain passing through it. The player does not decide what gets built — they decide what becomes possible.

### The Governor Role

The player acts as a governor, not a micromanager. Large infrastructure decisions (road corridors, rail lines, bridge placements) shape the city's growth patterns. Smaller decisions (zoning restrictions, business licensing, tax policy) become available as the city matures and governance institutions develop. The city grows without constant player intervention; the player's job is to guide its shape and character.

### Organic vs. Guided Growth

The simulation will produce cities that look like 1920s American towns if left to run — industry near resources, housing clustered around employment, commercial development along main thoroughfares, with character shaped by what industries happened to establish first. The player's interventions shift which opportunities get activated and how the road network channels growth, without overriding the underlying economic logic.

---

## Design Constraints and Non-Goals

- **No teleportation of goods**: Goods only move along the road network. A building not connected by road to its supplier is genuinely unsupplied.
- **No global demand meters**: There is no global residential, commercial, or industrial demand variable. All demand is local and causal.
- **No arbitrary spawn triggers**: Buildings do not appear because a timer expired or a score threshold was crossed. They appear because the economic case for them is positive.
- **People are not simulated individually**: We do not track individual citizens moving between home and work. Labor is a building-level output, consumed by other buildings within road-network distance. This keeps the simulation tractable while preserving economic realism.
- **Goods do not expire (initially)**: For simplicity in early development, produced goods that are not consumed within a tick roll over. Spoilage mechanics (relevant for food) can be added later as a complexity layer.
