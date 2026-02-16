# City Builder — Game Design Overview

**Working Title:** TBD (Candidates: *Boomtown*, *Iron & Smoke*, *Iron & Grit*, *Iron & Gilt*)
**Engine:** Bevy (Rust)
**Setting:** 1920s America — The Roaring Twenties
**Genre:** City Builder / Economic Simulation

---

## Vision

A city builder where the player is not an omniscient hand placing buildings — the player is a city **governor**. You create the conditions under which a city grows. You set policies, lay infrastructure, and respond to crises. The city itself is the living thing; the player is its steward.

The core philosophy: **the economy determines how the city grows**. The player's inputs — infrastructure decisions, policy changes, event responses, zoning choices — feed into an economic simulation that drives what actually gets built, where, and when. The tension between the player's vision and the city's emergent reality is where the fun lives.

Every playthrough should be genuinely different. Map geography, resource availability, and player decisions interact with the economic simulation to produce unique cities — a steel town on one map, a trade hub on another — without the player ever selecting a "city type."

---

## Setting: The 1920s

The 1920s is an ideal setting for this game for several reasons:

- **Urban inflection point.** This is the decade where small towns became cities and cities became metropolises. Electrification is rolling out, automobiles are beginning to reshape cities, steel-frame construction is enabling skyscrapers for the first time, and massive population growth is driven by immigration and rural-to-urban migration.
- **Resource constraints create gameplay.** No nuclear power, no modern construction equipment. Coal-fired power plants serve limited areas. Steel is expensive and must be shipped by rail. Construction is labor-intensive and slow. These constraints make every decision meaningful.
- **Transportation is in transition.** Horse-drawn vehicles coexist with early automobiles. Streetcars are the backbone of urban transit. Rail is king for freight and intercity travel. This gives a natural tech progression within the game.
- **Building materials tell a story.** Wood and brick are cheap and local but limited in scale (5-6 stories max with load-bearing masonry). Steel enables skyscrapers but is expensive and imported. Reinforced concrete is becoming standard. Each material has real gameplay implications.
- **Labor is a strategic resource.** No hydraulic excavators or tower cranes. Steam shovels, manual labor, early electric hoists. Skilled tradesmen (ironworkers, electricians, plumbers) are scarce. Construction ties up significant workforce.
- **Rich social dynamics.** Immigration waves, Prohibition, labor unions, political machines, Art Deco culture. Endless material for events and narrative.
- **Built-in narrative arc.** Starting in 1920, the player experiences the boom of the Roaring Twenties, the crash of '29, and the Depression — a natural mid-game crisis that reshapes the entire economic landscape.
- **Distinctive aesthetic.** Art Deco architecture, brick factories with smokestacks, streetcars, the gritty charm of early industrial cities. Visually distinct from every other city builder on the market.

**Scope:** Initial release focuses on the 1920s era. Future expansions could add subsequent decades (1930s Depression, 1940s wartime industry, 1950s suburbanization), each bringing new technology, challenges, and events.

---

## Core Design Pillars

### 1. Organic Growth Progression

The game supports growth from a tiny rural township to a thriving metropolis. Progression is **emergent from the economy**, not gated by arbitrary population milestones. You *can* build a hospital at 200 people — you just can't afford it, can't staff it, and construction takes forever because everything is imported.

**Township Phase**
- Dirt roads, a handful of houses, maybe a single general store.
- One connection to the outside world (rail line or highway).
- Everything is imported. Budget is razor thin.
- Maybe one guy with a truck handling all construction.
- Intimate feel, almost like a tycoon game.
- The player is placing individual structures nearly by hand.

**Small Town Phase**
- A small commercial district forms organically.
- Some roads can be paved. A small factory or workshop opens.
- First municipal services appear.
- Construction is still slow but you might have a small crew.

**City Phase**
- Real infrastructure: water treatment, proper utilities, highway connections.
- Local economy begins producing goods, reducing import dependency.
- Construction firms exist in-city; building is faster.
- Zoning politics start mattering.

**Metro Phase**
- Highways, transit systems, skyscrapers become feasible.
- Economy is largely self-sustaining.
- Challenge shifts from growth to management: traffic, inequality, competing interests.

The player can also choose to *not* pursue growth. Building a small, thriving farm town is a valid and satisfying playstyle.

### 2. Living Economy

The economy is the beating heart of the game. It is a network of **needs, production, and exchange** — not top-level stats that get adjusted, but a simulation of actual economic activity.

See the [Economy](#economy) section for full details.

### 3. Construction as Process

Buildings and infrastructure are not instantly placed. Construction is a pipeline:

**Plan → Approve → Procure → Construct → Complete**

- **Plan:** The player lays out roads, zones areas, places building footprints. Shown as blueprints or translucent outlines. The game provides cost estimates, time estimates, and flags issues (route goes through existing homes, zone conflicts, terrain problems).
- **Approve:** Small projects may auto-approve. Large projects (highways, major buildings) go through city council, which has political factions. Public hearings surface arguments for and against. The player navigates competing interests.
- **Procure:** The project needs materials — concrete, steel, lumber, etc. These must be sourced locally or imported. If supply is insufficient, the project stalls or costs increase. Multiple simultaneous projects compete for materials.
- **Construct:** Vehicles arrive, the site is active, progress is visible over time. Early game: painfully slow (one crew, imported materials). Late game: multiple construction firms, local supply chains, much faster. Construction affects traffic and nearby residents (noise, disruption).
- **Complete:** The building or road is finished and enters service.

**Visual States (Initial Implementation):**
- Planned: Translucent outline / blueprint overlay
- Under Construction: Translucent with scaffolding representation
- Complete: Solid geometry

**Future Polish:** Animated construction sequences — scaffolding going up, workers on site, material deliveries, buildings rising floor by floor. The goal is to make the player want to zoom in and watch.

---

## Economy

### Fundamental Model

The economy is a closed loop of money circulation. Residents earn wages, spend at businesses, pay taxes. Businesses pay employees, buy supplies (locally or imported), pay taxes, keep profit. The city budget comes from taxes and fees. Money enters the system through exports and external trade; money leaves through imports.

### Individual Simulation (Small Scale)

At small population (roughly 20-200 residents), each person is a discrete entity:
- Has a job (or doesn't), income, household membership
- Has tiered needs (simplified Maslow hierarchy):
  - **Basic:** Food, shelter
  - **Comfort:** Goods, entertainment
  - **Aspirational:** Culture, community
- Each need has a fulfillment level that decays over time and replenishes when the person accesses the relevant good/service
- Earns wages, spends to fulfill needs — each transaction is real money moving between entities

This produces rich emergent behavior at low computational cost. A general store that can't restock means people can't buy food, fulfillment drops, unhappiness rises, someone sees the business opportunity, a second store opens.

### Transition to Statistical Modeling (Large Scale)

At roughly 500-1,000+ population, the simulation transitions gradually and invisibly:

**Two-tier system: Notable Households + Statistical Population**

- **Notable Households (~100-200 at all times):** Discrete entities with names, jobs, spending habits, stories. Some are persistent (founding families, prominent business owners, recurring characters). Others rotate as new neighborhoods develop. Clicking a notable household's home shows real family details.
- **Statistical Population:** Modeled as demographic cohorts — groups of similar people (e.g., "working-class families in manufacturing," "single professionals in commercial jobs"). Each cohort has aggregate behavior derived from the same rules that govern individuals: average income, spending patterns, need fulfillment, happiness.
- **Invisible boundary:** When the player clicks a house belonging to the statistical population, the game generates plausible details on the fly from cohort data. The player never knows which households are notable vs. statistical.

New arrivals increasingly join the statistical population as the city grows. The cohort math is identical to individual math, just multiplied — if food prices rise 20%, individuals and cohorts both adjust spending and fulfillment identically.

### Goods and Supply Chains

Every physical thing in the game is a good in the economy: lumber, brick, steel, concrete, coal, food, textiles, consumer goods.

Each good has:
- **Source:** Imported or locally produced
- **Transport method:** Rail, truck, horse cart
- **Base cost**
- **Local price:** Fluctuates based on supply and demand

**Supply chain flow:**
1. City connects to the outside world (rail line, highway).
2. Goods flow in at base cost + transport markup. Distance from source increases cost.
3. Early game: everything is imported, everything is expensive.
4. As city grows: local production comes online (sawmill, brick factory, steel mill).
5. Local goods are cheaper (less/no transport cost) — a major economic milestone.
6. Local production has its own supply chain: workers, raw materials, equipment. Disruptions cascade.

**Import capacity matters.** A single-track rail line has limited freight throughput. If demand exceeds capacity, bottlenecks occur. This pressures the player to develop local production or invest in better trade infrastructure.

**Exports** work in reverse. Surplus local production can be exported for revenue. This enables an economic strategy — intentionally building up industries to become an export hub. A steel town plays differently than a textile center or agricultural hub.

### Market Pricing

Simple supply-and-demand model for each good:
- Each good has a local supply rate (production + imports) and demand rate (consumption + construction needs)
- Price = base cost × (demand / supply), with smoothing to prevent wild swings
- Price changes feed back into behavior: high lumber prices cause substitution to brick, attract new suppliers, slow construction
- Cascading effects: steel shortage → expensive construction → slower growth → fewer jobs → less spending → hurting local businesses

### City Budget

The player's primary economic lever:
- **Revenue:** Property tax, income tax (residents), commercial tax (businesses). Tax rates are player-controlled.
- **Expenditures:** Municipal services, infrastructure construction, maintenance.
- **Trade-off:** High taxes fund services but discourage growth. Low taxes attract people/businesses but limit infrastructure investment.
- **Dynamic costs:** Construction costs reflect the *actual economy*. If steel prices are high, bridges cost more. If there's a local asphalt plant, paving is cheaper. The budget is a real-time window into economic health.

### Economic Failure and Recovery

The economy can contract. Failure should feel dramatic but be recoverable:
- Factory closure → visible unemployment, business revenue drops, tax revenue falls
- Supply disruption → construction sites sit idle, no materials
- Recession → empty storefronts, people leaving town, city shrinking

Recovery paths: lower taxes, invest in job-creating infrastructure, diversify economy, attract new industry. The player steers a ship through storms.

---

## Terrain

Heightmap-based terrain system. The heightmap is a grid of elevation values rendered as a mesh, but this grid is invisible to the player — it's purely underlying data.

**Properties:**
- Resolution dense enough for smooth road contours without jaggedness
- Material/biome layer painted as a separate texture map: grass, dirt, rock, water
- Generated at game start, modified by player terraforming

**Water:**
- Rivers and bodies of water where heightmap dips below water table
- Initial implementation: flat planes at set elevation
- Affects gameplay: can't build on water without bridges, flooding risk in low areas, water access is economically valuable

**Terrain affects everything:**
- Building costs (hillside construction is expensive)
- Road grades (can't put a highway up a steep slope)
- Water flow (rivers follow valleys, flooding in lowlands)
- Resource availability (forests for lumber, mineral deposits for industry)
- Lot viability (steep lots don't develop or cost more to grade)

---

## Roads

Roads are the spine of the city. Freeform, not grid-locked.

**Technical Approach:**
- Modeled as splines (cubic Bezier or Catmull-Rom curves)
- Player places control points; game interpolates smooth curves
- Road mesh generated procedurally from spline: sample points along curve, generate cross-section, stitch into geometry
- Road conforms to terrain (initially projected onto heightmap; later, proper grading where road modifies surrounding terrain)

**Road Properties:**
- Width, material type (dirt, gravel, paved), speed limit
- Number of lanes (future)
- Construction state (planned, under construction, complete)

**Intersections:**
- Generated where splines meet
- Initial implementation: flat polygon at junction
- Data model tracks connectivity (which roads connect, at what angles) for future traffic simulation

**Bridges:**
- Road segments flagged as "not touching terrain"
- Require anchor points on solid ground at each end
- Span limited by material and era (1920s: steel truss, concrete arch, timber for short spans)
- Height must clear what's underneath
- Initial visual: flat road hovering over gap with box pillars
- Economically expensive and material-intensive

---

## Zones and Lot System

### Zone Painting

Zones are painted as freeform polygons using a brush tool — like painting in a graphics program. No grid snapping, no alignment to roads.

**Zone Types:** Residential, Commercial, Industrial, Mixed-Use

**Zone Parameters:** Density cap, height limit (future)

**Key principle:** The player sets rules; the economy decides what gets built. Zone a mixed-use area near downtown and you might get ground-floor retail with apartments above. Zone residential near a park and you get single-family homes. The simulation decides based on demand, accessibility, and attractiveness.

### Lot Subdivision

When zones are painted or modified, a subdivision algorithm generates potential building lots — irregular polygons defined by road network and zone boundaries, not grid cells.

**How it works:**
- Road frontage is the primary organizing principle
- Lots project inward from frontage edges
- Each lot gets properties: area, road frontage length, slope, distance to services
- Interior lots (no frontage) develop later via alleys or new side streets

**Context-sensitive sizing:**
- Dense urban core: narrow, deep lots (e.g., 25×100 ft, classic NYC tenement dimensions)
- Suburban/small town: wider lots (50×120, 80×130)
- Industrial zones: large irregular lots (factories need floor space, not frontage)
- Commercial on main roads: wider lots with more frontage (storefronts want visibility)

**Corner lots** are premium — two-street frontage, weighted toward commercial use.

**Lot dynamics:**
- Lots can merge (wealthy resident claims multiple lots)
- Lots can split
- Lots get redrawn when roads change
- Empty lots fill based on demand — frontage lots first, back lots later
- Infill development: empty lots develop later at higher density as area matures

### Building Spawning

Buildings spawn when the economy determines demand:
- Simulation evaluates lots: zone type, demand, accessibility (roads, transit, utilities), attractiveness (amenities, low crime, terrain)
- Appropriate building type selected probabilistically
- Early game: small houses, small shops
- As area develops: buildings replaced with larger ones (house → duplex → apartment)
- Organic, uneven growth pattern — not uniform grid fill

### Manually Placed Buildings

Player-placed structures (hospitals, police stations, etc.):
- Player picks a spot; game validates against constraints (size, road access, terrain)
- If placed away from a road, construction plan automatically includes a connector road to nearest existing road, factored into cost and time
- Enters the standard construction pipeline (plan → approve → procure → construct → complete)

---

## Event System

Inspired by Paradox Interactive games (Crusader Kings, Stellaris). Narrative events with player choices that have real consequences in the simulation.

### Architecture: Three Layers

**Engine Layer (Build Once)**
- Evaluates trigger conditions against world state each game tick
- Presents events to the player
- Executes consequence functions
- Schedules follow-up events
- Manages cooldowns and pacing (event budget: ~1-2 events per game-month, crisis events override)
- Prioritizes events contextually relevant to player's current activity

**Schema Layer (Expand Over Time)**
- Defines what conditions and effects are available
- Condition types: population thresholds, resource levels, faction approval, time ranges, building existence, previous event choices, etc.
- Effect types: modify resource, change faction relation, spawn/despawn entities, trigger follow-up event, unlock features, modify stats
- New game systems add new condition/effect types

**Content Layer (Ongoing / Expansions)**
- Actual event definitions in data files (RON, TOML, or custom format)
- Each event: ID, trigger conditions, narrative text, options with effects, chain references
- Moddable — community can author events
- Initial release: ~50-100 events covering core gameplay
- Expansions add hundreds more (Prohibition depth, labor politics, Crash of '29 chain, etc.)

### Event Types

**Scripted Events:** Hand-authored, tied to specific conditions. Major story beats — the stock market crash, Lindbergh's flight, radio going mainstream. Foreshadowed by earlier events in chains.

**Emergent Events:** Generated by simulation state. High crime triggers law enforcement events. Rapid growth triggers infrastructure strain. Neighborhood demographics trigger cultural events. Templated but feel organic.

### Event Chains

Choices cascade into future events. The speakeasy you tolerated in 1923 leads to the mob showing up in 1925. The banker who warned about speculation offers a lifeline in 1930. Creates narrative continuity and makes the city feel like it has a *story*.

Events can be tied to specific buildings or districts — a fire at the textile mill is a fire at *that* textile mill, on *that* block, and the player watches it unfold in the physical space they built.

### Factions (Future Expansion)

Initial implementation: abstract factions with approval ratings.
- **Potential factions:** Business interests, Labor, Residents, Reformers, etc.
- Events reference factions in conditions and effects
- Future depth: individual council members with personalities, elections influenced by demographics, corruption mechanics

---

## Rendering Approach

### Prototype Phase (Boxes and Planes)

Start with simple placeholder geometry to validate systems:

- **Residential:** Box with triangular prism roof (pitched roof). Warm color tones.
- **Commercial:** Wider, flatter box, slight front setback suggesting storefront. Cool color tones.
- **Industrial:** Large, flat box with sawtooth roof profile (factory skylights). Gray tones.
- **Roads:** Flat strips on terrain, color-coded by material (brown = dirt, gray = gravel, dark gray = paved).
- **Bridges:** Flat road segments hovering over gaps with box pillars.
- **Water:** Flat blue planes.
- **Construction states:** Translucent for planned, translucent with wireframe scaffolding for under construction, solid for complete.

Color-coding by building type ensures the city reads clearly even with primitive geometry.

### Future Art Direction

Art Deco aesthetic. Brick factories with smokestacks, ornate commercial facades, streetcars, period-appropriate vehicles. Visually distinct from all competitors.

---

## Build Order

### Wave 1: Core City Building
1. **Terrain** — Heightmap loading, mesh generation, material painting, camera controls
2. **Roads** — Spline placement, mesh generation, terrain conforming, basic intersection connectivity
3. **Zone Painting** — Brush-based zone painting, lot subdivision from zones + road network, visual feedback showing lots
4. **Building Spawning** — Placeholder boxes on lots driven by simple economic check (can be timer-based initially)
5. **Construction Pipeline** — Plan/under-construction/complete states with visual feedback
6. **Economy** — The real simulation: goods, supply chains, pricing, individual agents, city budget

### Wave 2: Event System
- Build the engine and schema layers
- Author starter set of events (~50-100) that make construction and economy feel alive
- Basic faction system (abstract approval ratings)

### Wave 3: Depth and Polish
- Deepen factions and politics (council members, elections, corruption)
- Rich event chains, historical anchors (Crash of '29)
- Construction animation and visual polish
- Traffic simulation
- Real art assets replacing placeholder geometry

### Future Expansions
- Additional decades (1930s, 1940s, 1950s)
- Expanded event content (Prohibition, labor movement, wartime industry)
- Deeper systems (transit networks, suburbs, advanced politics)
- Modding support for community-authored events and content

---

## Name Candidates

| Name | Notes |
|---|---|
| **Boomtown** | Punchy, immediately evocative of the era, describes the gameplay. Highly marketable. |
| **Iron & Smoke** | Atmospheric, paints a picture of industrial city growth. Factory smokestacks, coal plants, steam engines. |
| **Iron & Grit** | Short, punchy, captures scrappy determination. Working-class 1920s energy. |
| **Iron & Gilt** | Pairs raw material with Art Deco gold aspiration. Iron to start, gilt to finish. Subtle wordplay with "guilt." |
| **Iron & Asphalt** | Literal and tactile. Steel for buildings, asphalt for roads. Gritty, hands-dirty feel. |
| **Iron & Prospect** | "Prospect" as both future vision and prospecting for opportunity. Sounds like a street intersection. |
| **Iron & Brick** | Most grounded and literal. Primary building materials of the era. |
| **Iron & Acre** | Land-focused, earthy. Expanding acre by acre. |
