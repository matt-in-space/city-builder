# City Builder — Implementation Roadmap

**Goal:** Reach the core gameplay loop — display a terrain map, plan and construct roads and buildings, run a basic economy, and watch the city grow organically over time.

Each milestone is scoped to be completable in a focused session or two. Checkboxes represent concrete, individually completable units of work.

---

## Milestone 1: Project Scaffolding & Window

Get a Bevy app running with a camera looking at nothing. The foundation everything else builds on.

- [x] Initialize Bevy project with Cargo, basic dependencies (bevy, bevy_egui or equivalent for future UI)
- [x] Create app with default plugins, set window title and size
- [x] Add a 3D perspective camera with basic orbit controls (pan, zoom, rotate)
- [x] Add a ground-plane placeholder (flat colored plane) so the camera has something to look at
- [x] Add basic lighting (directional light for sun, ambient light)
- [x] Verify builds and runs cleanly on target platform

---

## Milestone 2: Terrain Generation & Rendering

Replace the flat plane with a real heightmap terrain the player can look around.

- [x] Define a `TerrainConfig` resource (map size, resolution, height scale)
- [x] Implement heightmap generation (Perlin/simplex noise with octaves for natural-looking hills and valleys)
- [x] Generate terrain mesh from heightmap (vertex grid, normals, UVs)
- [ ] Render terrain with a basic material (solid green or simple gradient by elevation)
- [ ] Add water plane at a fixed elevation (flat blue plane that fills low areas)
- [ ] Add terrain material/biome layer — paint grass, dirt, rock based on elevation and slope
- [ ] Ensure camera controls feel good when navigating the terrain (zoom limits, pan boundaries)

---

## Milestone 3: Road Placement (Data & Input)

Let the player draw freeform roads on the terrain. No mesh generation yet — just the data and debug visualization.

- [ ] Define road data structures: `RoadSegment` (control points, material type, width), `RoadNetwork` (graph of connected segments)
- [ ] Implement road placement input: click to place control points on terrain via raycasting
- [ ] Interpolate smooth curves between control points (Catmull-Rom or cubic Bezier spline)
- [ ] Debug visualization: draw the spline as a line/gizmo on the terrain so you can see the road path
- [ ] Handle road completion (confirm/cancel placement)
- [ ] Store roads in the `RoadNetwork` resource and track connectivity (which roads connect at which points)

---

## Milestone 4: Road Mesh Generation

Turn the road spline data into visible geometry on the terrain.

- [ ] Sample points along the road spline at regular intervals
- [ ] Generate cross-section vertices at each sample point (flat strip for now)
- [ ] Project road vertices down onto the terrain heightmap so roads follow the ground
- [ ] Stitch sample points into a triangle-strip mesh
- [ ] Apply a road material (brown for dirt, gray for paved — based on road type)
- [ ] Handle basic intersection rendering where roads meet (flat polygon fill at junction)
- [ ] Roads visually layer on top of terrain (slight Y offset to prevent z-fighting)

---

## Milestone 5: Basic UI & Game State

Add the minimal UI needed to interact with the game and control game flow.

- [ ] Implement game speed controls (pause, normal, fast, very fast)
- [ ] Add a basic HUD showing current date (month/year), city funds, population count
- [ ] Implement a toolbar for selecting tools (road tool, zone tool, building placement tool — stubs for now)
- [ ] Add a simple info panel that shows details when hovering/clicking objects
- [ ] Implement a basic notification/message system for surfacing game events to the player

---

## Milestone 6: Zone Painting

Let the player paint zones onto the terrain that define where buildings can appear.

- [ ] Define zone types as an enum (Residential, Commercial, Industrial)
- [ ] Implement brush-based zone painting: click and drag to paint zones onto terrain
- [ ] Store zone data as a spatial layer (grid overlay at sufficient resolution, or polygon regions)
- [ ] Render zones as colored overlays on the terrain (warm = residential, cool = commercial, gray = industrial)
- [ ] Allow zone erasing/overwriting
- [ ] Add toggle to show/hide zone overlay

---

## Milestone 7: Lot Subdivision

Turn painted zones into buildable lots based on the road network.

- [ ] When zones are painted or roads change, trigger lot subdivision for affected areas
- [ ] Identify zone areas that have road frontage (within a threshold distance of a road)
- [ ] Subdivide frontage into lots: project inward from road edge, create rectangular-ish lot polygons
- [ ] Assign lot properties: area, frontage length, slope (sampled from terrain), distance to road
- [ ] Filter out unbuildable lots (too steep, too small, on water)
- [ ] Debug visualization: draw lot boundaries as wireframe outlines on terrain
- [ ] Handle lot regeneration when roads or zones change

---

## Milestone 8: Building Spawning (Placeholder Geometry)

Make buildings appear on lots. Just colored boxes for now, driven by a simple timer.

- [ ] Define building types with basic properties: footprint size, height, color, zone type requirement
- [ ] Implement a simple spawning system: periodically check for empty lots that meet minimum requirements
- [ ] Place a building (box mesh) on a valid lot, oriented to face the nearest road
- [ ] Vary building size slightly by lot size (small lot = small house, bigger lot = bigger building)
- [ ] Residential: warm-toned box with pitched roof (triangular prism on top)
- [ ] Commercial: wider, flatter box in cool tones
- [ ] Industrial: large flat box with sawtooth roof in gray
- [ ] Prevent overlapping buildings; mark lots as occupied

---

## Milestone 9: Construction Pipeline (Visual States)

Buildings and roads shouldn't appear instantly. Add the plan → construct → complete flow.

- [ ] Add a `ConstructionState` component: Planned, UnderConstruction, Complete
- [ ] Planned buildings render as translucent outlines
- [ ] Under-construction buildings render as translucent with a simple scaffolding indicator (wireframe box or color change)
- [ ] Complete buildings render as solid geometry
- [ ] Construction progresses over game-time (configurable duration per building type)
- [ ] Apply the same states to roads: planned roads show as dashed/translucent, under construction with a visual indicator, complete as solid
- [ ] Surface construction progress in the info panel when clicking a building/road

---

## Milestone 10: Basic Population System

Introduce people into the simulation. Start with individual agents at small scale.

- [ ] Define a `Resident` entity with components: name (generated), household, employment status, income, needs (food, shelter, goods)
- [ ] Spawn initial residents when the game starts (small number, e.g. 10-20)
- [ ] Residents require housing — they claim a residential building as home
- [ ] Residents look for jobs at commercial/industrial buildings
- [ ] Need fulfillment ticks down over time; replenished when the resident can access goods/services
- [ ] New residents arrive over time if housing is available and conditions are acceptable (basic attractiveness check)
- [ ] Residents leave if needs go unfulfilled for too long
- [ ] HUD population counter reflects actual resident count

---

## Milestone 11: Basic Economy — Money Flow

Add money circulation so the city has a functioning economic loop.

- [ ] Define a `CityBudget` resource: funds, tax rate, income, expenses
- [ ] Residents earn wages from their employer (commercial/industrial building)
- [ ] Residents pay taxes (income tax as a percentage of wages)
- [ ] Businesses earn revenue from residents spending to fulfill needs
- [ ] Businesses pay wages to employees
- [ ] City collects taxes each game-month; budget updates
- [ ] Road and building construction costs deducted from city budget
- [ ] Display budget summary in HUD (funds, monthly income, monthly expenses)
- [ ] If city runs out of money, construction halts (can't start new projects)

---

## Milestone 12: Goods & Import System

Give the economy physical goods that flow through supply chains.

- [ ] Define a set of basic goods: lumber, brick, food, consumer goods
- [ ] Define an external trade connection (the city's lifeline to the outside world)
- [ ] Goods are available for import at a base cost + transport markup
- [ ] Construction projects require specific goods (road needs gravel/asphalt, building needs lumber/brick)
- [ ] Goods are consumed during construction — if unavailable, construction stalls
- [ ] Residents consume food and goods to fulfill needs — if unavailable, needs go unfulfilled
- [ ] Import capacity is limited (throughput cap on the trade connection)
- [ ] Display goods availability and prices in the UI

---

## Milestone 13: Basic Market Pricing

Make prices respond to supply and demand so the economy feels dynamic.

- [ ] Track local supply and demand rate for each good
- [ ] Price = base cost × (demand / supply) with smoothing
- [ ] High demand / low supply → prices rise
- [ ] Low demand / high supply → prices drop
- [ ] Construction costs dynamically reflect current material prices
- [ ] Residents adjust spending based on prices (high food prices = less food fulfillment)
- [ ] Display price trends in the UI (even just current price per good is fine to start)

---

## Milestone 14: Economy-Driven Building Spawning

Replace the timer-based building spawning with real economic logic.

- [ ] Buildings spawn based on demand: residential demand driven by incoming population wanting housing, commercial demand driven by unmet consumer needs, industrial demand driven by import substitution opportunity
- [ ] Evaluate lot desirability: proximity to roads, nearby amenities/services, terrain quality
- [ ] Better lots develop first; marginal lots develop later or not at all
- [ ] Buildings don't spawn if economy can't support them (no demand, no materials, no workers)
- [ ] Existing buildings can be demolished and replaced with larger ones as demand and land value increase
- [ ] Empty zones stay empty if there's no economic reason to build — no artificial fill

---

## Milestone 15: Organic Growth Loop (Integration)

Connect all systems so the city grows as a living organism. This is the core gameplay loop.

- [ ] Verify the full cycle: player builds road → paints zone → lots generate → economy spawns buildings → residents arrive → economy grows → more demand → more building
- [ ] Player infrastructure decisions (where to build roads, what to zone) meaningfully shape where and how the city grows
- [ ] The city can grow without player input once the initial infrastructure is in place (organic expansion within zoned areas)
- [ ] The city can stagnate or decline if the economy falters (supply shortages, budget crisis, unmet needs)
- [ ] Construction costs and timelines feel connected to the economy (cheap when materials are abundant, expensive/slow when scarce)
- [ ] Playtest the loop: is it fun to lay a road, zone some land, and watch a neighborhood emerge? Iterate until it is.
- [ ] Basic save/load game state

---

## What Comes After

Once the core loop is solid, the next priorities (not scoped here) are:

- **Event system** — Paradox-style narrative events with player choices
- **Factions and politics** — city council, approval ratings, competing interests
- **Manually placed buildings** — hospitals, police stations, fire departments with the auto-road-connector behavior
- **Local production** — sawmills, factories, brick plants that reduce import dependency
- **Traffic simulation** — vehicles on roads, congestion, routing
- **Bridge construction** — spanning water and terrain gaps
- **Construction animation** — scaffolding, workers, material deliveries
- **Art assets** — replacing placeholder boxes with 1920s Art Deco models
- **Sound and music** — era-appropriate jazz, ambient city sounds
- **Historical events** — Crash of '29, Prohibition, labor movements
