# Completed Features

Features that have been fully implemented. Each entry summarizes what was built and notes any implementation details worth knowing.

---

## Wave 1: Foundation

### Terrain Generation & Rendering
Heightmap-based terrain using fBm Perlin noise (seed 42, 6 octaves). 256x256 grid over a 500x500 world (≈2 units/cell). Biome classification (Sand/Grass/Dirt/Rock) based on elevation, slope, and water level. Vertex-colored mesh with per-vertex normals. Water plane at configurable elevation (default 10.0). Height scale defaults to 30.0.

**Key code:** `src/terrain.rs` — `TerrainConfig`, `Heightmap` (with `sample_world()` for bilinear interpolation), `BiomeMap`, generation + mesh systems.

### Camera Controls
Orbit camera with WASD ground-plane movement, scroll zoom along look direction, right-click drag rotation. Speed scales with camera height. Uses `CityCamera` marker component.

**Key code:** `src/camera.rs`

### Road Network & Mesh Generation
Freeform spline-based roads. Click to place control points on terrain (R to toggle tool, Enter to confirm, Escape to cancel). Catmull-Rom spline interpolation. Snap-to-existing nodes (3.0 radius). Automatic segment splitting at intersections. Road mesh generated from spline cross-sections projected onto terrain with 0.15 Y offset. Intersection disc polygons at junctions (12-sided). Vertex-colored by road type (Dirt/Gravel/Paved). Default width 2.0 units. Minimum segment length 3.0 units.

**Key code:** `src/road.rs` — `RoadNetwork` graph (nodes + segments), `RoadPlacementState`, `ActiveTool`, mesh generation, debug gizmos (white nodes, orange segments, yellow preview).

### UI & Game State
egui-based HUD with date (starting Jan 1920), speed controls, city funds, population count. Left toolbar (Select, Road, Zone stub, Building stub). Game speed: Pause/Normal/Fast/VeryFast (Space toggle, 1/2/3 keys). 10 real seconds = 1 game month at 1x. Cursor world position via per-frame terrain raycast. Info panel showing position, elevation, resource info, nearby road nodes. Timed notification system.

**Key code:** `src/ui.rs` — `GameTime`, `GameSpeed`, `CityBudget`, `CursorWorldPosition`, `Notifications`.

**Key detail:** UI systems run on `EguiPrimaryContextPass` schedule. Input consumption checked via `bevy_egui::input::EguiWantsInput`.

---

## Wave 2: Core Gameplay Loop (In Progress)

### Map Resources
Procedural resource layer overlaid on terrain at heightmap resolution (256x256). Five resource types generated from terrain features:

| Resource | Terrain Conditions | Noise Seed | Noise Threshold |
|---|---|---|---|
| Coal | Dirt/Rock biome, elevation > 40% | 200 | 0.5 (rare, clustered) |
| Clay | Near water, low elevation (water+0 to water+4) | 300 | 0.3 |
| Stone | Rock biome | 400 | 0.3 |
| Fertile Land | Very flat (>0.96), near water, low elevation | 500 | -0.2 |
| Timber | Grass/Dirt biome, moderate slope, above water+3 | 100 | 0.2 (most common) |

Priority order prevents overlap: Coal > Clay > Stone > Fertile Land > Timber. Each cell stores resource type + richness (0.0-1.0). Resources are visually represented as color tints blended into terrain vertex colors at 70% richness weight. Info panel shows resource type and richness on hover.

**Key code:** `src/resources.rs` — `ResourceType`, `ResourceMap` (with `sample_world()`), `generate_resource_map()`.

### Lot Subdivision (Spatial Claims)
Buildings claim space via `Lot` entities (center, rotation, half_extents, building reference). OBB-based overlap detection prevents buildings from overlapping each other, roads (with clearance), and water. Terrain steepness is checked (max 3.0 unit height delta across lot corners). Lots are created automatically when buildings spawn and are tied to their building entity. Debug wireframe visualization available via F3.

**Key code:** `src/building.rs` — `Lot` component, `obb_overlap()`, `validate_placement()`, `lot_corners()`, `draw_lot_debug()`.

**Not yet implemented:** Demolition/lot freeing (no demolition system yet). Spatial index deferred — brute-force iteration is fine at current building counts.

### Building Spawning & Economy-Driven Growth
Two building types: Logging Camp (producer, extracts timber, requires 5 workers) and Worker Cottage (residential, provides 2 workers). Buildings spawn organically along roads based on economic viability:

1. **Producer viability:** Walks sampled points along road splines checking for matching resources (richness > 0.2) with no existing extractor within 60 units.
2. **Residential viability:** Spawns when workers_needed > workers_provided across all buildings.
3. **Candidate finding:** Samples positions along both sides of every road segment with setback, validates placement (lot overlap, road clearance, water, steepness).
4. **Scoring:** Producers scored by resource richness (0-8) with penalty near residential. Residential scored by proximity to producers (0-6) and clustering bonus (0-2). Terrain flatness (0-2) for both.
5. **Spawning:** One building per tick (2-second interval, scaled by game speed). Best-scored candidate wins. Notification on spawn.

All buildings are currently gray cubes (4x3x4 units). Buildings orient to face the road.

**Key code:** `src/economy.rs` — `BuildingDef`, `BUILDING_DEFS`, `evaluate_and_spawn()`, `is_producer_viable()`. `src/building.rs` — `Building` component, `find_candidates()`, `score_candidate()`, `spawn_building()`, `SpawnTimer`.

**Economy debug panel (F3):** Shows worker math, producer/residential viability with reasons, candidate counts, best score, and last spawn location. `EconomyDebug` resource populated each tick.

**Deferred to P1/P2:** Varied building types and meshes, farms/farmland, DemandPressure resource (replaced by simpler viability checks), commercial buildings. See p1-starting-industries, p1-basic-economy, p2-economic-pressure.

### Debug Overlay System
Single F3 toggle (`DebugVisible` resource) controls all debug visualizations: economy debug panel (egui bottom bar), road network gizmos (white node spheres, orange segment lines), resource cell gizmos (yellow rects for cells with richness > 0.2), and lot boundary wireframes (white outlines). Road placement preview (yellow curve while actively placing) remains visible regardless of debug toggle.

**Key code:** `src/ui.rs` — `DebugVisible`, F3 in `speed_controls()`, economy panel in `draw_ui()`. `src/road.rs` — `draw_road_debug()`. `src/resources.rs` — `draw_resource_debug()`. `src/building.rs` — `draw_lot_debug()`.
